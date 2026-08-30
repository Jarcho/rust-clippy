use super::utils::{is_lint_level, is_word, namespace_and_lint};
use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::source::{SpanExt as _, first_line_of_span};
use clippy_utils::sym;
use rustc_ast::{Attribute, Item, ItemKind};
use rustc_errors::Applicability;
use rustc_lint::{EarlyContext, LintContext as _};

declare_clippy_lint! {
    /// ### What it does
    /// Checks for `extern crate` and `use` items annotated with
    /// lint attributes.
    ///
    /// This lint permits lint attributes for lints emitted on the items themself.
    /// For `use` items these lints are:
    /// * ambiguous_glob_reexports
    /// * dead_code
    /// * deprecated
    /// * hidden_glob_reexports
    /// * unreachable_pub
    /// * unused
    /// * unused_braces
    /// * unused_import_braces
    /// * clippy::disallowed_types
    /// * clippy::enum_glob_use
    /// * clippy::macro_use_imports
    /// * clippy::module_name_repetitions
    /// * clippy::redundant_pub_crate
    /// * clippy::single_component_path_imports
    /// * clippy::unsafe_removed_from_name
    /// * clippy::wildcard_imports
    ///
    /// For `extern crate` items these lints are:
    /// * `unused_imports` on items with `#[macro_use]`
    ///
    /// ### Why is this bad?
    /// Lint attributes have no effect on crate imports. Most
    /// likely a `!` was forgotten.
    ///
    /// ### Example
    /// ```ignore
    /// #[deny(dead_code)]
    /// extern crate foo;
    /// #[forbid(dead_code)]
    /// use foo::bar;
    /// ```
    ///
    /// Use instead:
    /// ```rust,ignore
    /// #[allow(unused_imports)]
    /// use foo::baz;
    /// #[allow(unused_imports)]
    /// #[macro_use]
    /// extern crate baz;
    /// ```
    #[clippy::version = "pre 1.29.0"]
    pub USELESS_ATTRIBUTE,
    correctness,
    "use of lint attributes on `extern crate` items"
}

pub(super) fn check(cx: &EarlyContext<'_>, item: &Item, attrs: &[Attribute]) {
    let skip_unused_imports = attrs.iter().any(|attr| attr.has_name(sym::macro_use));

    for attr in attrs {
        if let Some(lint_list) = &attr.meta_item_list()
            && attr.name().is_some_and(is_lint_level)
        {
            for lint in lint_list {
                match item.kind {
                    ItemKind::Use(..) => {
                        let (namespace @ (Some(sym::clippy) | None), Some(name)) = namespace_and_lint(lint) else {
                            return;
                        };

                        if namespace.is_none()
                            && matches!(
                                name,
                                sym::ambiguous_glob_reexports
                                    | sym::dead_code
                                    | sym::deprecated
                                    | sym::deprecated_in_future
                                    | sym::exported_private_dependencies
                                    | sym::hidden_glob_reexports
                                    | sym::unreachable_pub
                                    | sym::unused
                                    | sym::unused_braces
                                    | sym::unused_import_braces
                                    | sym::unused_imports
                                    | sym::redundant_imports
                            )
                        {
                            return;
                        }

                        if namespace == Some(sym::clippy)
                            && matches!(
                                name,
                                sym::wildcard_imports
                                    | sym::enum_glob_use
                                    | sym::redundant_pub_crate
                                    | sym::macro_use_imports
                                    | sym::unsafe_removed_from_name
                                    | sym::module_name_repetitions
                                    | sym::single_component_path_imports
                                    | sym::disallowed_types
                                    | sym::unused_trait_names
                            )
                        {
                            return;
                        }
                    },
                    ItemKind::ExternCrate(..) => {
                        if is_word(lint, sym::unused_imports) && skip_unused_imports {
                            return;
                        }
                        if is_word(lint, sym::unused_extern_crates) {
                            return;
                        }
                    },
                    _ => {},
                }
            }

            if !attr.span.in_external_macro(cx.sess().source_map())
                && let line_span = first_line_of_span(cx, attr.span)
                && let Some(src) = line_span.get_text(cx)
                && src.contains("#[")
            {
                #[expect(clippy::collapsible_span_lint_calls)]
                span_lint_and_then(cx, USELESS_ATTRIBUTE, line_span, "useless lint attribute", |diag| {
                    diag.span_suggestion(
                        line_span,
                        "if you just forgot a `!`, use",
                        src.replacen("#[", "#![", 1),
                        Applicability::MaybeIncorrect,
                    );
                });
            }
        }
    }
}
