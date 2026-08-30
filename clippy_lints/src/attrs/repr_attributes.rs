use rustc_hir::attrs::ReprAttr;
use rustc_hir::{Attribute, find_attr};
use rustc_lint::LateContext;
use rustc_span::Span;

use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::msrvs::{self, Msrv};

declare_clippy_lint! {
    /// ### What it does
    /// Checks for items with `#[repr(packed)]`-attribute without ABI qualification
    ///
    /// ### Why is this bad?
    /// Without qualification, `repr(packed)` implies `repr(Rust)`. The Rust-ABI is inherently unstable.
    /// While this is fine as long as the type is accessed correctly within Rust-code, most uses
    /// of `#[repr(packed)]` involve FFI and/or data structures specified by network-protocols or
    /// other external specifications. In such situations, the unstable Rust-ABI implied in
    /// `#[repr(packed)]` may lead to future bugs should the Rust-ABI change.
    ///
    /// In case you are relying on a well defined and stable memory layout, qualify the type's
    /// representation using the `C`-ABI. Otherwise, if the type in question is only ever
    /// accessed from Rust-code according to Rust's rules, use the `Rust`-ABI explicitly.
    ///
    /// ### Example
    /// ```no_run
    /// #[repr(packed)]
    /// struct NetworkPacketHeader {
    ///     header_length: u8,
    ///     header_version: u16
    /// }
    /// ```
    ///
    /// Use instead:
    /// ```no_run
    /// #[repr(C, packed)]
    /// struct NetworkPacketHeader {
    ///     header_length: u8,
    ///     header_version: u16
    /// }
    /// ```
    #[clippy::version = "1.85.0"]
    pub REPR_PACKED_WITHOUT_ABI,
    suspicious,
    "ensures that `repr(packed)` always comes with a qualified ABI"
}

pub(super) fn check(cx: &LateContext<'_>, item_span: Span, attrs: &[Attribute], msrv: Msrv) {
    if let Some(reprs) = find_attr!(attrs, Repr { reprs, .. } => reprs) {
        let packed_span = reprs
            .iter()
            .find(|(r, _)| matches!(r, ReprAttr::ReprPacked(..)))
            .map(|(_, s)| *s);

        if let Some(packed_span) = packed_span
            && !reprs
                .iter()
                .any(|(x, _)| *x == ReprAttr::ReprC || *x == ReprAttr::ReprRust)
            && msrv.meets(cx, msrvs::REPR_RUST)
        {
            span_lint_and_then(
                cx,
                REPR_PACKED_WITHOUT_ABI,
                item_span,
                "item uses `packed` representation without ABI-qualification",
                |diag| {
                    diag.warn(
                        "unqualified `#[repr(packed)]` defaults to `#[repr(Rust, packed)]`, which has no stable ABI",
                    )
                    .help("qualify the desired ABI explicitly via `#[repr(C, packed)]` or `#[repr(Rust, packed)]`")
                    .span_label(packed_span, "`packed` representation set here");
                },
            );
        }
    }
}
