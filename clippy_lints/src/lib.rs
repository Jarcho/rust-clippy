#![feature(array_windows)]
#![feature(binary_heap_into_iter_sorted)]
#![feature(box_patterns)]
#![feature(if_let_guard)]
#![feature(iter_intersperse)]
#![feature(let_chains)]
#![feature(lint_reasons)]
#![feature(never_type)]
#![feature(rustc_private)]
#![feature(stmt_expr_attributes)]
#![feature(unwrap_infallible)]
#![recursion_limit = "512"]
#![cfg_attr(feature = "deny-warnings", deny(warnings))]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::must_use_candidate,
    rustc::diagnostic_outside_of_impl,
    rustc::untranslatable_diagnostic
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications,
    rustc::internal
)]
// Disable this rustc lint for now, as it was also done in rustc
#![allow(rustc::potential_query_instability)]

// FIXME: switch to something more ergonomic here, once available.
// (Currently there is no way to opt into sysroot crates without `extern crate`.)
extern crate pulldown_cmark;
extern crate rustc_abi;
extern crate rustc_arena;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_attr;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_hir_analysis;
extern crate rustc_hir_pretty;
extern crate rustc_hir_typeck;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_lexer;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_parse;
extern crate rustc_resolve;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;
extern crate thin_vec;

#[macro_use]
extern crate clippy_utils;
#[macro_use]
extern crate declare_clippy_lint;

#[cfg(feature = "internal")]
pub mod deprecated_lints;
#[cfg_attr(feature = "internal", allow(clippy::missing_clippy_version_attribute))]
mod utils;

mod declared_lints;
mod renamed_lints;

// begin lints modules, do not remove this comment, it’s used in `update_lints`
mod absolute_paths;
mod allow_attributes;
mod almost_complete_range;
mod approx_const;
mod arc_with_non_send_sync;
mod as_conversions;
mod asm_syntax;
mod assertions_on_constants;
mod assertions_on_result_states;
mod assigning_clones;
mod async_yields_async;
mod attrs;
mod await_holding_invalid;
mod blocks_in_conditions;
mod bool_assert_comparison;
mod bool_to_int_with_if;
mod booleans;
mod borrow_deref_ref;
mod box_default;
mod cargo;
mod casts;
mod checked_conversions;
mod cognitive_complexity;
mod collapsible_if;
mod collection_is_never_read;
mod comparison_chain;
mod copies;
mod copy_iterator;
mod crate_in_macro_def;
mod create_dir;
mod dbg_macro;
mod default;
mod default_constructed_unit_structs;
mod default_instead_of_iter_empty;
mod default_numeric_fallback;
mod default_union_representation;
mod dereference;
mod derivable_impls;
mod derive;
mod disallowed_macros;
mod disallowed_methods;
mod disallowed_names;
mod disallowed_script_idents;
mod disallowed_types;
mod doc;
mod double_parens;
mod drop_forget_ref;
mod duplicate_mod;
mod else_if_without_else;
mod empty_drop;
mod empty_enum;
mod empty_with_brackets;
mod endian_bytes;
mod entry;
mod enum_clike;
mod equatable_if_let;
mod error_impl_error;
mod escape;
mod eta_reduction;
mod excessive_bools;
mod excessive_nesting;
mod exhaustive_items;
mod exit;
mod explicit_write;
mod extra_unused_type_parameters;
mod fallible_impl_from;
mod float_literal;
mod floating_point_arithmetic;
mod format;
mod format_args;
mod format_impl;
mod format_push_string;
mod formatting;
mod four_forward_slashes;
mod from_over_into;
mod from_raw_with_void_ptr;
mod from_str_radix_10;
mod functions;
mod future_not_send;
mod if_let_mutex;
mod if_not_else;
mod if_then_some_else_none;
mod ignored_unit_patterns;
mod impl_hash_with_borrow_str_and_bytes;
mod implicit_hasher;
mod implicit_return;
mod implicit_saturating_add;
mod implicit_saturating_sub;
mod implied_bounds_in_impls;
mod incompatible_msrv;
mod inconsistent_struct_constructor;
mod index_refutable_slice;
mod indexing_slicing;
mod ineffective_open_options;
mod infinite_iter;
mod inherent_impl;
mod inherent_to_string;
mod init_numbered_fields;
mod inline_fn_without_body;
mod instant_subtraction;
mod int_plus_one;
mod integer_division_remainder_used;
mod invalid_upcast_comparisons;
mod item_name_repetitions;
mod items_after_statements;
mod items_after_test_module;
mod iter_not_returning_iterator;
mod iter_over_hash_type;
mod iter_without_into_iter;
mod large_const_arrays;
mod large_enum_variant;
mod large_futures;
mod large_include_file;
mod large_stack_arrays;
mod large_stack_frames;
mod legacy_numeric_constants;
mod len_zero;
mod let_if_seq;
mod let_underscore;
mod let_with_type_underscore;
mod lifetimes;
mod lines_filter_map_ok;
mod literal_representation;
mod loops;
mod macro_metavars_in_unsafe;
mod macro_use;
mod main_recursion;
mod manual_assert;
mod manual_async_fn;
mod manual_bits;
mod manual_clamp;
mod manual_float_methods;
mod manual_hash_one;
mod manual_is_ascii_check;
mod manual_let_else;
mod manual_main_separator_str;
mod manual_non_exhaustive;
mod manual_range_patterns;
mod manual_rem_euclid;
mod manual_retain;
mod manual_slice_size_calculation;
mod manual_string_new;
mod manual_strip;
mod manual_unwrap_or_default;
mod map_unit_fn;
mod match_result_ok;
mod matches;
mod mem_replace;
mod methods;
mod min_ident_chars;
mod minmax;
mod misc;
mod misc_early;
mod mismatching_type_param_order;
mod missing_assert_message;
mod missing_asserts_for_indexing;
mod missing_const_for_fn;
mod missing_doc;
mod missing_enforced_import_rename;
mod missing_fields_in_debug;
mod missing_inline;
mod missing_trait_methods;
mod mixed_read_write_in_expression;
mod module_style;
mod multi_assignments;
mod multiple_bound_locations;
mod multiple_unsafe_ops_per_block;
mod mut_key;
mod mut_mut;
mod mut_reference;
mod mutable_debug_assertion;
mod mutex_atomic;
mod needless_arbitrary_self_type;
mod needless_bool;
mod needless_borrowed_ref;
mod needless_borrows_for_generic_args;
mod needless_continue;
mod needless_else;
mod needless_for_each;
mod needless_if;
mod needless_late_init;
mod needless_maybe_sized;
mod needless_parens_on_range_literals;
mod needless_pass_by_ref_mut;
mod needless_pass_by_value;
mod needless_question_mark;
mod needless_update;
mod neg_cmp_op_on_partial_ord;
mod neg_multiply;
mod new_without_default;
mod no_effect;
mod no_mangle_with_rust_abi;
mod non_canonical_impls;
mod non_copy_const;
mod non_expressive_names;
mod non_octal_unix_permissions;
mod non_send_fields_in_send_ty;
mod nonstandard_macro_braces;
mod octal_escapes;
mod only_used_in_recursion;
mod operators;
mod option_env_unwrap;
mod option_if_let_else;
mod overflow_check_conditional;
mod panic_in_result_fn;
mod panic_unimplemented;
mod partial_pub_fields;
mod partialeq_ne_impl;
mod partialeq_to_none;
mod pass_by_ref_or_value;
mod pattern_type_mismatch;
mod permissions_set_readonly_false;
mod precedence;
mod ptr;
mod ptr_offset_with_cast;
mod pub_underscore_fields;
mod pub_use;
mod question_mark;
mod question_mark_used;
mod ranges;
mod raw_strings;
mod rc_clone_in_vec_init;
mod read_zero_byte_vec;
mod redundant_async_block;
mod redundant_clone;
mod redundant_closure_call;
mod redundant_else;
mod redundant_field_names;
mod redundant_locals;
mod redundant_pub_crate;
mod redundant_slicing;
mod redundant_static_lifetimes;
mod redundant_type_annotations;
mod ref_option_ref;
mod ref_patterns;
mod reference;
mod regex;
mod repeat_vec_with_capacity;
mod reserve_after_initialization;
mod return_self_not_must_use;
mod returns;
mod same_name_method;
mod self_named_constructors;
mod semicolon_block;
mod semicolon_if_nothing_returned;
mod serde_api;
mod shadow;
mod significant_drop_tightening;
mod single_call_fn;
mod single_char_lifetime_names;
mod single_component_path_imports;
mod single_range_in_vec_init;
mod size_of_in_element_count;
mod size_of_ref;
mod slow_vector_initialization;
mod std_instead_of_core;
mod strings;
mod strlen_on_c_strings;
mod suspicious_operation_groupings;
mod suspicious_trait_impl;
mod suspicious_xor_used_as_pow;
mod swap;
mod swap_ptr_to_ref;
mod tabs_in_doc_comments;
mod temporary_assignment;
mod tests_outside_test_module;
mod thread_local_initializer_can_be_made_const;
mod to_digit_is_some;
mod to_string_trait_impl;
mod trailing_empty_array;
mod trait_bounds;
mod transmute;
mod tuple_array_conversions;
mod types;
mod unconditional_recursion;
mod undocumented_unsafe_blocks;
mod unicode;
mod uninhabited_references;
mod uninit_vec;
mod unit_return_expecting_ord;
mod unit_types;
mod unnamed_address;
mod unnecessary_box_returns;
mod unnecessary_map_on_constructor;
mod unnecessary_owned_empty_strings;
mod unnecessary_self_imports;
mod unnecessary_struct_initialization;
mod unnecessary_wraps;
mod unnested_or_patterns;
mod unsafe_removed_from_name;
mod unused_async;
mod unused_io_amount;
mod unused_peekable;
mod unused_rounding;
mod unused_self;
mod unused_unit;
mod unwrap;
mod unwrap_in_result;
mod upper_case_acronyms;
mod use_self;
mod useless_conversion;
mod vec;
mod vec_init_then_push;
mod visibility;
mod wildcard_imports;
mod write;
mod zero_div_zero;
mod zero_repeat_side_effects;
mod zero_sized_map_values;
// end lints modules, do not remove this comment, it’s used in `update_lints`

