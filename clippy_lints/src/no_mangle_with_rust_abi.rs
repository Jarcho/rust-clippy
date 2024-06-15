use clippy_utils::diagnostics::span_lint_and_then;
use clippy_utils::source::get_source_text;
use clippy_utils::tokenize_with_text_and_pos;
use core::ops::ControlFlow;
use rustc_errors::Applicability;
use rustc_hir::{Item, ItemKind};
use rustc_lexer::TokenKind;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::{sym, BytePos, Pos, SpanData};
use rustc_target::spec::abi::Abi;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for Rust ABI functions with the `#[no_mangle]` attribute.
    ///
    /// ### Why is this bad?
    /// The Rust ABI is not stable, but in many simple cases matches
    /// enough with the C ABI that it is possible to forget to add
    /// `extern "C"` to a function called from C. Changes to the
    /// Rust ABI can break this at any point.
    ///
    /// ### Example
    /// ```no_run
    ///  #[no_mangle]
    ///  fn example(arg_one: u32, arg_two: usize) {}
    /// ```
    ///
    /// Use instead:
    /// ```no_run
    ///  #[no_mangle]
    ///  extern "C" fn example(arg_one: u32, arg_two: usize) {}
    /// ```
    #[clippy::version = "1.69.0"]
    pub NO_MANGLE_WITH_RUST_ABI,
    pedantic,
    "convert Rust ABI functions to C ABI"
}
declare_lint_pass!(NoMangleWithRustAbi => [NO_MANGLE_WITH_RUST_ABI]);

impl<'tcx> LateLintPass<'tcx> for NoMangleWithRustAbi {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if let ItemKind::Fn(fn_sig, _, _) = &item.kind
            && fn_sig.header.abi == Abi::Rust
            && cx
                .tcx
                .hir()
                .attrs(item.hir_id())
                .iter()
                .any(|attr| attr.ident().is_some_and(|name| name.name == sym::no_mangle))
            && let Some(src) = get_source_text(cx, fn_sig.span)
            && let Some(src) = src.as_str()
        {
            if let ControlFlow::Break((pos, false)) = tokenize_with_text_and_pos(src)
                .filter(|(t, ..)| matches!(t, TokenKind::Ident))
                .try_fold(false, |found, (_, pos, s)| match s {
                    "fn" => ControlFlow::Break((pos, found)),
                    "extern" => ControlFlow::Continue(true),
                    _ => ControlFlow::Continue(found),
                })
            {
                span_lint_and_then(
                    cx,
                    NO_MANGLE_WITH_RUST_ABI,
                    fn_sig.span,
                    "`#[no_mangle]` set on a function with the default (`Rust`) ABI",
                    |diag| {
                        let data = fn_sig.span.data();
                        let pos = data.lo + BytePos::from_u32(pos);
                        let span = SpanData {
                            lo: pos,
                            hi: pos,
                            ..data
                        }
                        .span();

                        diag.span_suggestion(span, "set an ABI", "extern \"C\" ", Applicability::MaybeIncorrect)
                            .span_suggestion(
                                span,
                                "or explicitly set the default",
                                "extern \"Rust\" ",
                                Applicability::MaybeIncorrect,
                            );
                    },
                );
            }
        };
    }
}
