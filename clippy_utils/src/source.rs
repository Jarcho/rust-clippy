//! Utils for extracting, inspecting or transforming source code

#![allow(clippy::module_name_repetitions)]

use crate::{get_parent_expr, line_span};
use rustc_errors::Applicability;
use rustc_hir::{BinOpKind, Expr, ExprKind, MatchSource};
use rustc_lint::{LateContext, LintContext};
use rustc_span::hygiene;
use rustc_span::{BytePos, Pos, Span, SyntaxContext};
use std::borrow::Cow;

/// Like `snippet_block`, but add braces if the expr is not an `ExprKind::Block`.
/// Also takes an `Option<String>` which can be put inside the braces.
pub fn expr_block<'a, T: LintContext>(
    cx: &T,
    expr: &Expr<'_>,
    option: Option<String>,
    default: &'a str,
    indent_relative_to: Option<Span>,
) -> Cow<'a, str> {
    let code = snippet_block(cx, expr.span, default, indent_relative_to);
    let string = option.unwrap_or_default();
    if expr.span.from_expansion() {
        Cow::Owned(format!("{{ {} }}", snippet_with_macro_callsite(cx, expr.span, default)))
    } else if let ExprKind::Block(_, _) = expr.kind {
        Cow::Owned(format!("{}{}", code, string))
    } else if string.is_empty() {
        Cow::Owned(format!("{{ {} }}", code))
    } else {
        Cow::Owned(format!("{{\n{};\n{}\n}}", code, string))
    }
}

/// Returns a new Span that extends the original Span to the first non-whitespace char of the first
/// line.
///
/// ```rust,ignore
///     let x = ();
/// //          ^^
/// // will be converted to
///     let x = ();
/// //  ^^^^^^^^^^
/// ```
pub fn first_line_of_span<T: LintContext>(cx: &T, span: Span) -> Span {
    first_char_in_first_line(cx, span).map_or(span, |first_char_pos| span.with_lo(first_char_pos))
}

fn first_char_in_first_line<T: LintContext>(cx: &T, span: Span) -> Option<BytePos> {
    let line_span = line_span(cx, span);
    snippet_opt(cx, line_span).and_then(|snip| {
        snip.find(|c: char| !c.is_whitespace())
            .map(|pos| line_span.lo() + BytePos::from_usize(pos))
    })
}

/// Returns the indentation of the line of a span
///
/// ```rust,ignore
/// let x = ();
/// //      ^^ -- will return 0
///     let x = ();
/// //          ^^ -- will return 4
/// ```
pub fn indent_of<T: LintContext>(cx: &T, span: Span) -> Option<usize> {
    snippet_opt(cx, line_span(cx, span)).and_then(|snip| snip.find(|c: char| !c.is_whitespace()))
}

/// Gets a snippet of the indentation of the line of a span
pub fn snippet_indent<T: LintContext>(cx: &T, span: Span) -> Option<String> {
    snippet_opt(cx, line_span(cx, span)).map(|mut s| {
        let len = s.len() - s.trim_start().len();
        s.truncate(len);
        s
    })
}

// If the snippet is empty, it's an attribute that was inserted during macro
// expansion and we want to ignore those, because they could come from external
// sources that the user has no control over.
// For some reason these attributes don't have any expansion info on them, so
// we have to check it this way until there is a better way.
pub fn is_present_in_source<T: LintContext>(cx: &T, span: Span) -> bool {
    if let Some(snippet) = snippet_opt(cx, span) {
        if snippet.is_empty() {
            return false;
        }
    }
    true
}

/// Returns the positon just before rarrow
///
/// ```rust,ignore
/// fn into(self) -> () {}
///              ^
/// // in case of unformatted code
/// fn into2(self)-> () {}
///               ^
/// fn into3(self)   -> () {}
///               ^
/// ```
pub fn position_before_rarrow(s: &str) -> Option<usize> {
    s.rfind("->").map(|rpos| {
        let mut rpos = rpos;
        let chars: Vec<char> = s.chars().collect();
        while rpos > 1 {
            if let Some(c) = chars.get(rpos - 1) {
                if c.is_whitespace() {
                    rpos -= 1;
                    continue;
                }
            }
            break;
        }
        rpos
    })
}

