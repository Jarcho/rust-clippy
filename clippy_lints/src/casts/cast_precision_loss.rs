use super::utils;
use clippy_utils::diagnostics::span_lint;
use clippy_utils::ty::is_isize_or_usize;
use rustc_hir::Expr;
use rustc_lint::LateContext;
use rustc_middle::ty::{self, FloatTy, Ty};

declare_clippy_lint! {
    /// ### What it does
    /// Checks for casts from any numeric type to a float type where
    /// the receiving type cannot store all values from the original type without
    /// rounding errors. This possible rounding is to be expected, so this lint is
    /// `Allow` by default.
    ///
    /// Basically, this warns on casting any integer with 32 or more bits to `f32`
    /// or any 64-bit integer to `f64`.
    ///
    /// ### Why is this bad?
    /// It's not bad at all. But in some applications it can be
    /// helpful to know where precision loss can take place. This lint can help find
    /// those places in the code.
    ///
    /// ### Example
    /// ```no_run
    /// let x = u64::MAX;
    /// x as f64;
    /// ```
    #[clippy::version = "pre 1.29.0"]
    pub CAST_PRECISION_LOSS,
    pedantic,
    "casts that cause loss of precision, e.g., `x as f32` where `x: u64`"
}

pub(super) fn check(cx: &LateContext<'_>, expr: &Expr<'_>, cast_from: Ty<'_>, cast_to: Ty<'_>) {
    let Some(from_nbits) = utils::int_ty_to_nbits(cx.tcx, cast_from) else {
        return;
    };

    // FIXME: handle `f16` and `f128`
    let to_nbits = match cast_to.kind() {
        ty::Float(f @ (FloatTy::F32 | FloatTy::F64)) => f.bit_width(),
        _ => return,
    };

    if !(is_isize_or_usize(cast_from) || from_nbits >= to_nbits) {
        return;
    }

    let cast_to_f64 = to_nbits == 64;
    let mantissa_nbits = if cast_to_f64 { 52 } else { 23 };

    let has_width = if is_isize_or_usize(cast_from) {
        "can be up to 64 bits wide depending on the target architecture".to_owned()
    } else {
        format!("is {from_nbits} bits wide")
    };

    span_lint(
        cx,
        CAST_PRECISION_LOSS,
        expr.span,
        format!(
            "casting `{cast_from}` to `{cast_to}` may cause a loss of precision \
            (`{cast_from}` {has_width}, \
             but `{cast_to}`'s mantissa is only {mantissa_nbits} bits wide)",
        ),
    );
}
