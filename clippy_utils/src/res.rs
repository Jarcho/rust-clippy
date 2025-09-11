use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::DefId;
use rustc_hir::{
    self as hir, Expr, ExprKind, HirId, LangItem, Pat, PatExpr, PatExprKind, PatKind, Path, QPath, TyKind,
};
use rustc_lint::LateContext;
use rustc_middle::ty::layout::HasTyCtxt;
use rustc_middle::ty::{self, AdtDef, Binder, EarlyBinder, Ty, TypeckResults};
use rustc_span::{Ident, Symbol};

/// A `QPath` with the `HirId` of the node containing it.
type QPathId<'tcx> = (&'tcx QPath<'tcx>, HirId);

/// A HIR node which might be a `QPath`.
pub trait MaybeQPath<'tcx> {
    /// If this node is a path gets both the contained path and the `HirId` to
    /// use for type dependant lookup.
    fn opt_qpath(self) -> Option<QPathId<'tcx>>;
}

impl<'tcx> MaybeQPath<'tcx> for QPathId<'tcx> {
    #[inline]
    fn opt_qpath(self) -> Option<QPathId<'tcx>> {
        Some((self.0, self.1))
    }
}
impl<'tcx> MaybeQPath<'tcx> for &'tcx Expr<'_> {
    #[inline]
    fn opt_qpath(self) -> Option<QPathId<'tcx>> {
        match &self.kind {
            ExprKind::Path(qpath) => Some((qpath, self.hir_id)),
            _ => None,
        }
    }
}
impl<'tcx> MaybeQPath<'tcx> for &'tcx PatExpr<'_> {
    #[inline]
    fn opt_qpath(self) -> Option<QPathId<'tcx>> {
        match &self.kind {
            PatExprKind::Path(qpath) => Some((qpath, self.hir_id)),
            _ => None,
        }
    }
}
impl<'tcx, AmbigArg> MaybeQPath<'tcx> for &'tcx hir::Ty<'_, AmbigArg> {
    #[inline]
    fn opt_qpath(self) -> Option<QPathId<'tcx>> {
        match &self.kind {
            TyKind::Path(qpath) => Some((qpath, self.hir_id)),
            _ => None,
        }
    }
}
impl<'tcx> MaybeQPath<'tcx> for &'_ Pat<'tcx> {
    #[inline]
    fn opt_qpath(self) -> Option<QPathId<'tcx>> {
        match self.kind {
            PatKind::Expr(e) => e.opt_qpath(),
            _ => None,
        }
    }
}
impl<'tcx, T: MaybeQPath<'tcx>> MaybeQPath<'tcx> for Option<T> {
    #[inline]
    fn opt_qpath(self) -> Option<QPathId<'tcx>> {
        self.and_then(T::opt_qpath)
    }
}
impl<'tcx, T: Copy + MaybeQPath<'tcx>> MaybeQPath<'tcx> for &'_ T {
    #[inline]
    fn opt_qpath(self) -> Option<QPathId<'tcx>> {
        T::opt_qpath(*self)
    }
}

/// A resolved path and the explicit `Self` type if there is one.
type OptResPath<'tcx> = (Option<&'tcx hir::Ty<'tcx>>, Option<&'tcx Path<'tcx>>);

/// A HIR node which might be a `QPath::Resolved`.
///
/// The following are resolved paths:
/// * A path to a module or crate item.
/// * A path to a trait item via the trait's name.
/// * A path to a struct or variant constructor via the original type's path.
/// * A local.
///
/// All other paths are `TypeRelative` and require using `PathRes` to lookup the
/// resolution.
pub trait MaybeResPath {
    /// If this node is a resolved path gets both the contained path and the
    /// type associated with it.
    fn opt_path(&self) -> OptResPath<'_>;

