use clippy_utils::diagnostics::span_lint;
use clippy_utils::SpanlessEq;
use rustc_hir::{BinOpKind, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::lint::in_external_macro;
use rustc_middle::ty;
use rustc_session::declare_lint_pass;

declare_clippy_lint! {
    /// ### What it does
    /// Detects classic underflow/overflow checks.
    ///
    /// ### Why is this bad?
    /// Most classic C underflow/overflow checks will fail in
    /// Rust. Users can use functions like `overflowing_*` and `wrapping_*` instead.
    ///
    /// ### Example
    /// ```no_run
    /// # let a = 1;
    /// # let b = 2;
    /// a + b < a;
    /// ```
    #[clippy::version = "pre 1.29.0"]
    pub OVERFLOW_CHECK_CONDITIONAL,
    complexity,
    "overflow checks inspired by C which are likely to panic"
}

declare_lint_pass!(OverflowCheckConditional => [OVERFLOW_CHECK_CONDITIONAL]);

impl<'tcx> LateLintPass<'tcx> for OverflowCheckConditional {
    // a + b < a, a > a + b, a < a - b, a - b > a
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if let ExprKind::Binary(op, lhs, rhs) = expr.kind
            && let (lt, gt) = match op.node {
                BinOpKind::Lt => (lhs, rhs),
                BinOpKind::Gt => (rhs, lhs),
                _ => return,
            }
            && let ctxt = expr.span.ctxt()
            && let (op_lhs, op_rhs, other, commutative) = match (&lt.kind, &gt.kind) {
                (&ExprKind::Binary(op, lhs, rhs), _) if op.node == BinOpKind::Add && ctxt == lt.span.ctxt() => {
                    (lhs, rhs, gt, true)
                },
                (_, &ExprKind::Binary(op, lhs, rhs)) if op.node == BinOpKind::Sub && ctxt == gt.span.ctxt() => {
                    (lhs, rhs, lt, false)
                },
                _ => return,
            }
            && !other.can_have_side_effects()
            && let typeck = cx.typeck_results()
            && let ty = typeck.expr_ty(op_lhs)
            && matches!(ty.kind(), ty::Uint(_))
            && ty == typeck.expr_ty(op_rhs)
            && ty == typeck.expr_ty(other)
            && !in_external_macro(cx.tcx.sess, expr.span)
            && (SpanlessEq::new(cx).eq_expr(op_lhs, other)
                || (commutative && SpanlessEq::new(cx).eq_expr(op_rhs, other)))
        {
            span_lint(
                cx,
                OVERFLOW_CHECK_CONDITIONAL,
                expr.span,
                "you are trying to use classic C overflow conditions that will fail in Rust",
            );
        }
    }
}