/// Reindent a multiline string with possibility of ignoring the first line.
#[allow(clippy::needless_pass_by_value)]
pub fn reindent_multiline(s: Cow<'_, str>, ignore_first: bool, indent: Option<usize>) -> Cow<'_, str> {
    let s_space = reindent_multiline_inner(&s, ignore_first, indent, ' ');
    let s_tab = reindent_multiline_inner(&s_space, ignore_first, indent, '\t');
    reindent_multiline_inner(&s_tab, ignore_first, indent, ' ').into()
}

fn reindent_multiline_inner(s: &str, ignore_first: bool, indent: Option<usize>, ch: char) -> String {
    let x = s
        .lines()
        .skip(usize::from(ignore_first))
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                // ignore empty lines
                Some(l.char_indices().find(|&(_, x)| x != ch).unwrap_or((l.len(), ch)).0)
            }
        })
        .min()
        .unwrap_or(0);
    let indent = indent.unwrap_or(0);
    s.lines()
        .enumerate()
        .map(|(i, l)| {
            if (ignore_first && i == 0) || l.is_empty() {
                l.to_owned()
            } else if x > indent {
                l.split_at(x - indent).1.to_owned()
            } else {
                " ".repeat(indent - x) + l
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// Converts a span to a code snippet if available, otherwise returns the default.
///
/// This is useful if you want to provide suggestions for your lint or more generally, if you want
/// to convert a given `Span` to a `str`. To create suggestions consider using
/// [`snippet_with_applicability`] to ensure that the applicability stays correct.
///
/// # Example
/// ```rust,ignore
/// // Given two spans one for `value` and one for the `init` expression.
/// let value = Vec::new();
/// //  ^^^^^   ^^^^^^^^^^
/// //  span1   span2
///
/// // The snipped call would return the corresponding code snippet
/// snippet(cx, span1, "..") // -> "value"
/// snippet(cx, span2, "..") // -> "Vec::new()"
/// ```
pub fn snippet<'a, T: LintContext>(cx: &T, span: Span, default: &'a str) -> Cow<'a, str> {
    snippet_opt(cx, span).map_or_else(|| Cow::Borrowed(default), From::from)
}

/// Same as [`snippet`], but it adapts the applicability level by following rules:
///
/// - Applicability level `Unspecified` will never be changed.
/// - If the span is inside a macro, change the applicability level to `MaybeIncorrect`.
/// - If the default value is used and the applicability level is `MachineApplicable`, change it to
/// `HasPlaceholders`
pub fn snippet_with_applicability<'a, T: LintContext>(
    cx: &T,
    span: Span,
    default: &'a str,
    applicability: &mut Applicability,
) -> Cow<'a, str> {
    if *applicability != Applicability::Unspecified && span.from_expansion() {
        *applicability = Applicability::MaybeIncorrect;
    }
    snippet_opt(cx, span).map_or_else(
        || {
            if *applicability == Applicability::MachineApplicable {
                *applicability = Applicability::HasPlaceholders;
            }
            Cow::Borrowed(default)
        },
        From::from,
    )
}

/// Same as `snippet`, but should only be used when it's clear that the input span is
/// not a macro argument.
pub fn snippet_with_macro_callsite<'a, T: LintContext>(cx: &T, span: Span, default: &'a str) -> Cow<'a, str> {
    snippet(cx, span.source_callsite(), default)
}

/// Converts a span to a code snippet. Returns `None` if not available.
pub fn snippet_opt<T: LintContext>(cx: &T, span: Span) -> Option<String> {
    cx.sess().source_map().span_to_snippet(span).ok()
}

