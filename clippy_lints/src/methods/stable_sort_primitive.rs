use clippy_utils::diagnostics::{applicability_for_ctxt, span_lint_and_then};
use clippy_utils::is_slice_of_primitives;
use clippy_utils::source::SpanExt;
use rustc_hir::Expr;
use rustc_lint::LateContext;

use super::STABLE_SORT_PRIMITIVE;

pub(super) fn check<'tcx>(cx: &LateContext<'tcx>, e: &'tcx Expr<'_>, recv: &'tcx Expr<'_>) {
    if let Some(method_id) = cx.typeck_results().type_dependent_def_id(e.hir_id)
        && let Some(impl_id) = cx.tcx.impl_of_assoc(method_id)
        && cx.tcx.type_of(impl_id).instantiate_identity().is_slice()
        && let Some(slice_type) = is_slice_of_primitives(cx, recv)
        && let ctxt = e.span.ctxt()
        && let Some(recv_snip) = recv.span.get_source_text_at_ctxt(cx, ctxt)
    {
        span_lint_and_then(
            cx,
            STABLE_SORT_PRIMITIVE,
            e.span,
            format!("used `sort` on primitive type `{slice_type}`"),
            |diag| {
                diag.span_suggestion(
                    e.span,
                    "try",
                    format!("{recv_snip}.sort_unstable()"),
                    applicability_for_ctxt(ctxt),
                );
                diag.note(
                    "an unstable sort typically performs faster without any observable difference for this data type",
                );
            },
        );
    }
}
