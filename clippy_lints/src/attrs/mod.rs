pub mod allow_attributes;
pub mod allow_attributes_without_reason;
pub mod blanket_clippy_restriction_lints;
pub mod deprecated_cfg_attr;
pub mod deprecated_semver;
pub mod duplicated_attributes;
pub mod inline_always;
pub mod mixed_attributes_style;
pub mod non_minimal_cfg;
pub mod repr_attributes;
pub mod should_panic_without_expect;
pub mod unnecessary_clippy_cfg;
pub mod useless_attribute;

mod utils;

use clippy_config::Conf;
use clippy_utils::check_clippy_attr;
use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::msrvs::{self, Msrv, MsrvStack};
use rustc_ast::{self as ast, AttrArgs, AttrKind, Attribute, MetaItemInner, MetaItemKind};
use rustc_hir::{ImplItem, ImplItemKind, Item, ItemKind, TraitFn, TraitItem, TraitItemKind};
use rustc_lint::{EarlyContext, EarlyLintPass, LateContext, LateLintPass, LintContext as _, impl_lint_pass};
use rustc_span::sym;
use utils::is_lint_level;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for ignored tests without messages.
    ///
    /// ### Why is this bad?
    /// The reason for ignoring the test may not be obvious.
    ///
    /// ### Example
    /// ```no_run
    /// #[test]
    /// #[ignore]
    /// fn test() {}
    /// ```
    /// Use instead:
    /// ```no_run
    /// #[test]
    /// #[ignore = "Some good reason"]
    /// fn test() {}
    /// ```
    ///
    /// ### Note
    /// Clippy can only lint compiled code. For this lint to trigger, you must configure `cargo clippy`
    /// to include test compilation, for instance, by using flags such as `--tests` or `--all-targets`.
    #[clippy::version = "1.88.0"]
    pub IGNORE_WITHOUT_REASON,
    pedantic,
    "ignored tests without messages"
}

impl_lint_pass!(Attributes => [
    inline_always::INLINE_ALWAYS,
    repr_attributes::REPR_PACKED_WITHOUT_ABI,
]);

impl_lint_pass!(EarlyAttributes => [
    deprecated_cfg_attr::DEPRECATED_CFG_ATTR,
    deprecated_cfg_attr::DEPRECATED_CLIPPY_CFG_ATTR,
    non_minimal_cfg::NON_MINIMAL_CFG,
    unnecessary_clippy_cfg::UNNECESSARY_CLIPPY_CFG,
]);

impl_lint_pass!(PostExpansionEarlyAttributes => [
    IGNORE_WITHOUT_REASON,
    allow_attributes::ALLOW_ATTRIBUTES,
    allow_attributes_without_reason::ALLOW_ATTRIBUTES_WITHOUT_REASON,
    blanket_clippy_restriction_lints::BLANKET_CLIPPY_RESTRICTION_LINTS,
    deprecated_semver::DEPRECATED_SEMVER,
    duplicated_attributes::DUPLICATED_ATTRIBUTES,
    mixed_attributes_style::MIXED_ATTRIBUTES_STYLE,
    should_panic_without_expect::SHOULD_PANIC_WITHOUT_EXPECT,
    useless_attribute::USELESS_ATTRIBUTE,
]);

pub struct Attributes {
    msrv: Msrv,
}

impl Attributes {
    pub fn new(conf: &'static Conf) -> Self {
        Self { msrv: conf.msrv.into() }
    }
}

impl<'tcx> LateLintPass<'tcx> for Attributes {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'_>) {
        let attrs = cx.tcx.hir_attrs(item.hir_id());
        if let ItemKind::Fn { ident, body, .. } = item.kind {
            inline_always::check(cx, item.span, ident.name, attrs, Some(body));
        }
        repr_attributes::check(cx, item.span, attrs, self.msrv);
    }

    fn check_impl_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx ImplItem<'_>) {
        if let ImplItemKind::Fn(_, body) = item.kind {
            inline_always::check(
                cx,
                item.span,
                item.ident.name,
                cx.tcx.hir_attrs(item.hir_id()),
                Some(body),
            );
        }
    }

    fn check_trait_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx TraitItem<'_>) {
        if let TraitItemKind::Fn(_, kind) = item.kind {
            let body = match kind {
                TraitFn::Required(_) => None,
                TraitFn::Provided(body) => Some(body),
            };
            inline_always::check(cx, item.span, item.ident.name, cx.tcx.hir_attrs(item.hir_id()), body);
        }
    }
}