mod combined {
    use crate::allow_attributes::AllowAttribute;
    use crate::arc_with_non_send_sync::ArcWithNonSendSync;
    use crate::as_conversions::AsConversions;
    use crate::asm_syntax::{InlineAsmX86AttSyntax, InlineAsmX86IntelSyntax};
    use crate::assertions_on_constants::AssertionsOnConstants;
    use crate::assertions_on_result_states::AssertionsOnResultStates;
    use crate::async_yields_async::AsyncYieldsAsync;
    use crate::attrs::Attributes;
    use crate::blocks_in_conditions::BlocksInConditions;
    use crate::bool_assert_comparison::BoolAssertComparison;
    use crate::bool_to_int_with_if::BoolToIntWithIf;
    use crate::booleans::NonminimalBool;
    use crate::borrow_deref_ref::BorrowDerefRef;
    use crate::box_default::BoxDefault;
    use crate::collapsible_if::CollapsibleIf;
    use crate::collection_is_never_read::CollectionIsNeverRead;
    use crate::comparison_chain::ComparisonChain;
    use crate::copy_iterator::CopyIterator;
    use crate::crate_in_macro_def::CrateInMacroDef;
    use crate::create_dir::CreateDir;
    use crate::default::Default;
    use crate::default_constructed_unit_structs::DefaultConstructedUnitStructs;
    use crate::default_instead_of_iter_empty::DefaultIterEmpty;
    use crate::default_numeric_fallback::DefaultNumericFallback;
    use crate::default_union_representation::DefaultUnionRepresentation;
    use crate::derive::Derive;
    use crate::double_parens::DoubleParens;
    use crate::drop_forget_ref::DropForgetRef;
    use crate::duplicate_mod::DuplicateMod;
    use crate::else_if_without_else::ElseIfWithoutElse;
    use crate::empty_drop::EmptyDrop;
    use crate::empty_enum::EmptyEnum;
    use crate::empty_with_brackets::EmptyWithBrackets;
    use crate::endian_bytes::EndianBytes;
    use crate::entry::HashMapPass;
    use crate::enum_clike::UnportableVariant;
    use crate::equatable_if_let::PatternEquality;
    use crate::error_impl_error::ErrorImplError;
    use crate::eta_reduction::EtaReduction;
    use crate::exit::Exit;
    use crate::fallible_impl_from::FallibleImplFrom;
    use crate::float_literal::FloatLiteral;
    use crate::floating_point_arithmetic::FloatingPointArithmetic;
    use crate::format_push_string::FormatPushString;
    use crate::formatting::Formatting;
    use crate::four_forward_slashes::FourForwardSlashes;
    use crate::from_raw_with_void_ptr::FromRawWithVoidPtr;
    use crate::from_str_radix_10::FromStrRadix10;
    use crate::future_not_send::FutureNotSend;
    use crate::if_let_mutex::IfLetMutex;
    use crate::if_not_else::IfNotElse;
    use crate::ignored_unit_patterns::IgnoredUnitPatterns;
    use crate::impl_hash_with_borrow_str_and_bytes::ImplHashWithBorrowStrBytes;
    use crate::implicit_hasher::ImplicitHasher;
    use crate::implicit_return::ImplicitReturn;
    use crate::implicit_saturating_add::ImplicitSaturatingAdd;
    use crate::implicit_saturating_sub::ImplicitSaturatingSub;
    use crate::implied_bounds_in_impls::ImpliedBoundsInImpls;
    use crate::inconsistent_struct_constructor::InconsistentStructConstructor;
    use crate::ineffective_open_options::IneffectiveOpenOptions;
    use crate::infinite_iter::InfiniteIter;
    use crate::inherent_impl::MultipleInherentImpl;
    use crate::inherent_to_string::InherentToString;
    use crate::init_numbered_fields::NumberedFields;
    use crate::inline_fn_without_body::InlineFnWithoutBody;
    use crate::int_plus_one::IntPlusOne;
    use crate::integer_division_remainder_used::IntegerDivisionRemainderUsed;
    use crate::invalid_upcast_comparisons::InvalidUpcastComparisons;
    use crate::items_after_statements::ItemsAfterStatements;
    use crate::items_after_test_module::ItemsAfterTestModule;
    use crate::iter_over_hash_type::IterOverHashType;
    use crate::iter_without_into_iter::IterWithoutIntoIter;
    use crate::len_zero::LenZero;
    use crate::let_if_seq::LetIfSeq;
    use crate::let_underscore::LetUnderscore;
    use crate::let_with_type_underscore::UnderscoreTyped;
    use crate::lifetimes::Lifetimes;
    use crate::lines_filter_map_ok::LinesFilterMapOk;
    use crate::macro_use::MacroUseImports;
    use crate::main_recursion::MainRecursion;
    use crate::manual_async_fn::ManualAsyncFn;
    use crate::manual_float_methods::ManualFloatMethods;
    use crate::manual_range_patterns::ManualRangePatterns;
    use crate::manual_slice_size_calculation::ManualSliceSizeCalculation;
    use crate::manual_string_new::ManualStringNew;
    use crate::manual_unwrap_or_default::ManualUnwrapOrDefault;
    use crate::map_unit_fn::MapUnit;
    use crate::match_result_ok::MatchResultOk;
    use crate::minmax::MinMaxPass;
    use crate::misc::LintPass;
    use crate::misc_early::MiscEarlyLints;
    use crate::mismatching_type_param_order::TypeParamMismatch;
    use crate::missing_assert_message::MissingAssertMessage;
    use crate::missing_asserts_for_indexing::MissingAssertsForIndexing;
    use crate::missing_fields_in_debug::MissingFieldsInDebug;
    use crate::missing_inline::MissingInline;
    use crate::missing_trait_methods::MissingTraitMethods;
    use crate::mixed_read_write_in_expression::EvalOrderDependence;
    use crate::multi_assignments::MultiAssignments;
    use crate::multiple_bound_locations::MultipleBoundLocations;
    use crate::multiple_unsafe_ops_per_block::MultipleUnsafeOpsPerBlock;
    use crate::mut_mut::MutMut;
    use crate::mut_reference::UnnecessaryMutPassed;
    use crate::mutable_debug_assertion::DebugAssertWithMutCall;
    use crate::mutex_atomic::Mutex;
    use crate::needless_arbitrary_self_type::NeedlessArbitrarySelfType;
    use crate::needless_bool::{BoolComparison, NeedlessBool};
    use crate::needless_borrowed_ref::NeedlessBorrowedRef;
    use crate::needless_continue::NeedlessContinue;
    use crate::needless_else::NeedlessElse;
    use crate::needless_for_each::NeedlessForEach;
    use crate::needless_if::NeedlessIf;
    use crate::needless_late_init::NeedlessLateInit;
    use crate::needless_maybe_sized::NeedlessMaybeSized;
    use crate::needless_parens_on_range_literals::NeedlessParensOnRangeLiterals;
    use crate::needless_pass_by_value::NeedlessPassByValue;
    use crate::needless_update::NeedlessUpdate;
    use crate::neg_cmp_op_on_partial_ord::NoNegCompOpForPartialOrd;
    use crate::neg_multiply::NegMultiply;
    use crate::new_without_default::NewWithoutDefault;
    use crate::no_effect::NoEffect;
    use crate::no_mangle_with_rust_abi::NoMangleWithRustAbi;
    use crate::non_canonical_impls::NonCanonicalImpls;
    use crate::non_octal_unix_permissions::NonOctalUnixPermissions;
    use crate::octal_escapes::OctalEscapes;
    use crate::only_used_in_recursion::OnlyUsedInRecursion;
    use crate::option_env_unwrap::OptionEnvUnwrap;
    use crate::option_if_let_else::OptionIfLetElse;
    use crate::overflow_check_conditional::OverflowCheckConditional;
    use crate::panic_in_result_fn::PanicInResultFn;
    use crate::partial_pub_fields::PartialPubFields;
    use crate::partialeq_ne_impl::PartialEqNeImpl;
    use crate::partialeq_to_none::PartialeqToNone;
    use crate::pattern_type_mismatch::PatternTypeMismatch;
    use crate::permissions_set_readonly_false::PermissionsSetReadonlyFalse;
    use crate::precedence::Precedence;
    use crate::ptr::Ptr;
    use crate::ptr_offset_with_cast::PtrOffsetWithCast;
    use crate::pub_use::PubUse;
    use crate::question_mark_used::QuestionMarkUsed;
    use crate::rc_clone_in_vec_init::RcCloneInVecInit;
    use crate::read_zero_byte_vec::ReadZeroByteVec;
    use crate::redundant_async_block::RedundantAsyncBlock;
    use crate::redundant_clone::RedundantClone;
    use crate::redundant_closure_call::RedundantClosureCall;
    use crate::redundant_else::RedundantElse;
    use crate::redundant_locals::RedundantLocals;
    use crate::redundant_pub_crate::RedundantPubCrate;
    use crate::redundant_slicing::RedundantSlicing;
    use crate::redundant_type_annotations::RedundantTypeAnnotations;
    use crate::ref_option_ref::RefOptionRef;
    use crate::ref_patterns::RefPatterns;
    use crate::reference::DerefAddrOf;
    use crate::regex::Regex;
    use crate::repeat_vec_with_capacity::RepeatVecWithCapacity;
    use crate::reserve_after_initialization::ReserveAfterInitialization;
    use crate::return_self_not_must_use::ReturnSelfNotMustUse;
    use crate::returns::Return;
    use crate::semicolon_if_nothing_returned::SemicolonIfNothingReturned;
    use crate::serde_api::SerdeApi;
    use crate::shadow::Shadow;
    use crate::single_char_lifetime_names::SingleCharLifetimeNames;
    use crate::single_component_path_imports::SingleComponentPathImports;
    use crate::single_range_in_vec_init::SingleRangeInVecInit;
    use crate::size_of_ref::SizeOfRef;
    use crate::slow_vector_initialization::SlowVectorInit;
    use crate::std_instead_of_core::StdReexports;
    use crate::strings::{StrToString, StringAdd, StringLitAsBytes, StringToString, TrimSplitWhitespace};
    use crate::strlen_on_c_strings::StrlenOnCStrings;
    use crate::suspicious_operation_groupings::SuspiciousOperationGroupings;
    use crate::suspicious_trait_impl::SuspiciousImpl;
    use crate::suspicious_xor_used_as_pow::ConfusingXorAndPow;
    use crate::swap::Swap;
    use crate::swap_ptr_to_ref::SwapPtrToRef;
    use crate::tabs_in_doc_comments::TabsInDocComments;
    use crate::temporary_assignment::TemporaryAssignment;
    use crate::tests_outside_test_module::TestsOutsideTestModule;
    use crate::to_digit_is_some::ToDigitIsSome;
    use crate::to_string_trait_impl::ToStringTraitImpl;
    use crate::trailing_empty_array::TrailingEmptyArray;
    use crate::unicode::Unicode;
    use crate::uninhabited_references::UninhabitedReferences;
    use crate::uninit_vec::UninitVec;
    use crate::unit_return_expecting_ord::UnitReturnExpectingOrd;
    use crate::unit_types::UnitTypes;
    use crate::unnamed_address::UnnamedAddress;
    use crate::unnecessary_map_on_constructor::UnnecessaryMapOnConstructor;
    use crate::unnecessary_owned_empty_strings::UnnecessaryOwnedEmptyStrings;
    use crate::unnecessary_self_imports::UnnecessarySelfImports;
    use crate::unnecessary_struct_initialization::UnnecessaryStruct;
    use crate::unsafe_removed_from_name::UnsafeNameRemoval;
    use crate::unused_async::UnusedAsync;
    use crate::unused_io_amount::UnusedIoAmount;
    use crate::unused_peekable::UnusedPeekable;
    use crate::unused_rounding::UnusedRounding;
    use crate::unused_unit::UnusedUnit;
    use crate::unwrap::Unwrap;
    use crate::unwrap_in_result::UnwrapInResult;
    use crate::useless_conversion::UselessConversion;
    use crate::vec_init_then_push::VecInitThenPush;
    use crate::visibility::Visibility;
    use crate::zero_div_zero::ZeroDiv;
    use crate::zero_repeat_side_effects::ZeroRepeatSideEffects;
    use crate::zero_sized_map_values::ZeroSizedMapValues;