    /// If this node is a resolved path without an associated type gets contained
    /// path.
    fn opt_typeless_path(&self) -> Option<&Path<'_>> {
        match self.opt_path() {
            (
                Some(hir::Ty {
                    kind: TyKind::Infer(()),
                    ..
                })
                | None,
                p,
            ) => p,
            _ => None,
        }
    }

    /// If this node is a resolved path gets it's resolution. Returns `Res::Err`
    /// otherwise.
    #[inline]
    fn simple_res(&self) -> &Res {
        self.opt_path().1.map_or(&Res::Err, |p| &p.res)
    }

    /// If this node is a resolved path without an associated type gets the
    /// path's resolution. Returns `Res::Err` otherwise.
    #[inline]
    fn typeless_res(&self) -> &Res {
        self.opt_typeless_path().map_or(&Res::Err, |p| &p.res)
    }

    /// If this node is a path to a local gets the local's `HirId`.
    #[inline]
    fn path_local_id(&self) -> Option<HirId> {
        if let (_, Some(p)) = self.opt_path()
            && let Res::Local(id) = p.res
        {
            Some(id)
        } else {
            None
        }
    }

    /// If this node is a path to a local gets the local's `HirId` and identifier.
    fn path_local_id_and_ident(&self) -> Option<(HirId, &Ident)> {
        if let (_, Some(p)) = self.opt_path()
            && let Res::Local(id) = p.res
            && let [seg] = p.segments
        {
            Some((id, &seg.ident))
        } else {
            None
        }
    }

    /// Checks whether this node is a path that resolves to the specified local.
    #[inline]
    fn is_path_local(&self, id: HirId) -> bool {
        self.path_local_id() == Some(id)
    }
}
impl MaybeResPath for Path<'_> {
    #[inline]
    fn opt_path(&self) -> OptResPath<'_> {
        (None, Some(self))
    }

    #[inline]
    fn opt_typeless_path(&self) -> Option<&Path<'_>> {
        Some(self)
    }

    #[inline]
    fn simple_res(&self) -> &Res {
        &self.res
    }

    #[inline]
    fn typeless_res(&self) -> &Res {
        &self.res
    }
}
impl MaybeResPath for QPath<'_> {
    #[inline]
    fn opt_path(&self) -> OptResPath<'_> {
        match *self {
            Self::Resolved(ty, path) => (ty, Some(path)),
            _ => (None, None),
        }
    }
}
impl MaybeResPath for Expr<'_> {
    #[inline]
    fn opt_path(&self) -> OptResPath<'_> {
        match &self.kind {
            ExprKind::Path(qpath) => qpath.opt_path(),
            _ => (None, None),
        }
    }
}
impl MaybeResPath for PatExpr<'_> {
    #[inline]
    fn opt_path(&self) -> OptResPath<'_> {
        match &self.kind {
            PatExprKind::Path(qpath) => qpath.opt_path(),
            _ => (None, None),
        }
    }
}
impl<AmbigArg> MaybeResPath for hir::Ty<'_, AmbigArg> {
    #[inline]
    fn opt_path(&self) -> OptResPath<'_> {
        match &self.kind {
            TyKind::Path(qpath) => qpath.opt_path(),
            _ => (None, None),
        }
    }
}
impl MaybeResPath for Pat<'_> {
    #[inline]
    fn opt_path(&self) -> OptResPath<'_> {
        match self.kind {
            PatKind::Expr(e) => e.opt_path(),
            _ => (None, None),
        }
    }
}
impl<T: MaybeResPath> MaybeResPath for Option<T> {
    #[inline]
    fn opt_path(&self) -> OptResPath<'_> {
        match self {
            Some(x) => T::opt_path(x),
            None => (None, None),
        }
    }

    #[inline]
    fn opt_typeless_path(&self) -> Option<&Path<'_>> {
        self.as_ref().and_then(|p| T::opt_typeless_path(p))
    }

    #[inline]
    fn simple_res(&self) -> &Res {
        self.as_ref().map_or(&Res::Err, T::simple_res)
    }

    #[inline]
    fn typeless_res(&self) -> &Res {
        self.as_ref().map_or(&Res::Err, T::typeless_res)
    }
}
impl<T: MaybeResPath> MaybeResPath for &'_ T {
    #[inline]
    fn opt_path(&self) -> OptResPath<'_> {
        T::opt_path(*self)
    }
}

