use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::res::{PathRes, TyCtxtDefExt};
use clippy_utils::source::snippet;
use rustc_errors::Applicability;
use rustc_hir as hir;
use rustc_hir::LangItem::{OptionNone, OptionSome};
use rustc_lint::LateContext;
use rustc_span::symbol::sym;

use super::{OPTION_MAP_OR_NONE, RESULT_MAP_OR_INTO_OPTION};

// The expression inside a closure may or may not have surrounding braces
// which causes problems when generating a suggestion.
fn reduce_unit_expression<'a>(expr: &'a hir::Expr<'_>) -> Option<(&'a hir::Expr<'a>, &'a [hir::Expr<'a>])> {
    match expr.kind {
        hir::ExprKind::Call(func, arg_char) => Some((func, arg_char)),
        hir::ExprKind::Block(block, _) => {
            match (block.stmts, block.expr) {
                (&[], Some(inner_expr)) => {
                    // If block only contains an expression,
                    // reduce `|x| { x + 1 }` to `|x| x + 1`
                    reduce_unit_expression(inner_expr)
                },
                _ => None,
            }
        },
        _ => None,
    }
}

/// lint use of `_.map_or(None, _)` for `Option`s and `Result`s
pub(super) fn check<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx hir::Expr<'_>,
    recv: &'tcx hir::Expr<'_>,
    def_arg: &'tcx hir::Expr<'_>,
    map_arg: &'tcx hir::Expr<'_>,
) {
    let is_option = match cx.opt_diag_name(cx.typeck_results().expr_ty(recv)) {
        Some(sym::Option) => true,
        Some(sym::Result) => false,
        _ => return,
    };

    if !cx.is_path_lang_ctor(def_arg, OptionNone) {
        // nothing to lint!
        return;
    }

    if is_option {
        let self_snippet = snippet(cx, recv.span, "..");
        if let hir::ExprKind::Closure(&hir::Closure { body, fn_decl_span, .. }) = map_arg.kind
            && let arg_snippet = snippet(cx, fn_decl_span, "..")
            && let body = cx.tcx.hir_body(body)
            && let Some((func, [arg_char])) = reduce_unit_expression(body.value)
            && cx.is_path_lang_ctor(func, OptionSome)
        {
            let func_snippet = snippet(cx, arg_char.span, "..");
            let msg = "called `map_or(None, ..)` on an `Option` value";
            return span_lint_and_sugg(
                cx,
                OPTION_MAP_OR_NONE,
                expr.span,
                msg,
                "consider using `map`",
                format!("{self_snippet}.map({arg_snippet} {func_snippet})"),
                Applicability::MachineApplicable,
            );
        }

        let func_snippet = snippet(cx, map_arg.span, "..");
        let msg = "called `map_or(None, ..)` on an `Option` value";
        span_lint_and_sugg(
            cx,
            OPTION_MAP_OR_NONE,
            expr.span,
            msg,
            "consider using `and_then`",
            format!("{self_snippet}.and_then({func_snippet})"),
            Applicability::MachineApplicable,
        );
    } else if cx.is_path_lang_ctor(map_arg, OptionSome) {
        let msg = "called `map_or(None, Some)` on a `Result` value";
        let self_snippet = snippet(cx, recv.span, "..");
        span_lint_and_sugg(
            cx,
            RESULT_MAP_OR_INTO_OPTION,
            expr.span,
            msg,
            "consider using `ok`",
            format!("{self_snippet}.ok()"),
            Applicability::MachineApplicable,
        );
    }
}
