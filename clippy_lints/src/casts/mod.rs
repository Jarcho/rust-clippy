pub mod as_pointer_underscore;
pub mod as_ptr_cast_mut;
pub mod as_underscore;
pub mod borrow_as_ptr;
pub mod cast_abs_to_unsigned;
pub mod cast_enum_constructor;
pub mod cast_lossless;
pub mod cast_nan_to_int;
pub mod cast_possible_truncation;
pub mod cast_possible_wrap;
pub mod cast_precision_loss;
pub mod cast_ptr_alignment;
pub mod cast_sign_loss;
pub mod cast_slice_different_sizes;
pub mod cast_slice_from_raw_parts;
pub mod char_lit_as_u8;
pub mod confusing_method_to_numeric_cast;
pub mod fn_to_numeric_cast;
pub mod fn_to_numeric_cast_any;
pub mod fn_to_numeric_cast_with_truncation;
pub mod manual_dangling_ptr;
pub mod needless_type_cast;
pub mod ptr_as_ptr;
pub mod ptr_cast_constness;
pub mod ref_as_ptr;
pub mod unnecessary_cast;
pub mod zero_ptr;

mod utils;

use clippy_config::Conf;
use clippy_utils::is_hir_ty_cfg_dependant;
use clippy_utils::msrvs::{self, Msrv};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext as _, impl_lint_pass};

impl_lint_pass!(Casts => [
    as_pointer_underscore::AS_POINTER_UNDERSCORE,
    as_ptr_cast_mut::AS_PTR_CAST_MUT,
    as_underscore::AS_UNDERSCORE,
    borrow_as_ptr::BORROW_AS_PTR,
    cast_abs_to_unsigned::CAST_ABS_TO_UNSIGNED,
    cast_enum_constructor::CAST_ENUM_CONSTRUCTOR,
    cast_lossless::CAST_LOSSLESS,
    cast_nan_to_int::CAST_NAN_TO_INT,
    cast_possible_truncation::CAST_ENUM_TRUNCATION,
    cast_possible_truncation::CAST_POSSIBLE_TRUNCATION,
    cast_possible_wrap::CAST_POSSIBLE_WRAP,
    cast_precision_loss::CAST_PRECISION_LOSS,
    cast_ptr_alignment::CAST_PTR_ALIGNMENT,
    cast_sign_loss::CAST_SIGN_LOSS,
    cast_slice_different_sizes::CAST_SLICE_DIFFERENT_SIZES,
    cast_slice_from_raw_parts::CAST_SLICE_FROM_RAW_PARTS,
    char_lit_as_u8::CHAR_LIT_AS_U8,
    confusing_method_to_numeric_cast::CONFUSING_METHOD_TO_NUMERIC_CAST,
    fn_to_numeric_cast::FN_TO_NUMERIC_CAST,
    fn_to_numeric_cast_any::FN_TO_NUMERIC_CAST_ANY,
    fn_to_numeric_cast_with_truncation::FN_TO_NUMERIC_CAST_WITH_TRUNCATION,
    manual_dangling_ptr::MANUAL_DANGLING_PTR,
    needless_type_cast::NEEDLESS_TYPE_CAST,
    ptr_as_ptr::PTR_AS_PTR,
    ptr_cast_constness::PTR_CAST_CONSTNESS,
    ref_as_ptr::REF_AS_PTR,
    unnecessary_cast::UNNECESSARY_CAST,
    zero_ptr::ZERO_PTR,
]);

pub struct Casts {
    msrv: Msrv,
}

impl Casts {
    pub fn new(conf: &'static Conf) -> Self {
        Self { msrv: conf.msrv.into() }
    }
}

impl<'tcx> LateLintPass<'tcx> for Casts {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if let ExprKind::Cast(cast_from_expr, cast_to_hir) = expr.kind {
            if expr.span.in_external_macro(cx.sess().source_map()) {
                return;
            }
            if is_hir_ty_cfg_dependant(cx, cast_to_hir) {
                return;
            }
            let (cast_from, cast_to) = (
                cx.typeck_results().expr_ty(cast_from_expr),
                cx.typeck_results().expr_ty(expr),
            );

