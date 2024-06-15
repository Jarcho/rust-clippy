use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::source::{IntoSpan, SpanRangeExt};
use core::ops::Range;
use rustc_ast::{Block, Expr, ExprKind, Label, StmtKind};
use rustc_errors::Applicability;
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_middle::lint::in_external_macro;
use rustc_session::declare_lint_pass;
use rustc_span::{BytePos, Span, SyntaxContext};

declare_clippy_lint! {
    /// ### What it does
    /// The lint checks for `if`-statements appearing in loops
    /// that contain a `continue` statement in either their main blocks or their
    /// `else`-blocks, when omitting the `else`-block possibly with some
    /// rearrangement of code can make the code easier to understand.
    ///
    /// ### Why is this bad?
    /// Having explicit `else` blocks for `if` statements
    /// containing `continue` in their THEN branch adds unnecessary branching and
    /// nesting to the code. Having an else block containing just `continue` can
    /// also be better written by grouping the statements following the whole `if`
    /// statement within the THEN block and omitting the else block completely.
    ///
    /// ### Example
    /// ```no_run
    /// # fn condition() -> bool { false }
    /// # fn update_condition() {}
    /// # let x = false;
    /// while condition() {
    ///     update_condition();
    ///     if x {
    ///         // ...
    ///     } else {
    ///         continue;
    ///     }
    ///     println!("Hello, world");
    /// }
    /// ```
    ///
    /// Could be rewritten as
    ///
    /// ```no_run
    /// # fn condition() -> bool { false }
    /// # fn update_condition() {}
    /// # let x = false;
    /// while condition() {
    ///     update_condition();
    ///     if x {
    ///         // ...
    ///         println!("Hello, world");
    ///     }
    /// }
    /// ```
    ///
    /// As another example, the following code
    ///
    /// ```no_run
    /// # fn waiting() -> bool { false }
    /// loop {
    ///     if waiting() {
    ///         continue;
    ///     } else {
    ///         // Do something useful
    ///     }
    ///     # break;
    /// }
    /// ```
    /// Could be rewritten as
    ///
    /// ```no_run
    /// # fn waiting() -> bool { false }
    /// loop {
    ///     if waiting() {
    ///         continue;
    ///     }
    ///     // Do something useful
    ///     # break;
    /// }
    /// ```
    #[clippy::version = "pre 1.29.0"]
    pub NEEDLESS_CONTINUE,
    pedantic,
    "`continue` statements that can be replaced by a rearrangement of code"
}

declare_lint_pass!(NeedlessContinue => [NEEDLESS_CONTINUE]);

impl EarlyLintPass for NeedlessContinue {
    fn check_expr(&mut self, cx: &EarlyContext<'_>, expr: &Expr) {
        // Workaround rustfmt#6202
        #[expect(unused_parens)]
        if let (ExprKind::Loop(body, label, ..)
        | ExprKind::While(_, body, label)
        | ExprKind::ForLoop { body, label, .. }) = &expr.kind
            && !in_external_macro(cx.sess(), expr.span)
        {
            check_final_block_stmt(cx, body, label, expr.span.ctxt());
        }
    }
}

fn check_final_block_stmt(cx: &EarlyContext<'_>, b: &Block, label: &Option<Label>, ctxt: SyntaxContext) {
    if let [.., stmt] = &*b.stmts
        && let StmtKind::Semi(e) | StmtKind::Expr(e) = &stmt.kind
        && b.span.ctxt() == ctxt
        && stmt.span.ctxt() == ctxt
    {
        check_final_expr(cx, e, label, ctxt, false);
    }
}

fn check_final_expr(cx: &EarlyContext<'_>, e: &Expr, label: &Option<Label>, ctxt: SyntaxContext, in_match: bool) {
    if e.span.ctxt() != ctxt {
        return;
    }
    match &e.kind {
        ExprKind::Continue(dst)
            if label_targets_label(dst, label)
                && let Some(range) = expand_continue_span(cx, e.span) =>
        {
            span_lint_and_then(
                cx,
                NEEDLESS_CONTINUE,
                e.span,
                "this `continue` expression is redundant",
                |diag| {
                    diag.span_suggestion(
                        range.into_span(),
                        "remove this",
                        if in_match { " {}" } else { "" },
                        if ctxt.is_root() {
                            Applicability::MachineApplicable
                        } else {
                            Applicability::MaybeIncorrect
                        },
                    );
                },
            );
        },
        ExprKind::If(_, then, else_) => {
            check_final_block_stmt(cx, then, label, ctxt);
            if let Some(else_) = else_ {
                check_final_expr(cx, else_, label, ctxt, false);
            }
        },
        ExprKind::Match(_, arms, _) => {
            for arm in arms {
                if let Some(body) = &arm.body
                    && arm.span.ctxt() == ctxt
                {
                    check_final_expr(cx, body, label, ctxt, true)
                }
            }
        },
        ExprKind::Block(b, _) => check_final_block_stmt(cx, b, label, ctxt),
        _ => {},
    }
}

fn expand_continue_span(cx: &EarlyContext<'_>, sp: Span) -> Option<Range<BytePos>> {
    sp.expand_source_range_by(cx, |src, range| {
        // Make sure the span actually points to correct expression.
        if !src.get(range.clone())?.starts_with("continue") {
            return None;
        }
        let prefix = src.get(..range.start)?;
        let ex_start = prefix.len() - prefix.trim_end().len();
        let suffix = src.get(range.end..)?;
        let trim_suffix = suffix.trim_start();
        Some((
            ex_start,
            if trim_suffix.starts_with(';') {
                suffix.len() - trim_suffix.len() + 1
            } else {
                0
            },
        ))
    })
}

fn label_targets_label(dst: &Option<Label>, label: &Option<Label>) -> bool {
    match (dst, label) {
        // `loop { continue; }` or `'a loop { continue; }`
        (None, _) => true,
        // `loop { continue 'a; }`
        (_, None) => false,
        // `'a loop { continue 'a; }` or `'a loop { continue 'b; }`
        (Some(x), Some(y)) => x.ident == y.ident,
    }
}
