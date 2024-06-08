use clippy_utils::diagnostics::span_lint;
use clippy_utils::is_from_proc_macro;
use rustc_data_structures::fx::FxHashSet;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::{DefId, CRATE_DEF_INDEX};
use rustc_hir::{HirId, ItemKind, Node, Path};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::impl_lint_pass;
use rustc_span::symbol::kw;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for usage of items through absolute paths, like `std::env::current_dir`.
    ///
    /// ### Why restrict this?
    /// Many codebases have their own style when it comes to importing, but one that is seldom used
    /// is using absolute paths *everywhere*. This is generally considered unidiomatic, and you
    /// should add a `use` statement.
    ///
    /// The default maximum segments (2) is pretty strict, you may want to increase this in
    /// `clippy.toml`.
    ///
    /// Note: One exception to this is code from macro expansion - this does not lint such cases, as
    /// using absolute paths is the proper way of referencing items in one.
    ///
    /// ### Example
    /// ```no_run
    /// let x = std::f64::consts::PI;
    /// ```
    /// Use any of the below instead, or anything else:
    /// ```no_run
    /// use std::f64;
    /// use std::f64::consts;
    /// use std::f64::consts::PI;
    /// let x = f64::consts::PI;
    /// let x = consts::PI;
    /// let x = PI;
    /// use std::f64::consts as f64_consts;
    /// let x = f64_consts::PI;
    /// ```
    #[clippy::version = "1.73.0"]
    pub ABSOLUTE_PATHS,
    restriction,
    "checks for usage of an item without a `use` statement"
}
impl_lint_pass!(AbsolutePaths => [ABSOLUTE_PATHS]);

pub struct AbsolutePaths {
    pub absolute_paths_max_segments: u64,
    pub absolute_paths_allowed_crates: FxHashSet<String>,
}

impl<'tcx> LateLintPass<'tcx> for AbsolutePaths {
    // We should only lint `QPath::Resolved`s, but since `Path` is only used in `Resolved` and `UsePath`
    // we don't need to use a visitor or anything as we can just check if the `Node` for `hir_id` isn't
    // a `Use`
    #[expect(clippy::cast_possible_truncation)]
    fn check_path(&mut self, cx: &LateContext<'tcx>, path: &Path<'tcx>, hir_id: HirId) {
        let (first, len) = match path.segments {
            [] | [_] => return,
            [root, first, ..] if root.ident.name == kw::PathRoot => (first, path.segments.len() - 1),
            [first, ..] => (first, path.segments.len()),
        };

        if len > self.absolute_paths_max_segments as usize
            && !path.span.from_expansion()
            && let node = cx.tcx.hir_node(hir_id)
            && !matches!(node, Node::Item(item) if matches!(item.kind, ItemKind::Use(..)))
            && let is_abs_crate = first.ident.name == kw::Crate
            && let is_abs_external =
                matches!(first.res, Res::Def(DefKind::Mod, DefId { index, .. }) if index == CRATE_DEF_INDEX)
            && (is_abs_crate || is_abs_external)
            && (!is_abs_crate || !self.absolute_paths_allowed_crates.contains("crate"))
            && (!is_abs_external || !self.absolute_paths_allowed_crates.contains(first.ident.name.as_str()))
            && !is_from_proc_macro(cx, path)
        {
            span_lint(
                cx,
                ABSOLUTE_PATHS,
                path.span,
                "consider bringing this path into scope with the `use` keyword",
            );
        }
    }
}
