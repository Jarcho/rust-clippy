use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::last_path_segment;
use rustc_ast::LitKind;
use rustc_errors::Applicability::MachineApplicable;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::{sym, symbol, Symbol};

declare_clippy_lint! {
    /// ### What it does
    ///
    /// Checks for usage of `""` to create a `String`, such as `"".to_string()`, `"".to_owned()`,
    /// `String::from("")` and others.
    ///
    /// ### Why is this bad?
    ///
    /// Different ways of creating an empty string makes your code less standardized, which can
    /// be confusing.
    ///
    /// ### Example
    /// ```no_run
    /// let a = "".to_string();
    /// let b: String = "".into();
    /// ```
    /// Use instead:
    /// ```no_run
    /// let a = String::new();
    /// let b = String::new();
    /// ```
    #[clippy::version = "1.65.0"]
    pub MANUAL_STRING_NEW,
    pedantic,
    "empty String is being created manually"
}
declare_lint_pass!(ManualStringNew => [MANUAL_STRING_NEW]);

impl LateLintPass<'_> for ManualStringNew {
    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &Expr<'_>) {
        let target_id = match expr.kind {
            ExprKind::Call(func, [arg])
                if is_empty_str(arg)
                    && let ExprKind::Path(qpath) = &func.kind
                    && is_linted_fn(last_path_segment(qpath).ident.name) =>
            {
                cx.qpath_res(qpath, func.hir_id).opt_def_id()
            },
            ExprKind::MethodCall(path_segment, recv, [], _)
                if is_empty_str(recv) && is_linted_fn(path_segment.ident.name) =>
            {
                cx.typeck_results().type_dependent_def_id(expr.hir_id)
            },
            _ => return,
        };

        if let Some(target_id) = target_id
            && !expr.span.from_expansion()
            && let Some(did) = cx.tcx.trait_of_item(target_id)
            && matches!(
                cx.tcx.get_diagnostic_name(did),
                Some(sym::ToOwned | sym::Into | sym::From | sym::ToString)
            )
            && let Some(adt) = cx.typeck_results().expr_ty(expr).ty_adt_def()
            && Some(adt.did()) == cx.tcx.lang_items().string()
        {
            span_lint_and_sugg(
                cx,
                MANUAL_STRING_NEW,
                expr.span,
                "empty String is being created manually",
                "consider using",
                "String::new()".into(),
                MachineApplicable,
            );
        }
    }
}

fn is_empty_str(e: &Expr<'_>) -> bool {
    if let ExprKind::Lit(lit) = e.kind
        && let LitKind::Str(value, _) = lit.node
    {
        value == symbol::kw::Empty
    } else {
        false
    }
}

fn is_linted_fn(s: Symbol) -> bool {
    match s {
        sym::from | sym::to_string => true,
        _ => matches!(s.as_str(), "into" | "to_owned"),
    }
}
