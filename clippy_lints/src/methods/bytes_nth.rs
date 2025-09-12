use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::res::TyCtxtDefExt;
use clippy_utils::source::snippet_with_applicability;
use clippy_utils::sym;
use rustc_errors::Applicability;
use rustc_hir::{Expr, LangItem};
use rustc_lint::LateContext;
use rustc_middle::ty;

use crate::methods::method_call;

use super::BYTES_NTH;

pub(super) fn check<'tcx>(cx: &LateContext<'tcx>, expr: &Expr<'_>, recv: &'tcx Expr<'tcx>, n_arg: &'tcx Expr<'tcx>) {
    let caller_type = match *cx.typeck_results().expr_ty(recv).peel_refs().kind() {
        ty::Str => "str",
        ty::Adt(adt, _) if cx.is_lang_item(adt, LangItem::String) => "String",
        _ => return,
    };

    let mut applicability = Applicability::MachineApplicable;
    let receiver = snippet_with_applicability(cx, recv.span, "..", &mut applicability);
    let n = snippet_with_applicability(cx, n_arg.span, "..", &mut applicability);

    if let Some(parent) = clippy_utils::get_parent_expr(cx, expr)
        && let Some((name, _, _, _, _)) = method_call(parent)
        && name == sym::unwrap
    {
        span_lint_and_sugg(
            cx,
            BYTES_NTH,
            parent.span,
            format!("called `.bytes().nth().unwrap()` on a `{caller_type}`"),
            "try",
            format!("{receiver}.as_bytes()[{n}]",),
            applicability,
        );
    } else {
        span_lint_and_sugg(
            cx,
            BYTES_NTH,
            expr.span,
            format!("called `.bytes().nth()` on a `{caller_type}`"),
            "try",
            format!("{receiver}.as_bytes().get({n}).copied()"),
            applicability,
        );
    }
}
