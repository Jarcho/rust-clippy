use super::Attribute;
use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::source::SpanExt as _;
use rustc_ast::{MetaItemInner, MetaItemKind};
use rustc_errors::Applicability;
use rustc_lint::EarlyContext;
use rustc_span::sym;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for `any` and `all` combinators in `cfg` with only one condition.
    ///
    /// ### Why is this bad?
    /// If there is only one condition, no need to wrap it into `any` or `all` combinators.
    ///
    /// ### Example
    /// ```no_run
    /// #[cfg(any(unix))]
    /// pub struct Bar;
    /// ```
    ///
    /// Use instead:
    /// ```no_run
    /// #[cfg(unix)]
    /// pub struct Bar;
    /// ```
    #[clippy::version = "1.71.0"]
    pub NON_MINIMAL_CFG,
    style,
    "ensure that all `cfg(any())` and `cfg(all())` have more than one condition"
}

pub(super) fn check(cx: &EarlyContext<'_>, attr: &Attribute) {
    if attr.has_name(sym::cfg)
        && let Some(items) = attr.meta_item_list()
    {
        check_nested_cfg(cx, &items);
    }
}

fn check_nested_cfg(cx: &EarlyContext<'_>, items: &[MetaItemInner]) {
    for item in items {
        if let MetaItemInner::MetaItem(meta) = item {
            if !meta.has_name(sym::any) && !meta.has_name(sym::all) {
                continue;
            }
            if let MetaItemKind::List(list) = &meta.kind {
                check_nested_cfg(cx, list);
                if list.len() == 1 {
                    span_lint_and_then(
                        cx,
                        NON_MINIMAL_CFG,
                        meta.span,
                        "unneeded sub `cfg` when there is only one condition",
                        |diag| {
                            if let Some(snippet) = list[0].span().get_text(cx) {
                                diag.span_suggestion(
                                    meta.span,
                                    "try",
                                    snippet.to_owned(),
                                    Applicability::MaybeIncorrect,
                                );
                            }
                        },
                    );
                } else if list.is_empty() && meta.has_name(sym::all) {
                    span_lint_and_then(
                        cx,
                        NON_MINIMAL_CFG,
                        meta.span,
                        "unneeded sub `cfg` when there is no condition",
                        |_| {},
                    );
                }
            }
        }
    }
}
