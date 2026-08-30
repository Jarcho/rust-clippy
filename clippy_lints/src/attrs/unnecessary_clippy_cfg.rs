use super::Attribute;
use clippy_utils::diagnostics::{span_lint_and_note, span_lint_and_sugg};
use clippy_utils::source::SpanExt as _;
use itertools::Itertools as _;
use rustc_ast::AttrStyle;
use rustc_errors::Applicability;
use rustc_lint::{EarlyContext, Level};
use rustc_span::sym;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for `#[cfg_attr(clippy, allow(clippy::lint))]`
    /// and suggests to replace it with `#[allow(clippy::lint)]`.
    ///
    /// ### Why is this bad?
    /// There is no reason to put clippy attributes behind a clippy `cfg` as they are not
    /// run by anything else than clippy.
    ///
    /// ### Example
    /// ```no_run
    /// #![cfg_attr(clippy, allow(clippy::deprecated_cfg_attr))]
    /// ```
    ///
    /// Use instead:
    /// ```no_run
    /// #![allow(clippy::deprecated_cfg_attr)]
    /// ```
    #[clippy::version = "1.78.0"]
    pub UNNECESSARY_CLIPPY_CFG,
    suspicious,
    "usage of `cfg_attr(clippy, allow(clippy::lint))` instead of `allow(clippy::lint)`"
}

pub(super) fn check(
    cx: &EarlyContext<'_>,
    cfg_attr: &rustc_ast::MetaItem,
    behind_cfg_attr: &rustc_ast::MetaItem,
    attr: &Attribute,
) {
    if cfg_attr.has_name(sym::clippy)
        && let Some(ident) = behind_cfg_attr.ident()
        && Level::from_symbol(ident.name).is_some()
        && let Some(items) = behind_cfg_attr.meta_item_list()
    {
        let nb_items = items.len();
        let mut clippy_lints = Vec::with_capacity(items.len());
        for item in items {
            if let Some(meta_item) = item.meta_item()
                && let [part1, _] = meta_item.path.segments.as_slice()
                && part1.ident.name == sym::clippy
            {
                clippy_lints.push(item.span());
            }
        }
        if clippy_lints.is_empty() {
            return;
        }
        if nb_items == clippy_lints.len() {
            if let Some(snippet) = behind_cfg_attr.span.get_text(cx) {
                span_lint_and_sugg(
                    cx,
                    UNNECESSARY_CLIPPY_CFG,
                    attr.span,
                    "no need to put clippy lints behind a `clippy` cfg",
                    "replace with",
                    format!(
                        "#{}[{}]",
                        if attr.style == AttrStyle::Inner { "!" } else { "" },
                        snippet
                    ),
                    Applicability::MachineApplicable,
                );
            }
        } else {
            let snippet = clippy_lints.iter().filter_map(|sp| sp.get_text(cx)).join(",");
            span_lint_and_note(
                cx,
                UNNECESSARY_CLIPPY_CFG,
                clippy_lints,
                "no need to put clippy lints behind a `clippy` cfg",
                None,
                format!(
                    "write instead: `#{}[{}({})]`",
                    if attr.style == AttrStyle::Inner { "!" } else { "" },
                    ident.name,
                    snippet
                ),
            );
        }
    }
}