pub struct EarlyAttributes {
    msrv: MsrvStack,
}

impl EarlyAttributes {
    pub fn new(conf: &'static Conf) -> Self {
        Self { msrv: conf.msrv.into() }
    }
}

impl EarlyLintPass for EarlyAttributes {
    fn check_attribute(&mut self, cx: &EarlyContext<'_>, attr: &Attribute) {
        deprecated_cfg_attr::check(cx, attr, &self.msrv);
        deprecated_cfg_attr::check_clippy(cx, attr);
        non_minimal_cfg::check(cx, attr);
    }

    extract_msrv_attr!();
}

pub struct PostExpansionEarlyAttributes {
    msrv: MsrvStack,
}

impl PostExpansionEarlyAttributes {
    pub fn new(conf: &'static Conf) -> Self {
        Self { msrv: conf.msrv.into() }
    }
}

impl EarlyLintPass for PostExpansionEarlyAttributes {
    fn check_crate(&mut self, cx: &EarlyContext<'_>, _krate: &ast::Crate) {
        blanket_clippy_restriction_lints::check_command_line(cx);
    }

    fn check_attribute(&mut self, cx: &EarlyContext<'_>, attr: &Attribute) {
        check_clippy_attr(cx.sess(), attr);
        if let Some(items) = &attr.meta_item_list()
            && let Some(name) = attr.name()
        {
            if matches!(name, sym::allow) && self.msrv.meets(msrvs::LINT_REASONS_STABILIZATION) {
                allow_attributes::check(cx, attr);
            }
            if matches!(name, sym::allow | sym::expect) && self.msrv.meets(msrvs::LINT_REASONS_STABILIZATION) {
                allow_attributes_without_reason::check(cx, name, items, attr);
            }
            if is_lint_level(name) {
                blanket_clippy_restriction_lints::check(cx, name, items);
            }
            if items.is_empty() || !attr.has_name(sym::deprecated) {
                return;
            }
            for item in items {
                if let MetaItemInner::MetaItem(mi) = &item
                    && let MetaItemKind::NameValue(lit) = &mi.kind
                    && mi.has_name(sym::since)
                {
                    deprecated_semver::check(cx, item.span(), lit);
                }
            }
        }

        if attr.has_name(sym::should_panic) {
            should_panic_without_expect::check(cx, attr);
        }

        if attr.has_name(sym::ignore)
            && let AttrKind::Normal(normal_attr) = &attr.kind
            && !matches!(normal_attr.item.args, AttrArgs::Eq { .. })
        {
            span_lint_and_help(
                cx,
                IGNORE_WITHOUT_REASON,
                attr.span,
                "`#[ignore]` without reason",
                None,
                "add a reason with `= \"..\"`",
            );
        }
    }

    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &'_ ast::Item) {
        match item.kind {
            ast::ItemKind::ExternCrate(..) | ast::ItemKind::Use(..) => useless_attribute::check(cx, item, &item.attrs),
            _ => {},
        }

        mixed_attributes_style::check(cx, item.span, &item.attrs);
    }

    fn check_attributes(&mut self, cx: &EarlyContext<'_>, attrs: &[Attribute]) {
        self.msrv.check_attributes(attrs);
        duplicated_attributes::check(cx, attrs);
        msrvs::check_attrs(cx.sess(), attrs);
    }

    fn check_attributes_post(&mut self, _cx: &EarlyContext<'_>, attrs: &[Attribute]) {
        self.msrv.check_attributes_post(attrs);
    }
}
