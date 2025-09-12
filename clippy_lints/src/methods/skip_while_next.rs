use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::res::PathRes;
use rustc_hir as hir;
use rustc_lint::LateContext;
use rustc_span::sym;

use super::SKIP_WHILE_NEXT;

/// lint use of `skip_while().next()` for `Iterators`
pub(super) fn check<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx hir::Expr<'_>) {
    // lint if caller of `.skip_while().next()` is an Iterator
    if cx.is_type_dependent_assoc_of_diag_item(expr, sym::Iterator) {
        span_lint_and_help(
            cx,
            SKIP_WHILE_NEXT,
            expr.span,
            "called `skip_while(<p>).next()` on an `Iterator`",
            None,
            "this is more succinctly expressed by calling `.find(!<p>)` instead",
        );
    }
}
