use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::source::v2::SpanExt as _;
use clippy_utils::sugg::applicability_for_ctxt;
use rustc_ast::ast::{Expr, ExprKind, MethodCall};
use rustc_lexer::is_whitespace;
use rustc_lint::{EarlyContext, EarlyLintPass, declare_lint_pass};
use rustc_span::RelativeBytePos;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for unnecessary double parentheses.
    ///
    /// ### Why is this bad?
    /// This makes code harder to read and might indicate a
    /// mistake.
    ///
    /// ### Example
    /// ```no_run
    /// fn simple_double_parens() -> i32 {
    ///     ((0))
    /// }
    ///
    /// # fn foo(bar: usize) {}
    /// foo((0));
    /// ```
    ///
    /// Use instead:
    /// ```no_run
    /// fn simple_no_parens() -> i32 {
    ///     (0)
    /// }
    ///
    /// # fn foo(bar: usize) {}
    /// foo(0);
    /// ```
    #[clippy::version = "pre 1.29.0"]
    pub DOUBLE_PARENS,
    complexity,
    "Warn on unnecessary double parentheses"
}

declare_lint_pass!(DoubleParens => [DOUBLE_PARENS]);

impl EarlyLintPass for DoubleParens {
    fn check_expr(&mut self, cx: &EarlyContext<'_>, expr: &Expr) {
        let (inner, report_inner) = match &expr.kind {
            ExprKind::Paren(inner) if let ExprKind::Paren(_) | ExprKind::Tup(_) = inner.kind => (inner, false),
            ExprKind::Call(_, args) | ExprKind::MethodCall(MethodCall { args, .. })
                if let [inner] = &**args
                    && let ExprKind::Paren(_) = &inner.kind =>
            {
                (inner, true)
            },
            _ => return,
        };

        let outer_sp = expr.span.data();
        let inner_sp = inner.span.data();
        if outer_sp.ctxt == inner_sp.ctxt
            && let Some(spans) = outer_sp.map_split_range(cx, |scx, outer_range| {
                let outer_src = scx.get_text(outer_range.clone())?;
                let inner_start = inner_sp.lo.0.wrapping_sub(outer_sp.lo.0);
                let inner_end = inner_sp.hi.0.wrapping_sub(outer_sp.lo.0);

                // Check for the inner and outer parenthesis
                let pre_len = outer_src
                    .get(..inner_start as usize)?
                    .trim_end_matches(is_whitespace)
                    .strip_suffix('(')?
                    .len() as u32;
                if outer_src.get(inner_end as usize..)?.trim_start_matches(is_whitespace) != ")" {
                    return None;
                }
                let inner_src = outer_src
                    .get(inner_start as usize..inner_end as usize)?
                    .strip_prefix('(')?
                    .strip_suffix(')')?;

                // Don't lint pattern expansions. e.g. `(($($var)*))` may expand to `((foo))` or
                // `((foo, bar))` depending on what `$var` expands to.
                inner_src
                    .trim_start_matches(is_whitespace)
                    .strip_prefix('$')
                    .is_none_or(|text| !text.trim_start_matches(is_whitespace).starts_with('('))
                    .then(|| {
                        let (start, end) = if report_inner {
                            (inner_start, outer_range.start.0 + inner_end)
                        } else {
                            (pre_len, outer_range.end.0)
                        };
                        let start = outer_range.start.0 + start;
                        [
                            RelativeBytePos(start)..RelativeBytePos(start + 1),
                            RelativeBytePos(end - 1)..RelativeBytePos(end),
                        ]
                    })
            })
        {
            span_lint_and_then(cx, DOUBLE_PARENS, Vec::from(spans), "unnecessary parentheses", |diag| {
                diag.multipart_suggestion(
                    "remove them",
                    Vec::from(spans.map(|x| (x, String::new()))),
                    applicability_for_ctxt(outer_sp.ctxt),
                );
            });
        }
    }
}
