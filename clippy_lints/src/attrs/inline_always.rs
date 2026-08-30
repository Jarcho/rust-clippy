use super::utils::is_relevant_expr;
use clippy_utils::diagnostics::span_lint;
use rustc_hir::attrs::InlineAttr;
use rustc_hir::{Attribute, BodyId, find_attr};
use rustc_lint::LateContext;
use rustc_span::Span;
use rustc_span::symbol::Symbol;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for items annotated with `#[inline(always)]`,
    /// unless the annotated function is empty or simply panics.
    ///
    /// ### Why is this bad?
    /// While there are valid uses of this annotation (and once
    /// you know when to use it, by all means `allow` this lint), it's a common
    /// newbie-mistake to pepper one's code with it.
    ///
    /// As a rule of thumb, before slapping `#[inline(always)]` on a function,
    /// measure if that additional function call really affects your runtime profile
    /// sufficiently to make up for the increase in compile time.
    ///
    /// ### Known problems
    /// False positives, big time. This lint is meant to be
    /// deactivated by everyone doing serious performance work. This means having
    /// done the measurement.
    ///
    /// ### Example
    /// ```ignore
    /// #[inline(always)]
    /// fn not_quite_hot_code(..) { ... }
    /// ```
    #[clippy::version = "pre 1.29.0"]
    pub INLINE_ALWAYS,
    pedantic,
    "use of `#[inline(always)]`"
}

pub(super) fn check(cx: &LateContext<'_>, span: Span, name: Symbol, attrs: &[Attribute], body: Option<BodyId>) {
    if span.from_expansion() {
        return;
    }

    if let Some(span) = find_attr!(attrs, Inline(InlineAttr::Always, span) => *span)
        && body.is_none_or(|body| is_relevant_expr(cx, cx.tcx.hir_body(body).value))
    {
        span_lint(
            cx,
            INLINE_ALWAYS,
            span,
            format!("you have declared `#[inline(always)]` on `{name}`. This is usually a bad idea"),
        );
    }
}
