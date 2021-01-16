use crate::utils::{is_trait_impl_item, match_def_path, paths, snippet_with_applicability, span_lint_and_then};
use if_chain::if_chain;
use rustc_errors::Applicability;
use rustc_hir::{
    intravisit::FnKind, Block, Body, Expr, ExprKind, FnDecl, FnRetTy, GenericArg, GenericArgs, HirId, PathSegment,
    QPath, TyKind,
};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_middle::lint::in_external_macro;
use rustc_session::{declare_tool_lint, impl_lint_pass};
use rustc_span::{symbol::sym, Span};
use std::{iter, mem};

declare_clippy_lint! {
    /// **What it does:** Checks for functions which return a newly created `Box`..
    ///
    /// **Why is this bad?** This removes flexibility from the caller to decide where to place
    /// the value for zero performance gains.
    ///
    /// **Known problems:** This is an API breaking change, so may not be applicable.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// fn foo() -> Box<u32> {
    ///     Box::new(0)
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// fn foo() -> u32 {
    ///     0
    /// }
    /// ```
    pub RETURN_NEW_BOX,
    style,
    "return of a newly created `Box<T>`"
}

struct Searcher {
    decl_box_span: Span,
    decl_boxed_span: Span,
    return_exprs: Vec<(Span, Span)>,
}

#[derive(Default)]
pub struct ReturnNewBox {
    fn_stack: Vec<Option<Searcher>>,
    current_fn: Option<Searcher>,
}
impl_lint_pass!(ReturnNewBox => [RETURN_NEW_BOX]);

impl LateLintPass<'_> for ReturnNewBox {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        id: HirId,
    ) {
        if_chain! {
            if !in_external_macro(cx.sess(), span);
            if !is_trait_impl_item(cx, id);
            if !matches!(kind, FnKind::Closure(_));
            if let FnRetTy::Return(hir_ty) = decl.output;
            if let TyKind::Path(QPath::Resolved(None, hir_ty_path)) = hir_ty.kind;
            let def_id = cx.tcx.hir().local_def_id(id);
            let ty = cx.tcx.erase_late_bound_regions(cx.tcx.fn_sig(def_id)).output();
            if ty.is_box();
            if let [.., PathSegment { args: Some(GenericArgs { args: [GenericArg::Type(hir_ty_arg), ..], ..}), .. }]
                = hir_ty_path.segments;
            if ty.boxed_ty().is_sized(cx.tcx.at(span), cx.param_env);
            if let ExprKind::Block(block, _) = body.value.kind;
            then {
                let mut searcher = Searcher {
                    decl_box_span: decl.output.span(),
                    decl_boxed_span: hir_ty_arg.span,
                    return_exprs: Vec::new(),
                };

                if let Some(e) = block.expr {
                    if !get_new_box_sugg_spans(cx, e, &mut searcher.return_exprs) {
                        self.push_fn(None);
                        return;
                    }
                };

                self.push_fn(Some(searcher));
            } else {
                self.push_fn(None);
            }
        }
    }

    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if let ExprKind::Ret(Some(ret_expr)) = expr.kind {
            if in_external_macro(cx.sess(), expr.span) {
                self.current_fn = None;
                return;
            }

            if let Some(ref mut searcher) = self.current_fn {
                if !get_new_box_sugg_spans(cx, ret_expr, &mut searcher.return_exprs) {
                    self.current_fn = None;
                }
            }
        }
    }

    fn check_fn_post(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        _: &'tcx Body<'tcx>,
        _: Span,
        _: HirId,
    ) {
        self.pop_fn(cx);
    }
}

impl ReturnNewBox {
    fn push_fn(&mut self, searcher: Option<Searcher>) {
        self.fn_stack.push(mem::replace(&mut self.current_fn, searcher))
    }

    fn pop_fn(&mut self, cx: &LateContext<'_>) {
        if let Some(searcher) = mem::replace(&mut self.current_fn, self.fn_stack.pop().unwrap()) {
            span_lint_and_then(
                cx,
                RETURN_NEW_BOX,
                searcher.decl_box_span,
                "return of a new `Box`",
                |diag| {
                    let mut app = Applicability::MachineApplicable;
                    let sugg = iter::once((searcher.decl_box_span, searcher.decl_boxed_span))
                        .chain(searcher.return_exprs.into_iter())
                        .map(|(old_span, new_span)| {
                            (
                                old_span,
                                snippet_with_applicability(cx, new_span, "_", &mut app).into_owned(),
                            )
                        })
                        .collect();
                    diag.multipart_suggestion("use the boxed type instead", sugg, app);
                },
            );
        }
    }
}

fn get_new_box_sugg_spans(cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>, spans: &mut Vec<(Span, Span)>) -> bool {
    if cx
        .typeck_results()
        .expr_ty(expr)
        .conservative_is_privately_uninhabited(cx.tcx)
    {
        return true;
    }

    match expr.kind {
        ExprKind::Call(func, [arg]) => match func.kind {
            ExprKind::Path(QPath::TypeRelative(ty, name))
                if cx.typeck_results().node_type(ty.hir_id).is_box()
                    && (name.ident.name == sym::new || name.ident.name == sym::from) =>
            {
                spans.push((expr.span, arg.span));
                true
            }
            ExprKind::Path(QPath::Resolved(_, path))
                if path.res.opt_def_id().map_or(false, |id| {
                    match_def_path(cx, id, &paths::FROM_FROM) || match_def_path(cx, id, &paths::INTO_INTO)
                }) =>
            {
                spans.push((expr.span, arg.span));
                true
            }
            _ => false,
        },
        ExprKind::MethodCall(name, _, [self_arg], _)
            if name.ident.as_str() == "into"
                && cx
                    .typeck_results()
                    .type_dependent_def_id(expr.hir_id)
                    .map_or(false, |id| match_def_path(cx, id, &paths::INTO_INTO)) =>
        {
            spans.push((expr.span, self_arg.span));
            true
        }
        ExprKind::Block(Block { expr: None, .. }, _) => true,
        ExprKind::Block(Block { expr: Some(expr), .. }, _) => get_new_box_sugg_spans(cx, expr, spans),
        ExprKind::If(_, expr1, Some(expr2)) => {
            get_new_box_sugg_spans(cx, expr1, spans) && get_new_box_sugg_spans(cx, expr2, spans)
        },
        ExprKind::Match(_, arms, _) => arms.iter().all(|arm| get_new_box_sugg_spans(cx, arm.body, spans)),
        _ => false,
    }
}
