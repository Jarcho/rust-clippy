use clippy_utils::diagnostics::span_lint;
use clippy_utils::is_from_proc_macro;
use rustc_lint::{EarlyContext, EarlyLintPass, LateContext, LateLintPass, Level, LintContext};
use rustc_middle::ty::TyCtxt;
use rustc_session::{declare_lint_pass, declare_tool_lint};
use {rustc_ast as ast, rustc_hir as hir};

declare_tool_lint! {
    /// ### What it does
    /// Checks for nodes detected by `is_from_proc_macro`.
    pub clippy::DETECT_PROC_MACROS,
    Allow,
    "test `is_from_proc_macro` detection",
    report_in_external_macro: true
}

declare_lint_pass!(DetectProcMacros => [DETECT_PROC_MACROS]);

impl EarlyLintPass for DetectProcMacros {
    #![allow(clippy::items_after_statements, clippy::wildcard_imports)]
    fn check_crate(&mut self, cx: &EarlyContext<'_>, krate: &ast::Crate) {
        if matches!(cx.get_lint_level(DETECT_PROC_MACROS).level, Level::Allow) {
            return;
        }

        use ast::visit::*;
        struct V<'a, 'cx>(&'a EarlyContext<'cx>);
        impl<'cx> Visitor<'cx> for V<'_, 'cx> {
            fn visit_expr(&mut self, node: &'cx ast::Expr) {
                if !node.span.in_external_macro(self.0.sess().source_map()) && is_from_proc_macro(self.0, node) {
                    span_lint(self.0, DETECT_PROC_MACROS, node.span, "detected proc macro");
                }
                walk_expr(self, node);
            }
            fn visit_pat(&mut self, node: &'cx ast::Pat) {
                if !node.span.in_external_macro(self.0.sess().source_map()) && is_from_proc_macro(self.0, node) {
                    span_lint(self.0, DETECT_PROC_MACROS, node.span, "detected proc macro");
                }
                walk_pat(self, node);
            }
            fn visit_ty(&mut self, node: &'cx ast::Ty) {
                if !node.span.in_external_macro(self.0.sess().source_map()) && is_from_proc_macro(self.0, node) {
                    span_lint(self.0, DETECT_PROC_MACROS, node.span, "detected proc macro");
                }
                walk_ty(self, node);
            }
        }
        V(cx).visit_crate(krate);
    }
}

impl<'tcx> LateLintPass<'tcx> for DetectProcMacros {
    #![allow(clippy::items_after_statements, clippy::wildcard_imports)]
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        if matches!(cx.get_lint_level(DETECT_PROC_MACROS).level, Level::Allow) {
            return;
        }

        use hir::intravisit::*;
        struct V<'a, 'tcx>(&'a LateContext<'tcx>);
        impl<'tcx> Visitor<'tcx> for V<'_, 'tcx> {
            type NestedFilter = rustc_middle::hir::nested_filter::All;

            fn maybe_tcx(&mut self) -> TyCtxt<'tcx> {
                self.0.tcx
            }

            fn visit_expr(&mut self, node: &'tcx hir::Expr<'tcx>) {
                if !node.span.in_external_macro(self.0.tcx.sess.source_map()) && is_from_proc_macro(self.0, node) {
                    span_lint(self.0, DETECT_PROC_MACROS, node.span, "detected proc macro");
                }
                walk_expr(self, node);
            }
            fn visit_field_def(&mut self, node: &'tcx hir::FieldDef<'tcx>) {
                if !node.span.in_external_macro(self.0.tcx.sess.source_map()) && is_from_proc_macro(self.0, node) {
                    span_lint(self.0, DETECT_PROC_MACROS, node.span, "detected proc macro");
                }
                walk_field_def(self, node);
            }
            fn visit_foreign_item(&mut self, node: &'tcx hir::ForeignItem<'tcx>) {
                if !node.span.in_external_macro(self.0.tcx.sess.source_map()) && is_from_proc_macro(self.0, node) {
                    span_lint(self.0, DETECT_PROC_MACROS, node.span, "detected proc macro");
                }
                walk_foreign_item(self, node);
            }
            fn visit_item(&mut self, node: &'tcx hir::Item<'tcx>) {
                if !node.span.in_external_macro(self.0.tcx.sess.source_map()) && is_from_proc_macro(self.0, node) {
                    span_lint(self.0, DETECT_PROC_MACROS, node.span, "detected proc macro");
                }
                walk_item(self, node);
            }
            fn visit_impl_item(&mut self, node: &'tcx hir::ImplItem<'tcx>) {
                if !node.span.in_external_macro(self.0.tcx.sess.source_map()) && is_from_proc_macro(self.0, node) {
                    span_lint(self.0, DETECT_PROC_MACROS, node.span, "detected proc macro");
                }
                walk_impl_item(self, node);
            }
            fn visit_pat(&mut self, node: &'tcx hir::Pat<'tcx>) {
                if !node.span.in_external_macro(self.0.tcx.sess.source_map()) && is_from_proc_macro(self.0, node) {
                    span_lint(self.0, DETECT_PROC_MACROS, node.span, "detected proc macro");
                }
                walk_pat(self, node);
            }
            fn visit_trait_item(&mut self, node: &'tcx hir::TraitItem<'tcx>) {
                if !node.span.in_external_macro(self.0.tcx.sess.source_map()) && is_from_proc_macro(self.0, node) {
                    span_lint(self.0, DETECT_PROC_MACROS, node.span, "detected proc macro");
                }
                walk_trait_item(self, node);
            }
            fn visit_trait_ref(&mut self, node: &'tcx hir::TraitRef<'tcx>) {
                if !node.path.span.in_external_macro(self.0.tcx.sess.source_map()) && is_from_proc_macro(self.0, node) {
                    span_lint(self.0, DETECT_PROC_MACROS, node.path.span, "detected proc macro");
                }
                walk_trait_ref(self, node);
            }
            fn visit_ty(&mut self, node: &'tcx hir::Ty<'tcx, hir::AmbigArg>) {
                if !node.span.in_external_macro(self.0.tcx.sess.source_map())
                    && is_from_proc_macro(self.0, node.as_unambig_ty())
                {
                    span_lint(self.0, DETECT_PROC_MACROS, node.span, "detected proc macro");
                }
                walk_ty(self, node);
            }
            fn visit_variant(&mut self, node: &'tcx hir::Variant<'tcx>) {
                if !node.span.in_external_macro(self.0.tcx.sess.source_map()) && is_from_proc_macro(self.0, node) {
                    span_lint(self.0, DETECT_PROC_MACROS, node.span, "detected proc macro");
                }
                walk_variant(self, node);
            }
        }
        cx.tcx.hir_walk_toplevel_module(&mut V(cx));
    }
}
