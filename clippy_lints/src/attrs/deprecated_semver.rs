use clippy_utils::diagnostics::span_lint;
use clippy_utils::sym;
use rustc_ast::{LitKind, MetaItemLit};
use rustc_hir::VERSION_PLACEHOLDER;
use rustc_lint::EarlyContext;
use rustc_span::Span;
use semver::Version;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for `#[deprecated]` annotations with a `since`
    /// field that is not a valid semantic version. Also allows "TBD" to signal
    /// future deprecation.
    ///
    /// ### Why is this bad?
    /// For checking the version of the deprecation, it must be
    /// a valid semver. Failing that, the contained information is useless.
    ///
    /// ### Example
    /// ```no_run
    /// #[deprecated(since = "forever")]
    /// fn something_else() { /* ... */ }
    /// ```
    #[clippy::version = "pre 1.29.0"]
    pub DEPRECATED_SEMVER,
    correctness,
    "use of `#[deprecated(since = \"x\")]` where x is not semver"
}

pub(super) fn check(cx: &EarlyContext<'_>, span: Span, lit: &MetaItemLit) {
    if let LitKind::Str(is, _) = lit.kind
        && (is == sym::TBD || is.as_str() == VERSION_PLACEHOLDER || Version::parse(is.as_str()).is_ok())
    {
        return;
    }
    span_lint(
        cx,
        DEPRECATED_SEMVER,
        span,
        "the since field must contain a semver-compliant version",
    );
}