    rustc_lint::early_lint_methods!(
        rustc_lint::declare_combined_early_lint_pass,
        [
            pub(crate) ClippyEarlyLintPass,
            [
                SuspiciousOperationGroupings: SuspiciousOperationGroupings,
                DerefAddrOf: DerefAddrOf,
                DoubleParens: DoubleParens,
                UnsafeNameRemoval: UnsafeNameRemoval,
                ElseIfWithoutElse: ElseIfWithoutElse,
                IntPlusOne: IntPlusOne,
                Formatting: Formatting,
                MiscEarlyLints: MiscEarlyLints,
                UnusedUnit: UnusedUnit,
                CollapsibleIf: CollapsibleIf,
                Precedence: Precedence,
                NeedlessContinue: NeedlessContinue,
                RedundantElse: RedundantElse,
                NeedlessArbitrarySelfType: NeedlessArbitrarySelfType,
                TabsInDocComments: TabsInDocComments,
                OptionEnvUnwrap: OptionEnvUnwrap,
                InlineAsmX86AttSyntax: InlineAsmX86AttSyntax,
                InlineAsmX86IntelSyntax: InlineAsmX86IntelSyntax,
                OctalEscapes: OctalEscapes,
                SingleCharLifetimeNames: SingleCharLifetimeNames,
                CrateInMacroDef: CrateInMacroDef,
                EmptyWithBrackets: EmptyWithBrackets,
                PubUse: PubUse,
                UnusedRounding: UnusedRounding,
                MultiAssignments: MultiAssignments,
                PartialPubFields: PartialPubFields,
                RefPatterns: RefPatterns,
                NeedlessElse: NeedlessElse,
                Visibility: Visibility,
                MultipleBoundLocations: MultipleBoundLocations,
                UnnecessarySelfImports: UnnecessarySelfImports
                SingleComponentPathImports: SingleComponentPathImports::default(),
                DuplicateMod: DuplicateMod::default(),
            ]
        ]
    );

