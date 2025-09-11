use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::res::{PathRes, TyCtxtDefExt};
use clippy_utils::source::snippet_with_applicability;
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::LateContext;
use rustc_span::{Span, sym};

use super::OR_THEN_UNWRAP;

pub(super) fn check<'tcx>(
    cx: &LateContext<'tcx>,
    unwrap_expr: &Expr<'_>,
    recv: &'tcx Expr<'tcx>,
    or_arg: &'tcx Expr<'_>,
    or_span: Span,
) {
    if let ExprKind::Call(or_callee, [or_arg]) = or_arg.kind
        && let (msg, Some(ctor_did)) = match cx.opt_diag_name(cx.typeck_results().expr_ty(recv).ty_adt_def()) {
            Some(sym::Option) => (
                "found `.or(Some(…)).unwrap()`",
                cx.tcx.lang_items().option_some_variant(),
            ),
            Some(sym::Result) => ("found `.or(Ok(…)).unwrap()`", cx.tcx.lang_items().result_ok_variant()),
            _ => return,
        }
        && cx.path_ctor_parent_id(or_callee) == Some(ctor_did)
    {
        let mut applicability = Applicability::MachineApplicable;
        let suggestion = format!(
            "unwrap_or({})",
            snippet_with_applicability(cx, or_arg.span.source_callsite(), "..", &mut applicability)
        );

        span_lint_and_sugg(
            cx,
            OR_THEN_UNWRAP,
            unwrap_expr.span.with_lo(or_span.lo()),
            msg,
            "try",
            suggestion,
            applicability,
        );
    }
}
