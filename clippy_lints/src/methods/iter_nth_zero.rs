use clippy_utils::consts::{ConstEvalCtxt, Constant};
use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::res::{PathRes, TyCtxtDefExt};
use clippy_utils::source::snippet_with_applicability;
use rustc_errors::Applicability;
use rustc_hir as hir;
use rustc_lint::LateContext;
use rustc_span::sym;

use super::ITER_NTH_ZERO;

pub(super) fn check(cx: &LateContext<'_>, expr: &hir::Expr<'_>, recv: &hir::Expr<'_>, arg: &hir::Expr<'_>) {
    if cx.is_type_dependent_assoc_of_diag_item(expr, sym::Iterator)
        && let Some(Constant::Int(0)) = ConstEvalCtxt::new(cx).eval(arg)
    {
        if let hir::OwnerNode::ImplItem(item) = cx.tcx.hir_owner_node(expr.hir_id.owner)
            && item.ident.name == sym::next
            && let Some(parent) = cx.tcx.opt_parent(expr.hir_id.owner.to_def_id())
            && cx.is_diag_item(cx.tcx.trait_id_of_impl(parent), sym::Iterator)
        {
            return;
        }

        let mut app = Applicability::MachineApplicable;
        span_lint_and_sugg(
            cx,
            ITER_NTH_ZERO,
            expr.span,
            "called `.nth(0)` on a `std::iter::Iterator`, when `.next()` is equivalent",
            "try calling `.next()` instead of `.nth(0)`",
            format!("{}.next()", snippet_with_applicability(cx, recv.span, "..", &mut app)),
            app,
        );
    }
}