/// A type which may either contain a `DefId` or be referred to by a `DefId`.
pub trait MaybeDefId {
    fn opt_def_id(self) -> Option<DefId>;
}
impl MaybeDefId for DefId {
    #[inline]
    fn opt_def_id(self) -> Option<DefId> {
        Some(self)
    }
}
impl MaybeDefId for (DefKind, DefId) {
    #[inline]
    fn opt_def_id(self) -> Option<DefId> {
        Some(self.1)
    }
}
impl MaybeDefId for AdtDef<'_> {
    #[inline]
    fn opt_def_id(self) -> Option<DefId> {
        Some(self.did())
    }
}
impl MaybeDefId for Ty<'_> {
    #[inline]
    fn opt_def_id(self) -> Option<DefId> {
        self.ty_adt_def().opt_def_id()
    }
}
impl MaybeDefId for Res {
    #[inline]
    fn opt_def_id(self) -> Option<DefId> {
        Res::opt_def_id(&self)
    }
}
impl<T: Copy + MaybeDefId> MaybeDefId for &'_ T {
    #[inline]
    fn opt_def_id(self) -> Option<DefId> {
        T::opt_def_id(*self)
    }
}
impl<T: MaybeDefId> MaybeDefId for Option<T> {
    #[inline]
    fn opt_def_id(self) -> Option<DefId> {
        self.and_then(T::opt_def_id)
    }
}
impl<T: MaybeDefId> MaybeDefId for EarlyBinder<'_, T> {
    #[inline]
    fn opt_def_id(self) -> Option<DefId> {
        self.skip_binder().opt_def_id()
    }
}
impl<T: MaybeDefId> MaybeDefId for Binder<'_, T> {
    #[inline]
    fn opt_def_id(self) -> Option<DefId> {
        self.skip_binder().opt_def_id()
    }
}

/// A type which may contain both a `DefKind` and a `DefId`.
pub trait MaybeDef: Sized {
    fn opt_def(self) -> Option<(DefKind, DefId)>;

    /// Gets this definition as a resolution. Returns `Res::Err` if this is `None`.
    #[inline]
    fn to_res(self) -> Res {
        self.opt_def().map_or(Res::Err, |(kind, id)| Res::Def(kind, id))
    }
}
impl MaybeDef for (DefKind, DefId) {
    #[inline]
    fn opt_def(self) -> Option<(DefKind, DefId)> {
        Some(self)
    }
}
impl MaybeDef for Res {
    #[inline]
    fn opt_def(self) -> Option<(DefKind, DefId)> {
        match self {
            Res::Def(kind, id) => Some((kind, id)),
            _ => None,
        }
    }
}
impl<T: Copy + MaybeDef> MaybeDef for &'_ T {
    #[inline]
    fn opt_def(self) -> Option<(DefKind, DefId)> {
        T::opt_def(*self)
    }
}
impl<T: MaybeDef> MaybeDef for Option<T> {
    #[inline]
    fn opt_def(self) -> Option<(DefKind, DefId)> {
        self.and_then(T::opt_def)
    }
}

/// A collection of helper functions for identifying known `DefId`s.
pub trait TyCtxtDefExt<'tcx>: HasTyCtxt<'tcx> {
    /// Gets the diagnostic name of `id` if it has one.
    #[inline]
    fn opt_diag_name(&self, id: impl MaybeDefId) -> Option<Symbol> {
        id.opt_def_id().and_then(|id| self.tcx().get_diagnostic_name(id))
    }

    /// Checks if `id` has the given diagnostic name.
    #[inline]
    fn is_diag_item(&self, id: impl MaybeDefId, name: Symbol) -> bool {
        id.opt_def_id()
            .is_some_and(|id| self.tcx().is_diagnostic_item(name, id))
    }

