use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::is_from_proc_macro;
use rustc_ast::attr::AttributeExt as _;
use rustc_ast::{AttrStyle, Attribute};
use rustc_errors::Applicability;
use rustc_lint::{EarlyContext, LintContext as _};

declare_clippy_lint! {
    /// ### What it does
    /// Checks for usage of the `#[allow]` attribute and suggests replacing it with
    /// the `#[expect]` attribute (See [RFC 2383](https://rust-lang.github.io/rfcs/2383-lint-reasons.html))
    ///
    /// This lint only warns outer attributes (`#[allow]`), as inner attributes
    /// (`#![allow]`) are usually used to enable or disable lints on a global scale.
    ///
    /// ### Why is this bad?
    /// `#[expect]` attributes suppress the lint emission, but emit a warning, if
    /// the expectation is unfulfilled. This can be useful to be notified when the
    /// lint is no longer triggered.
    ///
    /// ### Example
    /// ```rust,ignore
    /// #[allow(unused_mut)]
    /// fn foo() -> usize {
    ///     let mut a = Vec::new();
    ///     a.len()
    /// }
    /// ```
    /// Use instead:
    /// ```rust,ignore
    /// #[expect(unused_mut)]
    /// fn foo() -> usize {
    ///     let mut a = Vec::new();
    ///     a.len()
    /// }
    /// ```
    #[clippy::version = "1.70.0"]
    pub ALLOW_ATTRIBUTES,
    restriction,
    "`#[allow]` will not trigger if a warning isn't found. `#[expect]` triggers if there are no warnings."
}

// Separate each crate's features.
pub fn check<'cx>(cx: &EarlyContext<'cx>, attr: &'cx Attribute) {
    if let AttrStyle::Outer = attr.style
        && let Some(path_span) = attr.path_span()
        && !attr.span.in_external_macro(cx.sess().source_map())
        && !is_from_proc_macro(cx, attr)
    {
        #[expect(clippy::collapsible_span_lint_calls, reason = "rust-clippy#7797")]
        span_lint_and_then(cx, ALLOW_ATTRIBUTES, path_span, "#[allow] attribute found", |diag| {
            diag.span_suggestion(path_span, "replace it with", "expect", Applicability::MachineApplicable);
        });
    }
}
