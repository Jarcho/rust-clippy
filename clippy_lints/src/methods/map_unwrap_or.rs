use clippy_utils::diagnostics::{span_lint, span_lint_and_sugg};
use clippy_utils::msrvs::{self, Msrv};
use clippy_utils::res::TyCtxtDefExt;
use clippy_utils::source::snippet;
use clippy_utils::usage::mutated_variables;
use rustc_errors::Applicability;
use rustc_hir as hir;
use rustc_lint::LateContext;
use rustc_span::symbol::sym;

use super::MAP_UNWRAP_OR;

/// lint use of `map().unwrap_or_else()` for `Option`s and `Result`s
///
/// Returns true if the lint was emitted
pub(super) fn check<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx hir::Expr<'_>,
    recv: &'tcx hir::Expr<'_>,
    map_arg: &'tcx hir::Expr<'_>,
    unwrap_arg: &'tcx hir::Expr<'_>,
    msrv: Msrv,
) -> bool {
    let msg = match cx.opt_diag_name(cx.typeck_results().expr_ty(recv)) {
        Some(sym::Option) => "called `map(<f>).unwrap_or_else(<g>)` on an `Option` value",
        Some(sym::Result) if msrv.meets(cx, msrvs::RESULT_MAP_OR_ELSE) => {
            "called `map(<f>).unwrap_or_else(<g>)` on a `Result` value"
        },
        _ => return false,
    };

    // Don't make a suggestion that may fail to compile due to mutably borrowing
    // the same variable twice.
    let map_mutated_vars = mutated_variables(recv, cx);
    let unwrap_mutated_vars = mutated_variables(unwrap_arg, cx);
    if let (Some(map_mutated_vars), Some(unwrap_mutated_vars)) = (map_mutated_vars, unwrap_mutated_vars) {
        if map_mutated_vars.intersection(&unwrap_mutated_vars).next().is_some() {
            return false;
        }
    } else {
        return false;
    }

    // get snippets for args to map() and unwrap_or_else()
    let map_snippet = snippet(cx, map_arg.span, "..");
    let unwrap_snippet = snippet(cx, unwrap_arg.span, "..");
    if map_arg.span.eq_ctxt(unwrap_arg.span) {
        // lint, with note if neither arg is > 1 line
        let multiline = map_snippet.lines().count() > 1 || unwrap_snippet.lines().count() > 1;
        if multiline {
            span_lint(cx, MAP_UNWRAP_OR, expr.span, msg);
        } else {
            let var_snippet = snippet(cx, recv.span, "..");
            span_lint_and_sugg(
                cx,
                MAP_UNWRAP_OR,
                expr.span,
                msg,
                "try",
                format!("{var_snippet}.map_or_else({unwrap_snippet}, {map_snippet})"),
                Applicability::MachineApplicable,
            );
        }
        true
    } else {
        false
    }
}