    /// Checks if `id` is the given `LangItem`.
    #[inline]
    fn is_lang_item(&self, id: impl MaybeDefId, item: LangItem) -> bool {
        id.opt_def_id()
            .is_some_and(|id| self.tcx().lang_items().get(item) == Some(id))
    }

    /// If `def` is a constructor gets the `DefId` of it's type or variant.
    #[inline]
    fn ctor_parent_id(&self, def: impl MaybeDef) -> Option<DefId> {
        match def.opt_def() {
            Some((DefKind::Ctor(..), id)) => self.tcx().opt_parent(id),
            _ => None,
        }
    }

    /// Checks if `def` is a constructor of `other`.
    #[inline]
    fn is_ctor_of(&self, def: impl MaybeDef, other: DefId) -> bool {
        self.ctor_parent_id(def) == Some(other)
    }

    /// Checks if `def` is a constructor of the given `LangItem`.
    #[inline]
    fn is_lang_ctor(&self, def: impl MaybeDef, item: LangItem) -> bool {
        self.is_lang_item(self.ctor_parent_id(def), item)
    }

    /// Checks if `def` is a constructor of the given diagnostic item.
    #[inline]
    fn is_diag_ctor(&self, def: impl MaybeDef, name: Symbol) -> bool {
        self.is_diag_item(self.ctor_parent_id(def), name)
    }

    /// Checks if `id` is an impl block.
    #[inline]
    fn is_impl(&self, id: impl MaybeDefId) -> bool {
        id.opt_def_id()
            .is_some_and(|id| matches!(self.tcx().def_kind(id), DefKind::Impl { .. }))
    }

    /// If `id` is an impl block gets the type it's an impl for.
    #[inline]
    fn opt_impl_ty(&self, id: impl MaybeDefId) -> Option<EarlyBinder<'tcx, Ty<'tcx>>> {
        id.opt_def_id()
            .filter(|&id| self.is_impl(id))
            .map(|id| self.tcx().type_of(id))
    }

    /// Checks if `id` is an impl block for the given diagnostic item.
    #[inline]
    fn is_impl_for_diag_ty(&self, id: impl MaybeDefId, name: Symbol) -> bool {
        self.is_diag_item(self.opt_impl_ty(id), name)
    }

    /// Checks if `id` is an impl block for the given `LangItem`.
    #[inline]
    fn is_impl_for_lang_ty(&self, id: impl MaybeDefId, item: LangItem) -> bool {
        self.is_lang_item(self.opt_impl_ty(id), item)
    }

    /// Gets the owning `DefId` of `def` is it's an associated item.
    #[inline]
    fn assoc_parent_id(&self, def: impl MaybeDef) -> Option<DefId> {
        match def.opt_def() {
            Some((DefKind::AssocConst | DefKind::AssocFn | DefKind::AssocTy, id)) => self.tcx().opt_parent(id),
            _ => None,
        }
    }

    /// Checks if `def` is an associated item owned by `other`.
    #[inline]
    fn is_assoc_of(&self, def: impl MaybeDef, other: DefId) -> bool {
        self.assoc_parent_id(def) == Some(other)
    }

    /// Checks if `def` is an associated item owned by the given diagnostic item.
    #[inline]
    fn is_assoc_of_diag_item(&self, def: impl MaybeDef, name: Symbol) -> bool {
        self.is_diag_item(self.assoc_parent_id(def), name)
    }

    /// If `def` is an associated item, gets the diagnostic name of it's owner.
    #[inline]
    fn assoc_parent_diag_name(&self, def: impl MaybeDef) -> Option<Symbol> {
        self.opt_diag_name(self.assoc_parent_id(def))
    }

    /// Checks if `def` is an associated item owned by the given `LangItem`.
    #[inline]
    fn is_assoc_of_lang_item(&self, def: impl MaybeDef, item: LangItem) -> bool {
        self.is_lang_item(self.assoc_parent_id(def), item)
    }

    /// If `def` is an associated item of an impl block, gets that impl block's `DefId`.
    #[inline]
    fn assoc_impl_id(&self, def: impl MaybeDef) -> Option<DefId> {
        self.assoc_parent_id(def).filter(|&id| self.is_impl(id))
    }

