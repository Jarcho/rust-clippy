use clippy_utils::consts::{ConstEvalCtxt, Constant};
use clippy_utils::diagnostics::span_lint;
use clippy_utils::res::PathRes;
use rustc_hir as hir;
use rustc_lint::LateContext;
use rustc_span::sym;

use super::ITERATOR_STEP_BY_ZERO;

pub(super) fn check<'tcx>(cx: &LateContext<'tcx>, expr: &hir::Expr<'_>, arg: &'tcx hir::Expr<'_>) {
    if cx.is_type_dependent_assoc_of_diag_item(expr, sym::Iterator)
        && let Some(Constant::Int(0)) = ConstEvalCtxt::new(cx).eval(arg)
    {
        span_lint(
            cx,
            ITERATOR_STEP_BY_ZERO,
            expr.span,
            "`Iterator::step_by(0)` will panic at runtime",
        );
    }
}
