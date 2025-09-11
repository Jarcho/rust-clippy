use super::ERR_EXPECT;
use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::msrvs::{self, Msrv};
use clippy_utils::res::TyCtxtDefExt;
use clippy_utils::ty::has_debug_impl;
use rustc_errors::Applicability;
use rustc_lint::LateContext;
use rustc_middle::ty;
use rustc_span::{Span, sym};

pub(super) fn check(
    cx: &LateContext<'_>,
    _expr: &rustc_hir::Expr<'_>,
    recv: &rustc_hir::Expr<'_>,
    expect_span: Span,
    err_span: Span,
    msrv: Msrv,
) {
    if let ty::Adt(adt, args) = *cx.typeck_results().expr_ty(recv).kind()
        && cx.is_diag_item(adt, sym::Result)
        && let Some(ok_ty) = args.types().next()
        && has_debug_impl(cx, ok_ty)
        && msrv.meets(cx, msrvs::EXPECT_ERR)
    {
        span_lint_and_sugg(
            cx,
            ERR_EXPECT,
            err_span.to(expect_span),
            "called `.err().expect()` on a `Result` value",
            "try",
            "expect_err".to_string(),
            Applicability::MachineApplicable,
        );
    }
}