    /// If `def` is an associated item of an impl block, gets the type of that impl block.
    #[inline]
    fn assoc_self_ty(&self, def: impl MaybeDef) -> Option<EarlyBinder<'tcx, Ty<'tcx>>> {
        self.opt_impl_ty(self.assoc_parent_id(def))
    }

    /// Checks if `def` is an associated item of an impl block of the given diagnostic item.
    #[inline]
    fn is_assoc_of_diag_ty(&self, def: impl MaybeDef, name: Symbol) -> bool {
        self.assoc_self_ty(def).is_some_and(|ty| self.is_diag_item(ty, name))
    }

    /// Checks if `def` is an associated item of an impl block of the given `LangItem`.
    #[inline]
    fn is_assoc_of_lang_ty(&self, def: impl MaybeDef, item: LangItem) -> bool {
        self.assoc_self_ty(def).is_some_and(|ty| self.is_lang_item(ty, item))
    }

    /// Checks if `def` is an associated item of an impl block of type `bool`.
    #[inline]
    fn is_assoc_of_bool(&self, def: impl MaybeDef) -> bool {
        self.assoc_self_ty(def)
            .is_some_and(|ty| matches!(ty.skip_binder().kind(), ty::Bool))
    }

    /// Checks if `def` is an associated item of an impl block of type `char`.
    #[inline]
    fn is_assoc_of_char(&self, def: impl MaybeDef) -> bool {
        self.assoc_self_ty(def)
            .is_some_and(|ty| matches!(ty.skip_binder().kind(), ty::Char))
    }

    /// Checks if `def` is an associated item of an impl block of any integer type.
    #[inline]
    fn is_assoc_of_int(&self, def: impl MaybeDef) -> bool {
        self.assoc_self_ty(def)
            .is_some_and(|ty| matches!(ty.skip_binder().kind(), ty::Int(_) | ty::Uint(_)))
    }

    /// Checks if `def` is an associated item of an impl block of any floating point type.
    #[inline]
    fn is_assoc_of_float(&self, def: impl MaybeDef) -> bool {
        self.assoc_self_ty(def)
            .is_some_and(|ty| matches!(ty.skip_binder().kind(), ty::Float(_)))
    }

    /// Checks if `def` is an associated item of an impl block of any raw pointer type.
    #[inline]
    fn is_assoc_of_raw_ptr(&self, def: impl MaybeDef) -> bool {
        self.assoc_self_ty(def)
            .is_some_and(|ty| matches!(ty.skip_binder().kind(), ty::RawPtr(..)))
    }

    /// Checks if `def` is an associated item of an impl block of type `str`.
    #[inline]
    fn is_assoc_of_str(&self, def: impl MaybeDef) -> bool {
        self.assoc_self_ty(def)
            .is_some_and(|ty| matches!(ty.skip_binder().kind(), ty::Str))
    }

    /// Checks if `def` is an associated item of an impl block of any slice type.
    #[inline]
    fn is_assoc_of_slice(&self, def: impl MaybeDef) -> bool {
        self.assoc_self_ty(def)
            .is_some_and(|ty| matches!(ty.skip_binder().kind(), ty::Slice(_)))
    }

    /// Checks if `def` is an associated item of an impl block of any array type.
    #[inline]
    fn is_assoc_of_array(&self, def: impl MaybeDef) -> bool {
        self.assoc_self_ty(def)
            .is_some_and(|ty| matches!(ty.skip_binder().kind(), ty::Array(..)))
    }
}
impl<'tcx, T: ?Sized + HasTyCtxt<'tcx>> TyCtxtDefExt<'tcx> for T {}

/// Either a `HirId` or a type which can be identified by one.
pub trait HasHirId {
    fn hir_id(self) -> HirId;
}
impl HasHirId for HirId {
    #[inline]
    fn hir_id(self) -> HirId {
        self
    }
}
impl HasHirId for &'_ Expr<'_> {
    #[inline]
    fn hir_id(self) -> HirId {
        self.hir_id
    }
}