/// Converts a span (from a block) to a code snippet if available, otherwise use default.
///
/// This trims the code of indentation, except for the first line. Use it for blocks or block-like
/// things which need to be printed as such.
///
/// The `indent_relative_to` arg can be used, to provide a span, where the indentation of the
/// resulting snippet of the given span.
///
/// # Example
///
/// ```rust,ignore
/// snippet_block(cx, block.span, "..", None)
/// // where, `block` is the block of the if expr
///     if x {
///         y;
///     }
/// // will return the snippet
/// {
///     y;
/// }
/// ```
///
/// ```rust,ignore
/// snippet_block(cx, block.span, "..", Some(if_expr.span))
/// // where, `block` is the block of the if expr
///     if x {
///         y;
///     }
/// // will return the snippet
/// {
///         y;
///     } // aligned with `if`
/// ```
/// Note that the first line of the snippet always has 0 indentation.
pub fn snippet_block<'a, T: LintContext>(
    cx: &T,
    span: Span,
    default: &'a str,
    indent_relative_to: Option<Span>,
) -> Cow<'a, str> {
    let snip = snippet(cx, span, default);
    let indent = indent_relative_to.and_then(|s| indent_of(cx, s));
    reindent_multiline(snip, true, indent)
}

/// Same as `snippet_block`, but adapts the applicability level by the rules of
/// `snippet_with_applicability`.
pub fn snippet_block_with_applicability<'a, T: LintContext>(
    cx: &T,
    span: Span,
    default: &'a str,
    indent_relative_to: Option<Span>,
    applicability: &mut Applicability,
) -> Cow<'a, str> {
    let snip = snippet_with_applicability(cx, span, default, applicability);
    let indent = indent_relative_to.and_then(|s| indent_of(cx, s));
    reindent_multiline(snip, true, indent)
}