    rustc_lint::late_lint_methods!(
        rustc_lint::declare_combined_late_lint_pass,
        [
            pub(crate) ClippyLateLintPass,
            [
                NonminimalBool: NonminimalBool,
                UnportableVariant: UnportableVariant,
                FloatLiteral: FloatLiteral,
                Ptr: Ptr,
                NeedlessBool: NeedlessBool,
                BoolComparison: BoolComparison,
                NeedlessForEach: NeedlessForEach,
                LintPass: LintPass,
                EtaReduction: EtaReduction,
                MutMut: MutMut,
                UnnecessaryMutPassed: UnnecessaryMutPassed,
                LenZero: LenZero,
                Attributes: Attributes,
                BlocksInConditions: BlocksInConditions,
                Unicode: Unicode,
                UninitVec: UninitVec,
                UnitReturnExpectingOrd: UnitReturnExpectingOrd,
                StringAdd: StringAdd,
                ImplicitReturn: ImplicitReturn,
                ImplicitSaturatingSub: ImplicitSaturatingSub,
                DefaultNumericFallback: DefaultNumericFallback,
                InconsistentStructConstructor: InconsistentStructConstructor,
                NonOctalUnixPermissions: NonOctalUnixPermissions,
                Lifetimes: Lifetimes,
                HashMapPass: HashMapPass,
                MinMaxPass: MinMaxPass,
                ZeroDiv: ZeroDiv,
                Mutex: Mutex,
                NeedlessUpdate: NeedlessUpdate,
                NeedlessBorrowedRef: NeedlessBorrowedRef,
                BorrowDerefRef: BorrowDerefRef,
                UnitTypes: UnitTypes,
                StringLitAsBytes: StringLitAsBytes,
                Derive: Derive,
                DropForgetRef: DropForgetRef,
                EmptyEnum: EmptyEnum,
                InvalidUpcastComparisons: InvalidUpcastComparisons,
                Swap: Swap,
                NegMultiply: NegMultiply,
                LetIfSeq: LetIfSeq,
                EvalOrderDependence: EvalOrderDependence,
                SerdeApi: SerdeApi,
                TemporaryAssignment: TemporaryAssignment,
                CopyIterator: CopyIterator,
                OverflowCheckConditional: OverflowCheckConditional,
                MissingInline: MissingInline,
                MatchResultOk: MatchResultOk,
                PartialEqNeImpl: PartialEqNeImpl,
                UnusedIoAmount: UnusedIoAmount,
                NeedlessPassByValue: NeedlessPassByValue,
                RefOptionRef: RefOptionRef,
                InfiniteIter: InfiniteIter,
                InlineFnWithoutBody: InlineFnWithoutBody,
                ImplicitHasher: ImplicitHasher,
                FallibleImplFrom: FallibleImplFrom,
                QuestionMarkUsed: QuestionMarkUsed,
                SuspiciousImpl: SuspiciousImpl,
                MapUnit: MapUnit,
                MultipleInherentImpl: MultipleInherentImpl,
                NoNegCompOpForPartialOrd: NoNegCompOpForPartialOrd,
                Unwrap: Unwrap,
                PtrOffsetWithCast: PtrOffsetWithCast,
                RedundantClone: RedundantClone,
                SlowVectorInit: SlowVectorInit,
                AssertionsOnConstants: AssertionsOnConstants,
                AssertionsOnResultStates: AssertionsOnResultStates,
                InherentToString: InherentToString,
                ComparisonChain: ComparisonChain,
                RedundantClosureCall: RedundantClosureCall,
                Return: Return,
                ItemsAfterStatements: ItemsAfterStatements,
                NeedlessParensOnRangeLiterals: NeedlessParensOnRangeLiterals,
                CreateDir: CreateDir,
                DebugAssertWithMutCall: DebugAssertWithMutCall,
                Exit: Exit,
                ToDigitIsSome: ToDigitIsSome,
                FloatingPointArithmetic: FloatingPointArithmetic,
                AsConversions: AsConversions,
                LetUnderscore: LetUnderscore,
                UnnamedAddress: UnnamedAddress,
                OptionIfLetElse: OptionIfLetElse,
                FutureNotSend: FutureNotSend,
                IfLetMutex: IfLetMutex,
                IfNotElse: IfNotElse,
                PatternEquality: PatternEquality,
                ManualAsyncFn: ManualAsyncFn,
                PanicInResultFn: PanicInResultFn,
                PatternTypeMismatch: PatternTypeMismatch,
                UnwrapInResult: UnwrapInResult,
                SemicolonIfNothingReturned: SemicolonIfNothingReturned,
                AsyncYieldsAsync: AsyncYieldsAsync,
                EmptyDrop: EmptyDrop,
                StrToString: StrToString,
                StringToString: StringToString,
                ZeroSizedMapValues: ZeroSizedMapValues,
                RedundantSlicing: RedundantSlicing,
                FromStrRadix10: FromStrRadix10,
                BoolAssertComparison: BoolAssertComparison,
                StrlenOnCStrings: StrlenOnCStrings,
                TrailingEmptyArray: TrailingEmptyArray,
                NeedlessLateInit: NeedlessLateInit,
                ReturnSelfNotMustUse: ReturnSelfNotMustUse,
                NumberedFields: NumberedFields,
                DefaultUnionRepresentation: DefaultUnionRepresentation,
                UnnecessaryOwnedEmptyStrings: UnnecessaryOwnedEmptyStrings,
                FormatPushString: FormatPushString,
                TrimSplitWhitespace: TrimSplitWhitespace,
                RcCloneInVecInit: RcCloneInVecInit,
                SwapPtrToRef: SwapPtrToRef,
                TypeParamMismatch: TypeParamMismatch,
                ReadZeroByteVec: ReadZeroByteVec,
                DefaultIterEmpty: DefaultIterEmpty,
                PartialeqToNone: PartialeqToNone,
                ManualStringNew: ManualStringNew,
                UnusedPeekable: UnusedPeekable,
                BoolToIntWithIf: BoolToIntWithIf,
                BoxDefault: BoxDefault,
                ImplicitSaturatingAdd: ImplicitSaturatingAdd,
                MissingTraitMethods: MissingTraitMethods,
                FromRawWithVoidPtr: FromRawWithVoidPtr,
                ConfusingXorAndPow: ConfusingXorAndPow,
                PermissionsSetReadonlyFalse: PermissionsSetReadonlyFalse,
                SizeOfRef: SizeOfRef,
                MultipleUnsafeOpsPerBlock: MultipleUnsafeOpsPerBlock,
                NoMangleWithRustAbi: NoMangleWithRustAbi,
                CollectionIsNeverRead: CollectionIsNeverRead,
                MissingAssertMessage: MissingAssertMessage,
                NeedlessMaybeSized: NeedlessMaybeSized,
                RedundantAsyncBlock: RedundantAsyncBlock,
                UnderscoreTyped: UnderscoreTyped,
                AllowAttribute: AllowAttribute,
                UnnecessaryStruct: UnnecessaryStruct,
                LinesFilterMapOk: LinesFilterMapOk,
                TestsOutsideTestModule: TestsOutsideTestModule,
                ManualSliceSizeCalculation: ManualSliceSizeCalculation,
                ItemsAfterTestModule: ItemsAfterTestModule,
                DefaultConstructedUnitStructs: DefaultConstructedUnitStructs,
                MissingFieldsInDebug: MissingFieldsInDebug,
                EndianBytes: EndianBytes,
                RedundantTypeAnnotations: RedundantTypeAnnotations,
                ArcWithNonSendSync: ArcWithNonSendSync,
                NeedlessIf: NeedlessIf,
                SingleRangeInVecInit: SingleRangeInVecInit,
                NonCanonicalImpls: NonCanonicalImpls,
                ManualRangePatterns: ManualRangePatterns,
                ManualFloatMethods: ManualFloatMethods,
                FourForwardSlashes: FourForwardSlashes,
                ErrorImplError: ErrorImplError,
                RedundantLocals: RedundantLocals,
                IgnoredUnitPatterns: IgnoredUnitPatterns,
                ImpliedBoundsInImpls: ImpliedBoundsInImpls,
                MissingAssertsForIndexing: MissingAssertsForIndexing,
                UnnecessaryMapOnConstructor: UnnecessaryMapOnConstructor,
                IterWithoutIntoIter: IterWithoutIntoIter,
                IterOverHashType: IterOverHashType,
                ImplHashWithBorrowStrBytes: ImplHashWithBorrowStrBytes,
                RepeatVecWithCapacity: RepeatVecWithCapacity,
                UninhabitedReferences: UninhabitedReferences,
                IneffectiveOpenOptions: IneffectiveOpenOptions,
                ToStringTraitImpl: ToStringTraitImpl,
                ZeroRepeatSideEffects: ZeroRepeatSideEffects,
                ManualUnwrapOrDefault: ManualUnwrapOrDefault,
                IntegerDivisionRemainderUsed: IntegerDivisionRemainderUsed,
                Shadow: Shadow::default(),
                MainRecursion: MainRecursion::default(),
                NoEffect: NoEffect::default(),
                Regex: Regex::default(),
                NewWithoutDefault: NewWithoutDefault::default(),
                UselessConversion: UselessConversion::default(),
                Default: Default::default(),
                RedundantPubCrate: RedundantPubCrate::default(),
                MacroUseImports: MacroUseImports::default(),
                VecInitThenPush: VecInitThenPush::default(),
                UnusedAsync: UnusedAsync::default(),
                OnlyUsedInRecursion: OnlyUsedInRecursion::default(),
                StdReexports: StdReexports::default(),
                ReserveAfterInitialization: ReserveAfterInitialization::default(),
            ]
        ]
    );
}

