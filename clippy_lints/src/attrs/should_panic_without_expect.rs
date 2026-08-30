use super::Attribute;
use clippy_utils::diagnostics::span_lint_and_sugg;
use rustc_ast::token::{Token, TokenKind};
use rustc_ast::tokenstream::TokenTree;
use rustc_ast::{AttrArgs, AttrKind};
use rustc_errors::Applicability;
use rustc_lint::EarlyContext;
use rustc_span::sym;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for `#[should_panic]` attributes without specifying the expected panic message.
    ///
    /// ### Why is this bad?
    /// The expected panic message should be specified to ensure that the test is actually
    /// panicking with the expected message, and not another unrelated panic.
    ///
    /// ### Example
    /// ```no_run
    /// fn random() -> i32 { 0 }
    ///
    /// #[should_panic]
    /// #[test]
    /// fn my_test() {
    ///     let _ = 1 / random();
    /// }
    /// ```
    ///
    /// Use instead:
    /// ```no_run
    /// fn random() -> i32 { 0 }
    ///
    /// #[should_panic = "attempt to divide by zero"]
    /// #[test]
    /// fn my_test() {
    ///     let _ = 1 / random();
    /// }
    /// ```
    #[clippy::version = "1.74.0"]
    pub SHOULD_PANIC_WITHOUT_EXPECT,
    pedantic,
    "ensures that all `should_panic` attributes specify its expected panic message"
}

pub(super) fn check(cx: &EarlyContext<'_>, attr: &Attribute) {
    if let AttrKind::Normal(normal_attr) = &attr.kind {
        if let AttrArgs::Eq { .. } = &normal_attr.item.args {
            // `#[should_panic = ".."]` found, good
            return;
        }

        if let AttrArgs::Delimited(args) = &normal_attr.item.args
            && let mut tt_iter = args.tokens.iter()
            && let Some(TokenTree::Token(
                Token {
                    kind: TokenKind::Ident(sym::expected, _),
                    ..
                },
                _,
            )) = tt_iter.next()
            && let Some(TokenTree::Token(
                Token {
                    kind: TokenKind::Eq, ..
                },
                _,
            )) = tt_iter.next()
            && let Some(TokenTree::Token(
                Token {
                    kind: TokenKind::Literal(_),
                    ..
                },
                _,
            )) = tt_iter.next()
        {
            // `#[should_panic(expected = "..")]` found, good
            return;
        }

        span_lint_and_sugg(
            cx,
            SHOULD_PANIC_WITHOUT_EXPECT,
            attr.span,
            "#[should_panic] attribute without a reason",
            "consider specifying the expected panic",
            "#[should_panic(expected = /* panic message */)]".into(),
            Applicability::HasPlaceholders,
        );
    }
}
