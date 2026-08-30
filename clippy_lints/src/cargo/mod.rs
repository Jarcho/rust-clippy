pub mod common_metadata;
pub mod feature_name;
pub mod lint_groups_priority;
pub mod multiple_crate_versions;
pub mod wildcard_dependencies;

use cargo_metadata::MetadataCommand;
use clippy_config::Conf;
use clippy_utils::diagnostics::span_lint;
use clippy_utils::is_lint_allowed;
use rustc_data_structures::fx::FxHashSet;
use rustc_hir::CRATE_HIR_ID;
use rustc_lint::{LateContext, LateLintPass, Lint, impl_lint_pass};
use rustc_span::DUMMY_SP;

impl_lint_pass!(Cargo => [
    common_metadata::CARGO_COMMON_METADATA,
    feature_name::NEGATIVE_FEATURE_NAMES,
    feature_name::REDUNDANT_FEATURE_NAMES,
    lint_groups_priority::LINT_GROUPS_PRIORITY,
    multiple_crate_versions::MULTIPLE_CRATE_VERSIONS,
    wildcard_dependencies::WILDCARD_DEPENDENCIES,
]);

pub struct Cargo {
    allowed_duplicate_crates: &'static FxHashSet<String>,
    ignore_publish: bool,
}

impl Cargo {
    pub fn new(conf: &'static Conf) -> Self {
        Self {
            allowed_duplicate_crates: &conf.allowed_duplicate_crates,
            ignore_publish: conf.cargo_ignore_publish,
        }
    }
}

impl LateLintPass<'_> for Cargo {
    fn check_crate(&mut self, cx: &LateContext<'_>) {
        static NO_DEPS_LINTS: &[&Lint] = &[
            common_metadata::CARGO_COMMON_METADATA,
            feature_name::REDUNDANT_FEATURE_NAMES,
            feature_name::NEGATIVE_FEATURE_NAMES,
            wildcard_dependencies::WILDCARD_DEPENDENCIES,
        ];
        static WITH_DEPS_LINTS: &[&Lint] = &[multiple_crate_versions::MULTIPLE_CRATE_VERSIONS];

        lint_groups_priority::check(cx);

        if !NO_DEPS_LINTS
            .iter()
            .all(|&lint| is_lint_allowed(cx, lint, CRATE_HIR_ID))
        {
            match MetadataCommand::new().no_deps().exec() {
                Ok(metadata) => {
                    common_metadata::check(cx, &metadata, self.ignore_publish);
                    feature_name::check(cx, &metadata);
                    wildcard_dependencies::check(cx, &metadata);
                },
                Err(e) => {
                    for lint in NO_DEPS_LINTS {
                        span_lint(cx, lint, DUMMY_SP, format!("could not read cargo metadata: {e}"));
                    }
                },
            }
        }

        if !WITH_DEPS_LINTS
            .iter()
            .all(|&lint| is_lint_allowed(cx, lint, CRATE_HIR_ID))
        {
            match MetadataCommand::new().exec() {
                Ok(metadata) => {
                    multiple_crate_versions::check(cx, &metadata, self.allowed_duplicate_crates);
                },
                Err(e) => {
                    for lint in WITH_DEPS_LINTS {
                        span_lint(cx, lint, DUMMY_SP, format!("could not read cargo metadata: {e}"));
                    }
                },
            }
        }
    }
}