use clippy_config::{get_configuration_metadata, Conf};
use clippy_utils::macros::FormatArgsStorage;
use rustc_data_structures::fx::FxHashSet;
use rustc_lint::{Lint, LintId};
use std::collections::BTreeMap;

/// Register all pre expansion lints
///
/// Pre-expansion lints run before any macro expansion has happened.
///
/// Note that due to the architecture of the compiler, currently `cfg_attr` attributes on crate
/// level (i.e `#![cfg_attr(...)]`) will still be expanded even when using a pre-expansion pass.
///
/// Used in `./src/driver.rs`.
pub fn register_pre_expansion_lints(store: &mut rustc_lint::LintStore, conf: &'static Conf) {
    // NOTE: Do not add any more pre-expansion passes. These should be removed eventually.
    let msrv = || conf.msrv.clone();

    store.register_pre_expansion_pass(move || Box::new(attrs::EarlyAttributes { msrv: msrv() }));
}

#[derive(Default)]
struct RegistrationGroups {
    all: Vec<LintId>,
    cargo: Vec<LintId>,
    complexity: Vec<LintId>,
    correctness: Vec<LintId>,
    nursery: Vec<LintId>,
    pedantic: Vec<LintId>,
    perf: Vec<LintId>,
    restriction: Vec<LintId>,
    style: Vec<LintId>,
    suspicious: Vec<LintId>,
    #[cfg(feature = "internal")]
    internal: Vec<LintId>,
}

impl RegistrationGroups {
    #[rustfmt::skip]
    fn register(self, store: &mut rustc_lint::LintStore) {
        store.register_group(true, "clippy::all", Some("clippy_all"), self.all);
        store.register_group(true, "clippy::cargo", Some("clippy_cargo"), self.cargo);
        store.register_group(true, "clippy::complexity", Some("clippy_complexity"), self.complexity);
        store.register_group(true, "clippy::correctness", Some("clippy_correctness"), self.correctness);
        store.register_group(true, "clippy::nursery", Some("clippy_nursery"), self.nursery);
        store.register_group(true, "clippy::pedantic", Some("clippy_pedantic"), self.pedantic);
        store.register_group(true, "clippy::perf", Some("clippy_perf"), self.perf);
        store.register_group(true, "clippy::restriction", Some("clippy_restriction"), self.restriction);
        store.register_group(true, "clippy::style", Some("clippy_style"), self.style);
        store.register_group(true, "clippy::suspicious", Some("clippy_suspicious"), self.suspicious);
        #[cfg(feature = "internal")]
        store.register_group(true, "clippy::internal", Some("clippy_internal"), self.internal);
    }
}

#[derive(Copy, Clone)]
pub(crate) enum LintCategory {
    Cargo,
    Complexity,
    Correctness,
    Nursery,
    Pedantic,
    Perf,
    Restriction,
    Style,
    Suspicious,
    #[cfg(feature = "internal")]
    Internal,
}
#[allow(clippy::enum_glob_use)]
use LintCategory::*;

impl LintCategory {
    fn is_all(self) -> bool {
        matches!(self, Correctness | Suspicious | Style | Complexity | Perf)
    }

    fn group(self, groups: &mut RegistrationGroups) -> &mut Vec<LintId> {
        match self {
            Cargo => &mut groups.cargo,
            Complexity => &mut groups.complexity,
            Correctness => &mut groups.correctness,
            Nursery => &mut groups.nursery,
            Pedantic => &mut groups.pedantic,
            Perf => &mut groups.perf,
            Restriction => &mut groups.restriction,
            Style => &mut groups.style,
            Suspicious => &mut groups.suspicious,
            #[cfg(feature = "internal")]
            Internal => &mut groups.internal,
        }
    }
}

pub(crate) struct LintInfo {
    /// Double reference to maintain pointer equality
    lint: &'static &'static Lint,
    category: LintCategory,
    explanation: &'static str,
}

pub fn explain(name: &str) -> i32 {
    let target = format!("clippy::{}", name.to_ascii_uppercase());
    if let Some(info) = declared_lints::LINTS.iter().find(|info| info.lint.name == target) {
        println!("{}", info.explanation);
        // Check if the lint has configuration
        let mut mdconf = get_configuration_metadata();
        let name = name.to_ascii_lowercase();
        mdconf.retain(|cconf| cconf.lints.contains(&name));
        if !mdconf.is_empty() {
            println!("### Configuration for {}:\n", info.lint.name_lower());
            for conf in mdconf {
                println!("{conf}");
            }
        }
        0
    } else {
        println!("unknown lint: {name}");
        1
    }
}

