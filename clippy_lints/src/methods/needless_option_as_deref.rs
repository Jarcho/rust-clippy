use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::res::{MaybeResPath, TyCtxtDefExt};
use clippy_utils::source::SpanRangeExt;
use clippy_utils::sym;
use clippy_utils::usage::local_used_after_expr;
use rustc_errors::Applicability;
use rustc_hir::Expr;
use rustc_lint::LateContext;
use rustc_span::Symbol;

use super::NEEDLESS_OPTION_AS_DEREF;

pub(super) fn check(cx: &LateContext<'_>, expr: &Expr<'_>, recv: &Expr<'_>, name: Symbol) {
    let typeck = cx.typeck_results();
    let outer_ty = typeck.expr_ty(expr);

    if cx.is_diag_item(outer_ty, sym::Option) && outer_ty == typeck.expr_ty(recv) {
        if name == sym::as_deref_mut && recv.is_syntactic_place_expr() {
            let Some(binding_id) = recv.path_local_id() else {
                return;
            };

            if local_used_after_expr(cx, binding_id, recv) {
                return;
            }
        }

        span_lint_and_sugg(
            cx,
            NEEDLESS_OPTION_AS_DEREF,
            expr.span,
            "derefed type is same as origin",
            "try",
            recv.span.get_source_text(cx).unwrap().to_owned(),
            Applicability::MachineApplicable,
        );
    }
}
