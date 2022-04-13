//! This module handles checking if the span given is from a proc-macro or not.
//!
//! Proc-macros are capable of setting the span of every token they output to a few possible spans.
//! This includes spans we can detect easily as coming from a proc-macro (e.g. the call site
//! or the def site), and spans we can't easily detect as such (e.g. the span of any token
//! passed into the proc macro). This capability means proc-macros are capable of generating code
//! with a span that looks like it was written by the user, but which should not be linted by clippy
//! as it was generated by an external macro.
//!
//! That brings us to this module. The current approach is to determine a small bit of text which
//! must exist at both the start and the end of an item (e.g. an expression or a path) assuming the
//! code was written, and check if the span contains that text. Note this will only work correctly
//! if the span is not from a `macro_rules` based macro.

use rustc_ast::ast::{IntTy, LitIntType, LitKind, StrStyle, UintTy};
use rustc_hir::{
    Block, BlockCheckMode, Destination, Expr, ExprKind, LoopSource, MatchSource, QPath, UnOp, UnsafeSource, YieldSource,
};
use rustc_lint::{LateContext, LintContext};
use rustc_middle::ty::TyCtxt;
use rustc_session::Session;
use rustc_span::{Span, Symbol};

#[derive(Clone, Copy)]
enum Pat {
    Str(&'static str),
    Sym(Symbol),
    Num,
}

/// Checks if the start and the end of the span's text matches the patterns. This will return false
/// if the span crosses multiple files or if source is not available.
fn span_matches_pat(sess: &Session, span: Span, start_pat: Pat, end_pat: Pat) -> bool {
    let pos = sess.source_map().lookup_byte_offset(span.lo());
    let Some(ref src) = pos.sf.src else {
        return false;
    };
    let end = span.hi() - pos.sf.start_pos;
    src.get(pos.pos.0 as usize..end.0 as usize).map_or(false, |s| {
        // Spans can be wrapped in a mixture or parenthesis, whitespace, and trailing commas.
        let start_str = s.trim_start_matches(|c: char| c.is_whitespace() || c == '(');
        let end_str = s.trim_end_matches(|c: char| c.is_whitespace() || c == ')' || c == ',');
        (match start_pat {
            Pat::Str(text) => start_str.starts_with(text),
            Pat::Sym(sym) => start_str.starts_with(sym.as_str()),
            Pat::Num => start_str.as_bytes().first().map_or(false, u8::is_ascii_digit),
        } && match end_pat {
            Pat::Str(text) => end_str.ends_with(text),
            Pat::Sym(sym) => end_str.ends_with(sym.as_str()),
            Pat::Num => end_str.as_bytes().last().map_or(false, u8::is_ascii_hexdigit),
        })
    })
}

/// Get the search patterns to use for the given literal
fn lit_search_pat(lit: &LitKind) -> (Pat, Pat) {
    match lit {
        LitKind::Str(_, StrStyle::Cooked) => (Pat::Str("\""), Pat::Str("\"")),
        LitKind::Str(_, StrStyle::Raw(0)) => (Pat::Str("r"), Pat::Str("\"")),
        LitKind::Str(_, StrStyle::Raw(_)) => (Pat::Str("r#"), Pat::Str("#")),
        LitKind::ByteStr(_) => (Pat::Str("b\""), Pat::Str("\"")),
        LitKind::Byte(_) => (Pat::Str("b'"), Pat::Str("'")),
        LitKind::Char(_) => (Pat::Str("'"), Pat::Str("'")),
        LitKind::Int(_, LitIntType::Signed(IntTy::Isize)) => (Pat::Num, Pat::Str("isize")),
        LitKind::Int(_, LitIntType::Unsigned(UintTy::Usize)) => (Pat::Num, Pat::Str("usize")),
        LitKind::Int(..) => (Pat::Num, Pat::Num),
        LitKind::Float(..) => (Pat::Num, Pat::Str("")),
        LitKind::Bool(true) => (Pat::Str("true"), Pat::Str("true")),
        LitKind::Bool(false) => (Pat::Str("false"), Pat::Str("false")),
        _ => (Pat::Str(""), Pat::Str("")),
    }
}

/// Get the search patterns to use for the given path
fn qpath_search_pat(path: &QPath<'_>) -> (Pat, Pat) {
    match path {
        QPath::Resolved(ty, path) => {
            let start = if ty.is_some() {
                Pat::Str("<")
            } else {
                path.segments
                    .first()
                    .map_or(Pat::Str(""), |seg| Pat::Sym(seg.ident.name))
            };
            let end = path.segments.last().map_or(Pat::Str(""), |seg| {
                if seg.args.is_some() {
                    Pat::Str(">")
                } else {
                    Pat::Sym(seg.ident.name)
                }
            });
            (start, end)
        },
        QPath::TypeRelative(_, name) => (Pat::Str(""), Pat::Sym(name.ident.name)),
        QPath::LangItem(..) => (Pat::Str(""), Pat::Str("")),
    }
}

/// Get the search patterns to use for the given expression
fn expr_search_pat(tcx: TyCtxt<'_>, e: &Expr<'_>) -> (Pat, Pat) {
    match e.kind {
        ExprKind::Box(e) => (Pat::Str("box"), expr_search_pat(tcx, e).1),
        ExprKind::ConstBlock(_) => (Pat::Str("const"), Pat::Str("}")),
        ExprKind::Tup([]) => (Pat::Str(")"), Pat::Str("(")),
        ExprKind::Unary(UnOp::Deref, _) => (Pat::Str("*"), expr_search_pat(tcx, e).1),
        ExprKind::Unary(UnOp::Not, _) => (Pat::Str("!"), expr_search_pat(tcx, e).1),
        ExprKind::Unary(UnOp::Neg, _) => (Pat::Str("-"), expr_search_pat(tcx, e).1),
        ExprKind::Lit(ref lit) => lit_search_pat(&lit.node),
        ExprKind::Array(_) | ExprKind::Repeat(..) => (Pat::Str("["), Pat::Str("]")),
        ExprKind::Call(e, []) | ExprKind::MethodCall(_, [e], _) => (expr_search_pat(tcx, e).0, Pat::Str("(")),
        ExprKind::Call(first, [.., last])
        | ExprKind::MethodCall(_, [first, .., last], _)
        | ExprKind::Binary(_, first, last)
        | ExprKind::Tup([first, .., last])
        | ExprKind::Assign(first, last, _)
        | ExprKind::AssignOp(_, first, last) => (expr_search_pat(tcx, first).0, expr_search_pat(tcx, last).1),
        ExprKind::Tup([e]) | ExprKind::DropTemps(e) => expr_search_pat(tcx, e),
        ExprKind::Cast(e, _) | ExprKind::Type(e, _) => (expr_search_pat(tcx, e).0, Pat::Str("")),
        ExprKind::Let(let_expr) => (Pat::Str("let"), expr_search_pat(tcx, let_expr.init).1),
        ExprKind::If(..) => (Pat::Str("if"), Pat::Str("}")),
        ExprKind::Loop(_, Some(_), _, _) | ExprKind::Block(_, Some(_)) => (Pat::Str("'"), Pat::Str("}")),
        ExprKind::Loop(_, None, LoopSource::Loop, _) => (Pat::Str("loop"), Pat::Str("}")),
        ExprKind::Loop(_, None, LoopSource::While, _) => (Pat::Str("while"), Pat::Str("}")),
        ExprKind::Loop(_, None, LoopSource::ForLoop, _) | ExprKind::Match(_, _, MatchSource::ForLoopDesugar) => {
            (Pat::Str("for"), Pat::Str("}"))
        },
        ExprKind::Match(_, _, MatchSource::Normal) => (Pat::Str("match"), Pat::Str("}")),
        ExprKind::Match(e, _, MatchSource::TryDesugar) => (expr_search_pat(tcx, e).0, Pat::Str("?")),
        ExprKind::Match(e, _, MatchSource::AwaitDesugar) | ExprKind::Yield(e, YieldSource::Await { .. }) => {
            (expr_search_pat(tcx, e).0, Pat::Str("await"))
        },
        ExprKind::Closure(_, _, id, _, _) => (Pat::Str(""), expr_search_pat(tcx, &tcx.hir().body(id).value).1),
        ExprKind::Block(
            Block {
                rules: BlockCheckMode::UnsafeBlock(UnsafeSource::UserProvided),
                ..
            },
            None,
        ) => (Pat::Str("unsafe"), Pat::Str("}")),
        ExprKind::Block(_, None) => (Pat::Str("{"), Pat::Str("}")),
        ExprKind::Field(e, name) => (expr_search_pat(tcx, e).0, Pat::Sym(name.name)),
        ExprKind::Index(e, _) => (expr_search_pat(tcx, e).0, Pat::Str("]")),
        ExprKind::Path(ref path) => qpath_search_pat(path),
        ExprKind::AddrOf(_, _, e) => (Pat::Str("&"), expr_search_pat(tcx, e).1),
        ExprKind::Break(Destination { label: None, .. }, None) => (Pat::Str("break"), Pat::Str("break")),
        ExprKind::Break(Destination { label: Some(name), .. }, None) => (Pat::Str("break"), Pat::Sym(name.ident.name)),
        ExprKind::Break(_, Some(e)) => (Pat::Str("break"), expr_search_pat(tcx, e).1),
        ExprKind::Continue(Destination { label: None, .. }) => (Pat::Str("continue"), Pat::Str("continue")),
        ExprKind::Continue(Destination { label: Some(name), .. }) => (Pat::Str("continue"), Pat::Sym(name.ident.name)),
        ExprKind::Ret(None) => (Pat::Str("return"), Pat::Str("return")),
        ExprKind::Struct(path, _, _) => (qpath_search_pat(path).0, Pat::Str("}")),
        ExprKind::Yield(e, YieldSource::Yield) => (Pat::Str("yield"), expr_search_pat(tcx, e).1),
        _ => (Pat::Str(""), Pat::Str("")),
    }
}

/// Checks if the expression likely came from a proc-macro
pub fn is_expr_from_proc_macro(cx: &LateContext<'_>, e: &Expr<'_>) -> bool {
    let (start_pat, end_pat) = expr_search_pat(cx.tcx, e);
    !span_matches_pat(cx.sess(), e.span, start_pat, end_pat)
}

/// Checks if the span actually refers to a match expression
pub fn is_span_match(cx: &LateContext<'_>, span: Span) -> bool {
    span_matches_pat(cx.sess(), span, Pat::Str("match"), Pat::Str("}"))
}
