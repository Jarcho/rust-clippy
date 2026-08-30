use cargo_metadata::Metadata;
use clippy_utils::diagnostics::span_lint;
use rustc_lint::LateContext;
use rustc_span::DUMMY_SP;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for wildcard dependencies in the `Cargo.toml`.
    ///
    /// ### Why is this bad?
    /// [As the edition guide says](https://rust-lang-nursery.github.io/edition-guide/rust-2018/cargo-and-crates-io/crates-io-disallows-wildcard-dependencies.html),
    /// it is highly unlikely that you work with any possible version of your dependency,
    /// and wildcard dependencies would cause unnecessary breakage in the ecosystem.
    ///
    /// ### Example
    /// ```toml
    /// [dependencies]
    /// regex = "*"
    /// ```
    /// Use instead:
    /// ```toml
    /// [dependencies]
    /// # allow patch updates, but not minor or major version changes
    /// some_crate_1 = "~1.2.3"
    ///
    /// # pin the version to a specific version
    /// some_crate_2 = "=1.2.3"
    /// ```
    #[clippy::version = "1.32.0"]
    pub WILDCARD_DEPENDENCIES,
    cargo,
    "wildcard dependencies being used"
}

pub(super) fn check(cx: &LateContext<'_>, metadata: &Metadata) {
    for dep in &metadata.packages[0].dependencies {
        // VersionReq::any() does not work
        if let Ok(wildcard_ver) = semver::VersionReq::parse("*")
            && let Some(ref source) = dep.source
            && !source.repr.starts_with("git")
            && dep.req == wildcard_ver
        {
            span_lint(
                cx,
                WILDCARD_DEPENDENCIES,
                DUMMY_SP,
                format!("wildcard dependency for `{}`", dep.name),
            );
        }
    }
}