/// Same as `snippet_with_applicability`, but first walks the span up to the given context. This
/// will result in the macro call, rather then the expansion, if the span is from a child context.
/// If the span is not from a child context, it will be used directly instead.
///
/// e.g. Given the expression `&vec![]`, getting a snippet from the span for `vec![]` as a HIR node
/// would result in `box []`. If given the context of the address of expression, this function will
/// correctly get a snippet of `vec![]`.
///
/// This will also return whether or not the snippet is a macro call.
pub fn snippet_with_context(
    cx: &LateContext<'_>,
    span: Span,
    outer: SyntaxContext,
    default: &'a str,
    applicability: &mut Applicability,
) -> (Cow<'a, str>, bool) {
    let (span, is_macro_call) = walk_span_to_context(span, outer).map_or_else(
        || {
            // The span is from a macro argument, and the outer context is the macro using the argument
            if *applicability != Applicability::Unspecified {
                *applicability = Applicability::MaybeIncorrect;
            }
            // TODO: get the argument span.
            (span, false)
        },
        |outer_span| (outer_span, span.ctxt() != outer),
    );

    (
        snippet_with_applicability(cx, span, default, applicability),
        is_macro_call,
    )
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExprPosition {
    // Also includes `return`, `yield`, `break` and closures
    Paren,
    AssignmentRhs,
    AssignmentLhs,
    RangeLhs,
    RangeRhs,
    OrLhs,
    OrRhs,
    AndLhs,
    AndRhs,
    Let,
    EqLhs,
    EqRhs,
    BitOrLhs,
    BitOrRhs,
    BitXorLhs,
    BitXorRhs,
    BitAndLhs,
    BitAndRhs,
    ShiftLhs,
    ShiftRhs,
    AddLhs,
    AddRhs,
    MulLhs,
    MulRhs,
    // Also includes type ascription
    Cast,
    Prefix,
    Postfix,
}

/// Extracts a snippet of the given expression taking into account the `SyntaxContext` the snippet
/// needs to be taken from. Parenthesis will be added if needed to place the snippet in the target
/// precedence level. Returns a placeholder (`(..)`) if a snippet can't be extracted (e.g. an
/// invalid span).
///
/// The `SyntaxContext` of the expression will be walked up to the given target context (usually
/// from the parent expression) before extracting a snippet. This allows getting the call to a macro
/// rather than the expression from expanding the macro. e.g. In the expression `&vec![]` taking a
/// snippet of the chile of the borrow expression will get a snippet of what `vec![]` expands in to.
/// With the target context set to the same as the borrow expression, this will get a snippet of the
/// call to the macro.
///
/// The applicability will be modified in two ways:
/// * If a snippet can't be extracted it will be changed from `MachineApplicable` or
///   `MaybeIncorrect` to `HasPlaceholders`.
/// * If the snippet is taken from a macro expansion then it will be changed from
///   `MachineApplicable` to `MaybeIncorrect`.
pub fn snippet_expr(
    cx: &LateContext<'_>,
    expr: &Expr<'_>,
    target_position: ExprPosition,
    ctxt: SyntaxContext,
    app: &mut Applicability,
) -> String {
    let (snip, is_mac_call) = snippet_with_context(cx, expr.span, ctxt, "(..)", app);

    match snip {
        Cow::Borrowed(snip) => snip.to_owned(),
        Cow::Owned(snip) if is_mac_call => snip,
        Cow::Owned(mut snip) => {
            let ctxt = expr.span.ctxt();

            // Attempt to determine if parenthesis are needed base on the target position. The snippet may have
            // parenthesis already, so attempt to find those.
            // TODO: Remove parenthesis if they aren't needed at the target position.
            let needs_paren = match expr.peel_drop_temps().kind {
                ExprKind::Binary(_, lhs, rhs)
                    if (ctxt == lhs.span.ctxt() && expr.span.lo() != lhs.span.lo())
                        || (ctxt == rhs.span.ctxt() && expr.span.hi() != rhs.span.hi()) =>
                {
                    false
                },
                ExprKind::Binary(op, ..) => match op.node {
                    BinOpKind::Add | BinOpKind::Sub => target_position > ExprPosition::AddLhs,
                    BinOpKind::Mul | BinOpKind::Div | BinOpKind::Rem => target_position > ExprPosition::MulLhs,
                    BinOpKind::And => target_position > ExprPosition::AndLhs,
                    BinOpKind::Or => target_position > ExprPosition::OrLhs,
                    BinOpKind::BitXor => target_position > ExprPosition::BitXorLhs,
                    BinOpKind::BitAnd => target_position > ExprPosition::BitAndLhs,
                    BinOpKind::BitOr => target_position > ExprPosition::BitOrLhs,
                    BinOpKind::Shl | BinOpKind::Shr => target_position > ExprPosition::ShiftLhs,
                    BinOpKind::Eq | BinOpKind::Lt | BinOpKind::Le | BinOpKind::Ne | BinOpKind::Gt | BinOpKind::Ge => {
                        target_position > ExprPosition::EqLhs
                    },
                },
                ExprKind::Box(..) | ExprKind::Unary(..) | ExprKind::AddrOf(..) if snip.starts_with('(') => false,
                ExprKind::Box(..) | ExprKind::Unary(..) | ExprKind::AddrOf(..) => {
                    target_position > ExprPosition::Prefix
                },
                ExprKind::Let(..) if snip.starts_with('(') => false,
                ExprKind::Let(..) => target_position > ExprPosition::Let,
                ExprKind::Cast(lhs, rhs)
                    if (ctxt == lhs.span.ctxt() && expr.span.lo() != lhs.span.lo())
                        || (ctxt == rhs.span.ctxt() && expr.span.hi() != rhs.span.hi()) =>
                {
                    false
                },
                ExprKind::Cast(..) | ExprKind::Type(..) => target_position > ExprPosition::Cast,

                ExprKind::Closure(..)
                | ExprKind::Break(..)
                | ExprKind::Ret(..)
                | ExprKind::Yield(..)
                | ExprKind::Assign(..)
                | ExprKind::AssignOp(..) => target_position > ExprPosition::AssignmentRhs,

                // Postfix operators, or expression with braces of some form
                ExprKind::Array(_)
                | ExprKind::Call(..)
                | ExprKind::ConstBlock(_)
                | ExprKind::MethodCall(..)
                | ExprKind::Tup(..)
                | ExprKind::Lit(..)
                | ExprKind::DropTemps(_)
                | ExprKind::If(..)
                | ExprKind::Loop(..)
                | ExprKind::Match(..)
                | ExprKind::Block(..)
                | ExprKind::Field(..)
                | ExprKind::Index(..)
                | ExprKind::Path(_)
                | ExprKind::Continue(_)
                | ExprKind::InlineAsm(_)
                | ExprKind::LlvmInlineAsm(_)
                | ExprKind::Struct(..)
                | ExprKind::Repeat(..)
                | ExprKind::Err => false,
            };

            if needs_paren {
                snip.insert(0, '(');
                snip.push(')');
            }
            snip
        },
    }
}

/// Gets which position the expression is in relative to it's parent. Defaults to `Paren` if the
/// parent node is not an expression.
pub fn position_of_expr(cx: &LateContext<'_>, expr: &Expr<'_>) -> ExprPosition {
    match get_parent_expr(cx, expr) {
        None => ExprPosition::Paren,
        Some(parent) => match parent.kind {
            ExprKind::DropTemps(_) => position_of_expr(cx, parent),
            ExprKind::Binary(op, lhs, _) => match (op.node, expr.hir_id == lhs.hir_id) {
                (BinOpKind::Add | BinOpKind::Sub, true) => ExprPosition::AddLhs,
                (BinOpKind::Add | BinOpKind::Sub, false) => ExprPosition::AddRhs,
                (BinOpKind::Mul | BinOpKind::Div | BinOpKind::Rem, true) => ExprPosition::MulLhs,
                (BinOpKind::Mul | BinOpKind::Div | BinOpKind::Rem, false) => ExprPosition::MulRhs,
                (BinOpKind::And, true) => ExprPosition::AndLhs,
                (BinOpKind::And, false) => ExprPosition::AndRhs,
                (BinOpKind::Or, true) => ExprPosition::OrLhs,
                (BinOpKind::Or, false) => ExprPosition::OrRhs,
                (BinOpKind::BitXor, true) => ExprPosition::BitXorLhs,
                (BinOpKind::BitXor, false) => ExprPosition::BitXorRhs,
                (BinOpKind::BitAnd, true) => ExprPosition::BitAndLhs,
                (BinOpKind::BitAnd, false) => ExprPosition::BitAndRhs,
                (BinOpKind::BitOr, true) => ExprPosition::BitOrLhs,
                (BinOpKind::BitOr, false) => ExprPosition::BitOrRhs,
                (BinOpKind::Shl | BinOpKind::Shr, true) => ExprPosition::ShiftLhs,
                (BinOpKind::Shl | BinOpKind::Shr, false) => ExprPosition::ShiftRhs,
                (
                    BinOpKind::Eq | BinOpKind::Lt | BinOpKind::Le | BinOpKind::Ne | BinOpKind::Gt | BinOpKind::Ge,
                    true,
                ) => ExprPosition::EqLhs,
                (
                    BinOpKind::Eq | BinOpKind::Lt | BinOpKind::Le | BinOpKind::Ne | BinOpKind::Gt | BinOpKind::Ge,
                    false,
                ) => ExprPosition::EqRhs,
            },
            ExprKind::Unary(..) | ExprKind::AddrOf(..) => ExprPosition::Prefix,
            ExprKind::Cast(..) | ExprKind::Type(..) => ExprPosition::Cast,
            ExprKind::Assign(lhs, ..) | ExprKind::AssignOp(_, lhs, _) if lhs.hir_id == expr.hir_id => {
                ExprPosition::AssignmentLhs
            },
            ExprKind::Assign(..) | ExprKind::AssignOp(..) => ExprPosition::AssignmentRhs,
            ExprKind::Call(e, _) | ExprKind::MethodCall(_, _, [e, ..], _) | ExprKind::Index(e, _)
                if expr.hir_id == e.hir_id =>
            {
                ExprPosition::Postfix
            },
            ExprKind::Match(_, _, MatchSource::TryDesugar | MatchSource::AwaitDesugar) | ExprKind::Field(..) => {
                ExprPosition::Postfix
            },
            _ => ExprPosition::Paren,
        },
    }
}

/// Walks the span up to the target context, thereby returning the macro call site if the span is
/// inside a macro expansion, or the original span if it is not. Note this will return `None` in the
/// case of the span being in a macro expansion, but the target context is from expanding a macro
/// argument.
///
/// Given the following
///
/// ```rust,ignore
/// macro_rules! m { ($e:expr) => { f($e) }; }
/// g(m!(0))
/// ```
///
/// If called with a span of the call to `f` and a context of the call to `g` this will return a
/// span containing `m!(0)`. However, if called with a span of the literal `0` this will give a span
/// containing `0` as the context is the same as the outer context.
///
/// This will traverse through multiple macro calls. Given the following:
///
/// ```rust,ignore
/// macro_rules! m { ($e:expr) => { n!($e, 0) }; }
/// macro_rules! n { ($e:expr, $f:expr) => { f($e, $f) }; }
/// g(m!(0))
/// ```
///
/// If called with a span of the call to `f` and a context of the call to `g` this will return a
/// span containing `m!(0)`.
pub fn walk_span_to_context(span: Span, outer: SyntaxContext) -> Option<Span> {
    let outer_span = hygiene::walk_chain(span, outer);
    (outer_span.ctxt() == outer).then(|| outer_span)
}

/// Removes block comments from the given `Vec` of lines.
///
/// # Examples
///
/// ```rust,ignore
/// without_block_comments(vec!["/*", "foo", "*/"]);
/// // => vec![]
///
/// without_block_comments(vec!["bar", "/*", "foo", "*/"]);
/// // => vec!["bar"]
/// ```
pub fn without_block_comments(lines: Vec<&str>) -> Vec<&str> {
    let mut without = vec![];

    let mut nest_level = 0;

    for line in lines {
        if line.contains("/*") {
            nest_level += 1;
            continue;
        } else if line.contains("*/") {
            nest_level -= 1;
            continue;
        }

        if nest_level == 0 {
            without.push(line);
        }
    }

    without
}

#[cfg(test)]
mod test {
    use super::{reindent_multiline, without_block_comments};

    #[test]
    fn test_reindent_multiline_single_line() {
        assert_eq!("", reindent_multiline("".into(), false, None));
        assert_eq!("...", reindent_multiline("...".into(), false, None));
        assert_eq!("...", reindent_multiline("    ...".into(), false, None));
        assert_eq!("...", reindent_multiline("\t...".into(), false, None));
        assert_eq!("...", reindent_multiline("\t\t...".into(), false, None));
    }

    #[test]
    #[rustfmt::skip]
    fn test_reindent_multiline_block() {
        assert_eq!("\
    if x {
        y
    } else {
        z
    }", reindent_multiline("    if x {
            y
        } else {
            z
        }".into(), false, None));
        assert_eq!("\
    if x {
    \ty
    } else {
    \tz
    }", reindent_multiline("    if x {
        \ty
        } else {
        \tz
        }".into(), false, None));
    }

    #[test]
    #[rustfmt::skip]
    fn test_reindent_multiline_empty_line() {
        assert_eq!("\
    if x {
        y

    } else {
        z
    }", reindent_multiline("    if x {
            y

        } else {
            z
        }".into(), false, None));
    }

    #[test]
    #[rustfmt::skip]
    fn test_reindent_multiline_lines_deeper() {
        assert_eq!("\
        if x {
            y
        } else {
            z
        }", reindent_multiline("\
    if x {
        y
    } else {
        z
    }".into(), true, Some(8)));
    }

    #[test]
    fn test_without_block_comments_lines_without_block_comments() {
        let result = without_block_comments(vec!["/*", "", "*/"]);
        println!("result: {:?}", result);
        assert!(result.is_empty());

        let result = without_block_comments(vec!["", "/*", "", "*/", "#[crate_type = \"lib\"]", "/*", "", "*/", ""]);
        assert_eq!(result, vec!["", "#[crate_type = \"lib\"]", ""]);

        let result = without_block_comments(vec!["/* rust", "", "*/"]);
        assert!(result.is_empty());

        let result = without_block_comments(vec!["/* one-line comment */"]);
        assert!(result.is_empty());

        let result = without_block_comments(vec!["/* nested", "/* multi-line", "comment", "*/", "test", "*/"]);
        assert!(result.is_empty());

        let result = without_block_comments(vec!["/* nested /* inline /* comment */ test */ */"]);
        assert!(result.is_empty());

        let result = without_block_comments(vec!["foo", "bar", "baz"]);
        assert_eq!(result, vec!["foo", "bar", "baz"]);
    }
}
