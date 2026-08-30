use super::utils::extract_clippy_lint;
use clippy_utils::diagnostics::{span_lint_and_help, span_lint_and_then};
use clippy_utils::sym;
use rustc_ast::MetaItemInner;
use rustc_lint::{EarlyContext, Level, LintContext as _};
use rustc_span::DUMMY_SP;
use rustc_span::symbol::Symbol;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for `warn`/`deny`/`forbid` attributes targeting the whole clippy::restriction category.
    ///
    /// ### Why is this bad?
    /// Restriction lints sometimes are in contrast with other lints or even go against idiomatic rust.
    /// These lints should only be enabled on a lint-by-lint basis and with careful consideration.
    ///
    /// ### Example
    /// ```no_run
    /// #![deny(clippy::restriction)]
    /// ```
    ///
    /// Use instead:
    /// ```no_run
    /// #![deny(clippy::as_conversions)]
    /// ```
    #[clippy::version = "1.47.0"]
    pub BLANKET_CLIPPY_RESTRICTION_LINTS,
    suspicious,
    "enabling the complete restriction group"
}

pub(super) fn check(cx: &EarlyContext<'_>, name: Symbol, items: &[MetaItemInner]) {
    for lint in items {
        if name != sym::allow && extract_clippy_lint(lint) == Some(sym::restriction) {
            span_lint_and_help(
                cx,
                BLANKET_CLIPPY_RESTRICTION_LINTS,
                lint.span(),
                "`clippy::restriction` is not meant to be enabled as a group",
                None,
                "enable the restriction lints you need individually",
            );
        }
    }
}

pub(super) fn check_command_line(cx: &EarlyContext<'_>) {
    for (name, level) in &cx.sess().opts.lint_opts {
        if name == "clippy::restriction" && *level > Level::Allow {
            span_lint_and_then(
                cx,
                BLANKET_CLIPPY_RESTRICTION_LINTS,
                DUMMY_SP,
                "`clippy::restriction` is not meant to be enabled as a group",
                |diag| {
                    diag.note(format!(
                        "because of the command line `--{} clippy::restriction`",
                        level.as_str()
                    ));
                    diag.help("enable the restriction lints you need individually");
                },
            );
        }
    }
}