/// A type which contains the results of type dependant name resolution.
///
/// All the functions on this trait will lookup the path's resolution. This lookup
/// is not free and should be done at most once per item. e.g.
///
/// ```ignore
/// // Don't do this
/// let is_option_ctor = cx.is_path_lang_item(item, LangItem::String)
///     || cx.is_path_diag_item(item, sym::PathBuf);
///
/// // Prefer this
/// let is_option_ctor = cx.path_def_id(item).is_some_and(|did| {
///     tcx.lang_items().string() == Some(did)
///         || tcx.is_diagnostic_item(sym::PathBuf, did)
/// });
/// ```
pub trait PathRes<'tcx> {
    /// Gets the definition a node resolves to if it has a type dependent resolution.
    fn type_dependent_def(&self, node: impl HasHirId) -> Option<(DefKind, DefId)>;

    /// Gets the resolution of the path.
    ///
    /// `id` must be the `HirId` of the node containing `qpath`.
    #[cfg_attr(debug_assertions, track_caller)]
    fn qpath_res(&self, qpath: &QPath<'_>, id: HirId) -> Res {
        match qpath {
            QPath::Resolved(_, p) => p.res,
            QPath::TypeRelative(..) | QPath::LangItem(..) => self.type_dependent_def(id).to_res(),
        }
    }

    /// Gets the resolution of the item if it's a path. Returns `Res::Err` otherwise.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn path_res<'a>(&self, path: impl MaybeQPath<'a>) -> Res {
        match path.opt_qpath() {
            Some((qpath, hir_id)) => self.qpath_res(qpath, hir_id),
            None => Res::Err,
        }
    }

    /// Gets the definition the given node resolves to.
    #[cfg_attr(debug_assertions, track_caller)]
    fn path_def<'a>(&self, path: impl MaybeQPath<'a>) -> Option<(DefKind, DefId)> {
        match path.opt_qpath() {
            Some((&QPath::Resolved(_, p), _)) => match p.res {
                Res::Def(kind, id) => Some((kind, id)),
                _ => None,
            },
            Some((QPath::TypeRelative(..) | QPath::LangItem(..), id)) => self.type_dependent_def(id),
            _ => None,
        }
    }

    /// Gets the `DefId` of the item the given node resolves to.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn path_def_id<'a>(&self, path: impl MaybeQPath<'a>) -> Option<DefId> {
        self.path_def(path).map(|(_, id)| id)
    }

    /// Checks if the path resolves to the specified item.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn is_path_item<'a>(&self, path: impl MaybeQPath<'a>, did: DefId) -> bool {
        self.path_def_id(path) == Some(did)
    }

    /// Gets the diagnostic name of the item the given node resolves to.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn path_diag_name<'a>(&self, path: impl MaybeQPath<'a>) -> Option<Symbol>
    where
        Self: HasTyCtxt<'tcx>,
    {
        self.opt_diag_name(self.path_def_id(path))
    }

    /// Checks if the path resolves to the specified diagnostic item.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn is_path_diag_item<'a>(&self, path: impl MaybeQPath<'a>, name: Symbol) -> bool
    where
        Self: HasTyCtxt<'tcx>,
    {
        self.is_diag_item(self.path_def_id(path), name)
    }

    /// Checks if the path resolves to the specified `LangItem`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn is_path_lang_item<'a>(&self, path: impl MaybeQPath<'a>, item: LangItem) -> bool
    where
        Self: HasTyCtxt<'tcx>,
    {
        self.is_lang_item(self.path_def_id(path), item)
    }

    /// If the path resolves to a constructor, gets the `DefId` of the corresponding struct/variant.
    #[cfg_attr(debug_assertions, track_caller)]
    fn path_ctor_parent_id<'a>(&self, path: impl MaybeQPath<'a>) -> Option<DefId>
    where
        Self: HasTyCtxt<'tcx>,
    {
        self.ctor_parent_id(self.path_def(path))
    }

    /// Checks if the path resolves to the constructor of the specified `LangItem`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn is_path_lang_ctor<'a>(&self, path: impl MaybeQPath<'a>, item: LangItem) -> bool
    where
        Self: HasTyCtxt<'tcx>,
    {
        self.is_lang_ctor(self.path_def(path), item)
    }

    /// Checks if the node has a type-dependent resolution to the given diagnostic item.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn is_type_dependent_diag_item(&self, node: impl HasHirId, name: Symbol) -> bool
    where
        Self: HasTyCtxt<'tcx>,
    {
        self.is_diag_item(self.type_dependent_def(node.hir_id()), name)
    }

    /// Checks if the node has a type-dependent resolution to the given `LangItem`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn is_type_dependent_lang_item(&self, node: impl HasHirId, item: LangItem) -> bool
    where
        Self: HasTyCtxt<'tcx>,
    {
        self.is_lang_item(self.type_dependent_def(node.hir_id()), item)
    }

    /// If the node has a type-dependent resolution to an associated item gets
    /// its parent's `DefId`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn type_dependent_assoc_parent(&self, node: impl HasHirId) -> Option<DefId>
    where
        Self: HasTyCtxt<'tcx>,
    {
        self.assoc_parent_id(self.type_dependent_def(node.hir_id()))
    }

    /// Checks if the node has a type-dependent resolution to an associated item
    /// of the given diagnostic item.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn is_type_dependent_assoc_of_diag_item(&self, node: impl HasHirId, name: Symbol) -> bool
    where
        Self: HasTyCtxt<'tcx>,
    {
        self.is_assoc_of_diag_item(self.type_dependent_def(node.hir_id()), name)
    }

    /// Checks if the node has a type-dependent resolution to an associated item
    /// of the given `LangItem`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn is_type_dependent_assoc_of_lang_item(&self, node: impl HasHirId, item: LangItem) -> bool
    where
        Self: HasTyCtxt<'tcx>,
    {
        self.is_assoc_of_lang_item(self.type_dependent_def(node.hir_id()), item)
    }

    /// Checks if the node has a type-dependent resolution to an associated item
    /// of an impl block of the given diagnostic item.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn is_type_dependent_assoc_of_diag_ty(&self, node: impl HasHirId, name: Symbol) -> bool
    where
        Self: HasTyCtxt<'tcx>,
    {
        self.is_assoc_of_diag_ty(self.type_dependent_def(node.hir_id()), name)
    }

    /// Checks if the node has a type-dependent resolution to an associated item
    /// of an impl block of the given `LangItem`.
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn is_type_dependent_assoc_of_lang_ty(&self, node: impl HasHirId, item: LangItem) -> bool
    where
        Self: HasTyCtxt<'tcx>,
    {
        self.is_assoc_of_lang_ty(self.type_dependent_def(node.hir_id()), item)
    }
}
impl<'tcx> PathRes<'tcx> for LateContext<'tcx> {
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn type_dependent_def(&self, node: impl HasHirId) -> Option<(DefKind, DefId)> {
        if let Some(typeck) = self.maybe_typeck_results() {
            PathRes::type_dependent_def(typeck, node)
        } else {
            // It's possible to get the `TypeckResults` for any other body, but
            // attempting to lookup the type of something across bodies like this
            // is a good indication of a bug.
            debug_assert!(false, "attempted type-dependent lookup in a non-body context");
            None
        }
    }
}
impl PathRes<'_> for TypeckResults<'_> {
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    fn type_dependent_def(&self, node: impl HasHirId) -> Option<(DefKind, DefId)> {
        let id = node.hir_id();
        if id.owner == self.hir_owner {
            self.type_dependent_def(id)
        } else {
            debug_assert!(
                false,
                "attempted type-dependent lookup for a node in the wrong body.\
                    \n  in body `{:?}`\
                    \n  expected body `{:?}`",
                self.hir_owner, id.owner,
            );
            None
        }
    }
}