fn register_categories(store: &mut rustc_lint::LintStore) {
    let mut groups = RegistrationGroups::default();

    for LintInfo { lint, category, .. } in declared_lints::LINTS {
        if category.is_all() {
            groups.all.push(LintId::of(lint));
        }

        category.group(&mut groups).push(LintId::of(lint));
    }

    let lints: Vec<&'static Lint> = declared_lints::LINTS.iter().map(|info| *info.lint).collect();

    store.register_lints(&lints);
    groups.register(store);
}

/// Register all lints and lint groups with the rustc lint store
///
/// Used in `./src/driver.rs`.
#[expect(clippy::too_many_lines)]
pub fn register_lints(store: &mut rustc_lint::LintStore, conf: &'static Conf) {
    let Conf {
        ref absolute_paths_allowed_crates,
        absolute_paths_max_segments,
        accept_comment_above_attributes,
        accept_comment_above_statement,
        allow_dbg_in_tests,
        allow_expect_in_tests,
        allow_mixed_uninlined_format_args,
        allow_one_hash_in_raw_strings,
        allow_panic_in_tests,
        allow_print_in_tests,
        allow_private_module_inception,
        allow_unwrap_in_tests,
        allow_useless_vec_in_tests,
        ref allowed_dotfiles,
        ref allowed_idents_below_min_chars,
        ref allowed_scripts,
        ref allowed_wildcard_imports,
        ref arithmetic_side_effects_allowed_binary,
        ref arithmetic_side_effects_allowed_unary,
        ref arithmetic_side_effects_allowed,
        array_size_threshold,
        avoid_breaking_exported_api,
        ref await_holding_invalid_types,
        cargo_ignore_publish,
        cognitive_complexity_threshold,
        ref disallowed_macros,
        ref disallowed_methods,
        ref disallowed_names,
        ref disallowed_types,
        ref doc_valid_idents,
        enable_raw_pointer_heuristic_for_send,
        enforce_iter_loop_reborrow,
        ref enforced_import_renames,
        enum_variant_name_threshold,
        enum_variant_size_threshold,
        excessive_nesting_threshold,
        future_size_threshold,
        ref ignore_interior_mutability,
        large_error_threshold,
        literal_representation_threshold,
        matches_for_let_else,
        max_fn_params_bools,
        max_include_file_size,
        max_struct_bools,
        max_suggested_slice_pattern_length,
        max_trait_bounds,
        min_ident_chars_threshold,
        missing_docs_in_crate_items,
        ref msrv,
        pass_by_value_size_limit,
        semicolon_inside_block_ignore_singleline,
        semicolon_outside_block_ignore_multiline,
        single_char_binding_names_threshold,
        stack_size_threshold,
        ref standard_macro_braces,
        struct_field_name_threshold,
        suppress_restriction_lint_in_const,
        too_large_for_stack,
        too_many_arguments_threshold,
        too_many_lines_threshold,
        trivial_copy_size_limit,
        type_complexity_threshold,
        unnecessary_box_size,
        unreadable_literal_lint_fractions,
        upper_case_acronyms_aggressive,
        vec_box_size_threshold,
        verbose_bit_mask_threshold,
        warn_on_all_wildcard_imports,
        check_private_items,
        pub_underscore_fields_behavior,
        ref allowed_duplicate_crates,
        allow_comparison_to_zero,
        ref allowed_prefixes,
        ref allow_renamed_params_for,

        blacklisted_names: _,
        cyclomatic_complexity_threshold: _,
        warn_unsafe_macro_metavars_in_private_macros,
    } = *conf;
    let msrv = || msrv.clone();

    register_removed_non_tool_lints(store);
    register_categories(store);

    include!("lib.deprecated.rs");

    #[cfg(feature = "internal")]
    {
        if std::env::var("ENABLE_METADATA_COLLECTION").eq(&Ok("1".to_string())) {
            store.register_late_pass(|_| Box::new(utils::internal_lints::metadata_collector::MetadataCollector::new()));
            return;
        }
    }

    let format_args_storage = FormatArgsStorage::default();
    let format_args = format_args_storage.clone();
    store.register_early_pass(move || {
        Box::new(utils::format_args_collector::FormatArgsCollector::new(
            format_args.clone(),
        ))
    });

    // all the internal lints
    #[cfg(feature = "internal")]
    {
        store.register_early_pass(|| {
            Box::new(utils::internal_lints::unsorted_clippy_utils_paths::UnsortedClippyUtilsPaths)
        });
        store.register_early_pass(|| Box::new(utils::internal_lints::produce_ice::ProduceIce));
        store.register_late_pass(|_| Box::new(utils::internal_lints::collapsible_calls::CollapsibleCalls));
        store.register_late_pass(|_| {
            Box::new(utils::internal_lints::compiler_lint_functions::CompilerLintFunctions::new())
        });
        store.register_late_pass(|_| Box::new(utils::internal_lints::invalid_paths::InvalidPaths));
        store.register_late_pass(|_| {
            Box::<utils::internal_lints::interning_defined_symbol::InterningDefinedSymbol>::default()
        });
        store.register_late_pass(|_| {
            Box::<utils::internal_lints::lint_without_lint_pass::LintWithoutLintPass>::default()
        });
        store.register_late_pass(|_| Box::<utils::internal_lints::unnecessary_def_path::UnnecessaryDefPath>::default());
        store.register_late_pass(|_| Box::new(utils::internal_lints::outer_expn_data_pass::OuterExpnDataPass));
        store.register_late_pass(|_| Box::new(utils::internal_lints::msrv_attr_impl::MsrvAttrImpl));
        store.register_late_pass(|_| {
            Box::new(utils::internal_lints::almost_standard_lint_formulation::AlmostStandardFormulation::new())
        });
    }

    store.register_late_pass(|_| Box::new(combined::ClippyLateLintPass::new()));
    store.register_early_pass(|| Box::new(combined::ClippyEarlyLintPass::new()));
    store.register_late_pass(move |_| {
        Box::new(operators::arithmetic_side_effects::ArithmeticSideEffects::new(
            arithmetic_side_effects_allowed
                .iter()
                .flat_map(|el| [[el.clone(), "*".to_string()], ["*".to_string(), el.clone()]])
                .chain(arithmetic_side_effects_allowed_binary.clone())
                .collect(),
            arithmetic_side_effects_allowed
                .iter()
                .chain(arithmetic_side_effects_allowed_unary.iter())
                .cloned()
                .collect(),
        ))
    });
    store.register_late_pass(|_| Box::new(utils::dump_hir::DumpHir));
    store.register_late_pass(|_| Box::new(utils::author::Author));
    store.register_late_pass(move |_| {
        Box::new(await_holding_invalid::AwaitHolding::new(
            await_holding_invalid_types.clone(),
        ))
    });
    store.register_late_pass(move |_| {
        Box::new(types::Types::new(
            vec_box_size_threshold,
            type_complexity_threshold,
            avoid_breaking_exported_api,
        ))
    });
    store.register_late_pass(|_| Box::<significant_drop_tightening::SignificantDropTightening<'_>>::default());
    store.register_late_pass(move |_| Box::new(approx_const::ApproxConstant::new(msrv())));
    let format_args = format_args_storage.clone();
    store.register_late_pass(move |_| {
        Box::new(methods::Methods::new(
            avoid_breaking_exported_api,
            msrv(),
            allow_expect_in_tests,
            allow_unwrap_in_tests,
            allowed_dotfiles.clone(),
            format_args.clone(),
        ))
    });
    store.register_late_pass(move |_| Box::new(matches::Matches::new(msrv())));
    store.register_early_pass(move || Box::new(manual_non_exhaustive::ManualNonExhaustiveStruct::new(msrv())));
    store.register_late_pass(move |_| Box::new(manual_non_exhaustive::ManualNonExhaustiveEnum::new(msrv())));
    store.register_late_pass(move |_| Box::new(manual_strip::ManualStrip::new(msrv())));
    store.register_early_pass(move || Box::new(redundant_static_lifetimes::RedundantStaticLifetimes::new(msrv())));
    store.register_early_pass(move || Box::new(redundant_field_names::RedundantFieldNames::new(msrv())));
    store.register_late_pass(move |_| Box::new(checked_conversions::CheckedConversions::new(msrv())));
    store.register_late_pass(move |_| Box::new(mem_replace::MemReplace::new(msrv())));
    store.register_late_pass(move |_| Box::new(ranges::Ranges::new(msrv())));
    store.register_late_pass(move |_| Box::new(from_over_into::FromOverInto::new(msrv())));
    store.register_late_pass(move |_| Box::new(use_self::UseSelf::new(msrv())));
    store.register_late_pass(move |_| Box::new(missing_const_for_fn::MissingConstForFn::new(msrv())));
    store.register_late_pass(move |_| Box::new(needless_question_mark::NeedlessQuestionMark));
    store.register_late_pass(move |_| Box::new(casts::Casts::new(msrv())));
    store.register_early_pass(move || Box::new(unnested_or_patterns::UnnestedOrPatterns::new(msrv())));
    store.register_late_pass(|_| Box::new(size_of_in_element_count::SizeOfInElementCount));
    store.register_late_pass(|_| Box::new(same_name_method::SameNameMethod));
    store.register_late_pass(move |_| {
        Box::new(index_refutable_slice::IndexRefutableSlice::new(
            max_suggested_slice_pattern_length,
            msrv(),
        ))
    });
    store.register_late_pass(move |_| Box::new(loops::Loops::new(msrv(), enforce_iter_loop_reborrow)));
    store.register_late_pass(move |_| Box::new(transmute::Transmute::new(msrv())));
    store.register_late_pass(move |_| {
        Box::new(cognitive_complexity::CognitiveComplexity::new(
            cognitive_complexity_threshold,
        ))
    });
    store.register_late_pass(move |_| Box::new(escape::BoxedLocal { too_large_for_stack }));
    store.register_late_pass(move |_| {
        Box::new(vec::UselessVec {
            too_large_for_stack,
            msrv: msrv(),
            span_to_lint_map: BTreeMap::new(),
            allow_in_test: allow_useless_vec_in_tests,
        })
    });
    store.register_late_pass(move |_| Box::new(panic_unimplemented::PanicUnimplemented { allow_panic_in_tests }));
    store.register_late_pass(move |_| Box::new(derivable_impls::DerivableImpls::new(msrv())));
    store.register_late_pass(move |_| Box::new(copies::CopyAndPaste::new(ignore_interior_mutability.clone())));
    let format_args = format_args_storage.clone();
    store.register_late_pass(move |_| Box::new(format::UselessFormat::new(format_args.clone())));
    store.register_late_pass(move |_| Box::new(disallowed_names::DisallowedNames::new(disallowed_names)));
    store.register_late_pass(move |_| {
        Box::new(functions::Functions::new(
            too_many_arguments_threshold,
            too_many_lines_threshold,
            large_error_threshold,
            avoid_breaking_exported_api,
            allow_renamed_params_for.clone(),
        ))
    });
    store.register_late_pass(move |_| Box::new(doc::Documentation::new(doc_valid_idents, check_private_items)));
    store.register_late_pass(move |_| Box::new(missing_doc::MissingDoc::new(missing_docs_in_crate_items)));
    store.register_late_pass(move |_| Box::new(exhaustive_items::ExhaustiveItems));
    store.register_late_pass(move |_| Box::new(large_enum_variant::LargeEnumVariant::new(enum_variant_size_threshold)));
    let format_args = format_args_storage.clone();
    store.register_late_pass(move |_| Box::new(explicit_write::ExplicitWrite::new(format_args.clone())));
    store.register_late_pass(move |tcx| {
        Box::new(pass_by_ref_or_value::PassByRefOrValue::new(
            trivial_copy_size_limit,
            pass_by_value_size_limit,
            avoid_breaking_exported_api,
            tcx.sess.target.pointer_width,
        ))
    });
    store.register_late_pass(move |_| Box::new(question_mark::QuestionMark::new(msrv(), matches_for_let_else)));
    store.register_late_pass(move |_| {
        Box::new(indexing_slicing::IndexingSlicing::new(
            suppress_restriction_lint_in_const,
        ))
    });
    store.register_late_pass(move |_| Box::new(non_copy_const::NonCopyConst::new(ignore_interior_mutability.clone())));
    store.register_late_pass(move |_| Box::new(unnecessary_wraps::UnnecessaryWraps::new(avoid_breaking_exported_api)));
    store.register_late_pass(move |_| Box::new(trait_bounds::TraitBounds::new(max_trait_bounds, msrv())));
    store.register_late_pass(move |_| Box::new(mut_key::MutableKeyType::new(ignore_interior_mutability.clone())));
    let format_args = format_args_storage.clone();
    store.register_late_pass(move |_| Box::new(format_impl::FormatImpl::new(format_args.clone())));
    store.register_early_pass(move || {
        Box::new(literal_representation::LiteralDigitGrouping::new(
            unreadable_literal_lint_fractions,
        ))
    });
    store.register_early_pass(move || {
        Box::new(literal_representation::DecimalLiteralRepresentation::new(
            literal_representation_threshold,
        ))
    });
    store.register_late_pass(move |_| {
        Box::new(item_name_repetitions::ItemNameRepetitions::new(
            enum_variant_name_threshold,
            struct_field_name_threshold,
            avoid_breaking_exported_api,
            allow_private_module_inception,
            allowed_prefixes,
        ))
    });
    store.register_late_pass(move |_| {
        Box::new(upper_case_acronyms::UpperCaseAcronyms::new(
            avoid_breaking_exported_api,
            upper_case_acronyms_aggressive,
        ))
    });
    store.register_late_pass(move |_| Box::new(unused_self::UnusedSelf::new(avoid_breaking_exported_api)));
    store.register_late_pass(move |_| Box::new(large_stack_arrays::LargeStackArrays::new(array_size_threshold.into())));
    store.register_late_pass(move |_| Box::new(large_const_arrays::LargeConstArrays::new(array_size_threshold.into())));
    store.register_late_pass(move |_| {
        Box::new(excessive_bools::ExcessiveBools::new(
            max_struct_bools,
            max_fn_params_bools,
        ))
    });
    store.register_late_pass(move |_| {
        Box::new(wildcard_imports::WildcardImports::new(
            warn_on_all_wildcard_imports,
            allowed_wildcard_imports.clone(),
        ))
    });
    store.register_late_pass(|_| Box::<dereference::Dereferencing<'_>>::default());
    store.register_late_pass(move |_| Box::new(large_futures::LargeFuture::new(future_size_threshold)));
    store.register_early_pass(move || {
        Box::new(non_expressive_names::NonExpressiveNames {
            single_char_binding_names_threshold,
        })
    });
    store.register_early_pass(move || Box::new(nonstandard_macro_braces::MacroBraces::new(standard_macro_braces)));
    store.register_late_pass(move |_| Box::new(disallowed_macros::DisallowedMacros::new(disallowed_macros.clone())));
    store.register_late_pass(move |_| Box::new(disallowed_methods::DisallowedMethods::new(disallowed_methods.clone())));
    store.register_late_pass(move |_| Box::new(if_then_some_else_none::IfThenSomeElseNone::new(msrv())));
    store.register_early_pass(move || Box::new(module_style::ModStyle));
    store.register_late_pass(move |_| Box::new(disallowed_types::DisallowedTypes::new(disallowed_types.clone())));
    store.register_late_pass(move |_| {
        Box::new(missing_enforced_import_rename::ImportRename::new(
            enforced_import_renames.clone(),
        ))
    });
    store.register_early_pass(move || Box::new(disallowed_script_idents::DisallowedScriptIdents::new(allowed_scripts)));
    store.register_late_pass(move |_| Box::new(self_named_constructors::SelfNamedConstructors));
    store.register_late_pass(move |_| Box::new(iter_not_returning_iterator::IterNotReturningIterator));
    store.register_late_pass(move |_| Box::new(manual_assert::ManualAssert));
    store.register_late_pass(move |_| {
        Box::new(non_send_fields_in_send_ty::NonSendFieldInSendTy::new(
            enable_raw_pointer_heuristic_for_send,
        ))
    });
    store.register_late_pass(move |_| {
        Box::new(undocumented_unsafe_blocks::UndocumentedUnsafeBlocks::new(
            accept_comment_above_statement,
            accept_comment_above_attributes,
        ))
    });
    let format_args = format_args_storage.clone();
    store.register_late_pass(move |_| {
        Box::new(format_args::FormatArgs::new(
            format_args.clone(),
            msrv(),
            allow_mixed_uninlined_format_args,
        ))
    });
    store.register_late_pass(move |_| Box::new(manual_bits::ManualBits::new(msrv())));
    store.register_late_pass(move |_| Box::new(dbg_macro::DbgMacro::new(allow_dbg_in_tests)));
    let format_args = format_args_storage.clone();
    store.register_late_pass(move |_| Box::new(write::Write::new(format_args.clone(), allow_print_in_tests)));
    store.register_late_pass(move |_| {
        Box::new(cargo::Cargo {
            ignore_publish: cargo_ignore_publish,
            allowed_duplicate_crates: allowed_duplicate_crates.clone(),
        })
    });
    store.register_late_pass(move |_| Box::new(large_include_file::LargeIncludeFile::new(max_include_file_size)));
    store.register_early_pass(move || Box::new(almost_complete_range::AlmostCompleteRange::new(msrv())));
    store.register_late_pass(move |_| Box::new(manual_rem_euclid::ManualRemEuclid::new(msrv())));
    store.register_late_pass(move |_| Box::new(manual_retain::ManualRetain::new(msrv())));
    store.register_late_pass(move |_| {
        Box::new(operators::Operators::new(
            verbose_bit_mask_threshold,
            allow_comparison_to_zero,
        ))
    });
    store.register_late_pass(move |_| Box::new(instant_subtraction::InstantSubtraction::new(msrv())));
    store.register_late_pass(move |_| Box::new(manual_clamp::ManualClamp::new(msrv())));
    store.register_late_pass(move |_| Box::new(manual_is_ascii_check::ManualIsAsciiCheck::new(msrv())));
    store.register_late_pass(move |_| {
        Box::new(semicolon_block::SemicolonBlock::new(
            semicolon_inside_block_ignore_singleline,
            semicolon_outside_block_ignore_multiline,
        ))
    });
    store.register_late_pass(move |_| {
        Box::new(extra_unused_type_parameters::ExtraUnusedTypeParameters::new(
            avoid_breaking_exported_api,
        ))
    });
    store.register_late_pass(move |_| Box::new(manual_main_separator_str::ManualMainSeparatorStr::new(msrv())));
    store.register_late_pass(move |_| {
        Box::new(unnecessary_box_returns::UnnecessaryBoxReturns::new(
            avoid_breaking_exported_api,
            unnecessary_box_size,
        ))
    });
    store.register_early_pass(move || {
        Box::new(excessive_nesting::ExcessiveNesting {
            excessive_nesting_threshold,
            nodes: rustc_ast::node_id::NodeSet::new(),
        })
    });
    store.register_late_pass(move |_| {
        Box::new(min_ident_chars::MinIdentChars {
            allowed_idents_below_min_chars: allowed_idents_below_min_chars.clone(),
            min_ident_chars_threshold,
        })
    });
    store.register_late_pass(move |_| Box::new(large_stack_frames::LargeStackFrames::new(stack_size_threshold)));
    store.register_late_pass(move |_| {
        Box::new(needless_pass_by_ref_mut::NeedlessPassByRefMut::new(
            avoid_breaking_exported_api,
        ))
    });
    store.register_late_pass(move |_| {
        Box::new(single_call_fn::SingleCallFn {
            avoid_breaking_exported_api,
            def_id_to_usage: rustc_data_structures::fx::FxIndexMap::default(),
        })
    });
    store.register_early_pass(move || {
        Box::new(raw_strings::RawStrings {
            allow_one_hash_in_raw_strings,
        })
    });
    store.register_late_pass(move |_| Box::new(legacy_numeric_constants::LegacyNumericConstants::new(msrv())));
    store.register_late_pass(move |_| Box::new(tuple_array_conversions::TupleArrayConversions { msrv: msrv() }));
    store.register_late_pass(move |_| {
        Box::new(absolute_paths::AbsolutePaths {
            absolute_paths_max_segments,
            absolute_paths_allowed_crates: absolute_paths_allowed_crates.clone(),
        })
    });
    store.register_late_pass(move |_| {
        Box::new(needless_borrows_for_generic_args::NeedlessBorrowsForGenericArgs::new(
            msrv(),
        ))
    });
    store.register_late_pass(move |_| Box::new(manual_hash_one::ManualHashOne::new(msrv())));
    store.register_late_pass(|_| Box::<unconditional_recursion::UnconditionalRecursion>::default());
    store.register_late_pass(move |_| {
        Box::new(pub_underscore_fields::PubUnderscoreFields {
            behavior: pub_underscore_fields_behavior,
        })
    });
    store.register_late_pass(move |_| {
        Box::new(thread_local_initializer_can_be_made_const::ThreadLocalInitializerCanBeMadeConst::new(msrv()))
    });
    store.register_late_pass(move |_| Box::new(incompatible_msrv::IncompatibleMsrv::new(msrv())));
    store.register_late_pass(move |_| Box::new(assigning_clones::AssigningClones::new(msrv())));
    store.register_late_pass(move |_| {
        Box::new(macro_metavars_in_unsafe::ExprMetavarsInUnsafe {
            warn_unsafe_macro_metavars_in_private_macros,
            ..Default::default()
        })
    });
    // add lints here, do not remove this comment, it's used in `new_lint`
}

#[rustfmt::skip]
fn register_removed_non_tool_lints(store: &mut rustc_lint::LintStore) {
    store.register_removed(
        "should_assert_eq",
        "`assert!()` will be more flexible with RFC 2011",
    );
    store.register_removed(
        "extend_from_slice",
        "`.extend_from_slice(_)` is a faster way to extend a Vec by a slice",
    );
    store.register_removed(
        "range_step_by_zero",
        "`iterator.step_by(0)` panics nowadays",
    );
    store.register_removed(
        "unstable_as_slice",
        "`Vec::as_slice` has been stabilized in 1.7",
    );
    store.register_removed(
        "unstable_as_mut_slice",
        "`Vec::as_mut_slice` has been stabilized in 1.7",
    );
    store.register_removed(
        "misaligned_transmute",
        "this lint has been split into cast_ptr_alignment and transmute_ptr_to_ptr",
    );
    store.register_removed(
        "assign_ops",
        "using compound assignment operators (e.g., `+=`) is harmless",
    );
    store.register_removed(
        "if_let_redundant_pattern_matching",
        "this lint has been changed to redundant_pattern_matching",
    );
    store.register_removed(
        "unsafe_vector_initialization",
        "the replacement suggested by this lint had substantially different behavior",
    );
    store.register_removed(
        "reverse_range_loop",
        "this lint is now included in reversed_empty_ranges",
    );
}

/// Register renamed lints.
///
/// Used in `./src/driver.rs`.
pub fn register_renamed(ls: &mut rustc_lint::LintStore) {
    for (old_name, new_name) in renamed_lints::RENAMED_LINTS {
        ls.register_renamed(old_name, new_name);
    }
}

// only exists to let the dogfood integration test works.
// Don't run clippy as an executable directly
#[allow(dead_code)]
fn main() {
    panic!("Please use the cargo-clippy executable");
}
