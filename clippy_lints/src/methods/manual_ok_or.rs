use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::res::{MaybeResPath, PathRes, TyCtxtDefExt};
use clippy_utils::source::{SpanRangeExt, indent_of, reindent_multiline};
use rustc_errors::Applicability;
use rustc_hir::LangItem::{ResultErr, ResultOk};
use rustc_hir::{Expr, ExprKind, PatKind};
use rustc_lint::LateContext;
use rustc_span::symbol::sym;

use super::MANUAL_OK_OR;

pub(super) fn check<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
    recv: &'tcx Expr<'_>,
    or_expr: &'tcx Expr<'_>,
    map_expr: &'tcx Expr<'_>,
) {
    if let Some(method_id) = cx.typeck_results().type_dependent_def_id(expr.hir_id)
        && let Some(impl_id) = cx.tcx.impl_of_assoc(method_id)
        && cx.is_diag_item(cx.tcx.type_of(impl_id).instantiate_identity(), sym::Option)
        && let ExprKind::Call(err_path, [err_arg]) = or_expr.kind
        && cx.is_path_lang_ctor(err_path, ResultErr)
        && is_ok_wrapping(cx, map_expr)
        && let Some(recv_snippet) = recv.span.get_source_text(cx)
        && let Some(err_arg_snippet) = err_arg.span.get_source_text(cx)
        && let Some(indent) = indent_of(cx, expr.span)
    {
        let reindented_err_arg_snippet = reindent_multiline(err_arg_snippet.as_str(), true, Some(indent + 4));
        span_lint_and_sugg(
            cx,
            MANUAL_OK_OR,
            expr.span,
            "this pattern reimplements `Option::ok_or`",
            "replace with",
            format!("{recv_snippet}.ok_or({reindented_err_arg_snippet})"),
            Applicability::MachineApplicable,
        );
    }
}

fn is_ok_wrapping(cx: &LateContext<'_>, map_expr: &Expr<'_>) -> bool {
    match map_expr.kind {
        ExprKind::Path(ref qpath) if cx.is_path_lang_ctor((qpath, map_expr.hir_id), ResultOk) => true,
        ExprKind::Closure(closure) => {
            let body = cx.tcx.hir_body(closure.body);
            if let PatKind::Binding(_, param_id, ..) = body.params[0].pat.kind
                && let ExprKind::Call(callee, [ok_arg]) = body.value.kind
                && cx.is_path_lang_ctor(callee, ResultOk)
            {
                ok_arg.is_path_local(param_id)
            } else {
                false
            }
        },
        _ => false,
    }
}