            if !expr.span.from_expansion() && unnecessary_cast::check(cx, expr, cast_from_expr, cast_from, cast_to) {
                return;
            }
            char_lit_as_u8::check(cx, expr, cast_from_expr, cast_to);
            cast_slice_from_raw_parts::check(cx, expr, cast_from_expr, cast_to, self.msrv);
            cast_ptr_alignment::check(cx, expr, cast_from, cast_to);
            ptr_cast_constness::check(cx, expr, cast_from_expr, cast_from, cast_to, self.msrv);
            ptr_as_ptr::check(cx, expr, cast_from_expr, cast_from, cast_to_hir, cast_to, self.msrv);
            as_ptr_cast_mut::check(cx, expr, cast_from_expr, cast_to);
            confusing_method_to_numeric_cast::check(cx, expr, cast_from_expr, cast_from, cast_to);
            zero_ptr::check(cx, expr, cast_from_expr, cast_to_hir, self.msrv);

            if self.msrv.meets(cx, msrvs::MANUAL_DANGLING_PTR) {
                manual_dangling_ptr::check(cx, expr, cast_from_expr, cast_to_hir);
            }

            if cast_to.is_numeric() {
                cast_possible_truncation::check(cx, expr, cast_from_expr, cast_from, cast_to, cast_to_hir.span);
                if cast_from.is_numeric() {
                    cast_possible_wrap::check(cx, expr, cast_from_expr, cast_from, cast_to, self.msrv);
                    cast_precision_loss::check(cx, expr, cast_from, cast_to);
                    cast_sign_loss::check(cx, expr, cast_from_expr, cast_from, cast_to, self.msrv);
                    cast_abs_to_unsigned::check(cx, expr, cast_from_expr, cast_from, cast_to, self.msrv);
                    cast_nan_to_int::check(cx, expr, cast_from_expr, cast_from, cast_to);
                }
                cast_lossless::check(cx, expr, cast_from_expr, cast_from, cast_to, cast_to_hir, self.msrv);
                cast_enum_constructor::check(cx, expr, cast_from_expr, cast_from);
                fn_to_numeric_cast_any::check(cx, expr, cast_from_expr, cast_from, cast_to);
                fn_to_numeric_cast::check(cx, expr, cast_from_expr, cast_from, cast_to);
                fn_to_numeric_cast_with_truncation::check(cx, expr, cast_from_expr, cast_from, cast_to);
            }

            as_underscore::check(cx, expr, cast_to_hir);
            as_pointer_underscore::check(cx, cast_to, cast_to_hir);

            let was_borrow_as_ptr_emitted = self.msrv.meets(cx, msrvs::BORROW_AS_PTR)
                && borrow_as_ptr::check(cx, expr, cast_from_expr, cast_to_hir, self.msrv);
            if !was_borrow_as_ptr_emitted && self.msrv.meets(cx, msrvs::PTR_FROM_REF) {
                ref_as_ptr::check(cx, expr, cast_from_expr, cast_to_hir);
            }
        }

        if self.msrv.meets(cx, msrvs::RAW_REF_OP) {
            borrow_as_ptr::check_implicit_cast(cx, expr);
        }
        if self.msrv.meets(cx, msrvs::PTR_SLICE_RAW_PARTS) {
            cast_slice_from_raw_parts::check_implicit_cast(cx, expr);
        }
        cast_ptr_alignment::check_cast_method(cx, expr);
        cast_slice_different_sizes::check(cx, expr, self.msrv);
        ptr_cast_constness::check_null_ptr_cast_method(cx, expr);
    }

    fn check_body(&mut self, cx: &LateContext<'tcx>, body: &rustc_hir::Body<'tcx>) {
        needless_type_cast::check(cx, body);
    }
}
