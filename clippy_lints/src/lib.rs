#![feature(array_windows)]
#![feature(binary_heap_into_iter_sorted)]
#![feature(box_patterns)]
#![feature(control_flow_enum)]
#![feature(f128)]
#![feature(f16)]
#![feature(if_let_guard)]
#![feature(iter_intersperse)]
#![feature(iter_partition_in_place)]
#![feature(let_chains)]
#![feature(never_type)]
#![feature(rustc_private)]
#![feature(stmt_expr_attributes)]
#![feature(unwrap_infallible)]
#![recursion_limit = "512"]
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

#[cfg_attr(feature = "internal", allow(clippy::missing_clippy_version_attribute))]
mod utils;

mod declared_lints;
mod deprecated_lints;

// begin lints modules, do not remove this comment, it’s used in `update_lints`
mod absolute_paths;
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
mod byte_char_slices;
mod cargo;
mod casts;
mod cfg_not_test;
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
mod field_scoped_visibility_modifiers;
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
mod manual_rotate;
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
mod missing_const_for_thread_local;
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
mod panic_in_result_fn;
mod panic_unimplemented;
mod panicking_overflow_checks;
mod partial_pub_fields;
mod partialeq_ne_impl;
mod partialeq_to_none;
mod pass_by_ref_or_value;
mod pathbuf_init_then_push;
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
mod set_contains_or_insert;
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
mod string_patterns;
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
mod unused_result_ok;
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

use absolute_paths::AbsolutePaths;
use almost_complete_range::AlmostCompleteRange;
use approx_const::ApproxConstant;
use arc_with_non_send_sync::ArcWithNonSendSync;
use as_conversions::AsConversions;
use asm_syntax::{InlineAsmX86AttSyntax, InlineAsmX86IntelSyntax};
use assertions_on_constants::AssertionsOnConstants;
use assertions_on_result_states::AssertionsOnResultStates;
use assigning_clones::AssigningClones;
use async_yields_async::AsyncYieldsAsync;
use attrs::Attributes;
use await_holding_invalid::AwaitHolding;
use blocks_in_conditions::BlocksInConditions;
use bool_assert_comparison::BoolAssertComparison;
use bool_to_int_with_if::BoolToIntWithIf;
use booleans::NonminimalBool;
use borrow_deref_ref::BorrowDerefRef;
use box_default::BoxDefault;
use byte_char_slices::ByteCharSlice;
use cargo::Cargo;
use casts::Casts;
use cfg_not_test::CfgNotTest;
use checked_conversions::CheckedConversions;
use cognitive_complexity::CognitiveComplexity;
use collapsible_if::CollapsibleIf;
use collection_is_never_read::CollectionIsNeverRead;
use comparison_chain::ComparisonChain;
use copies::CopyAndPaste;
use copy_iterator::CopyIterator;
use crate_in_macro_def::CrateInMacroDef;
use create_dir::CreateDir;
use dbg_macro::DbgMacro;
use default::Default;
use default_constructed_unit_structs::DefaultConstructedUnitStructs;
use default_instead_of_iter_empty::DefaultIterEmpty;
use default_numeric_fallback::DefaultNumericFallback;
use default_union_representation::DefaultUnionRepresentation;
use dereference::Dereferencing;
use derivable_impls::DerivableImpls;
use derive::Derive;
use disallowed_macros::DisallowedMacros;
use disallowed_methods::DisallowedMethods;
use disallowed_names::DisallowedNames;
use disallowed_script_idents::DisallowedScriptIdents;
use disallowed_types::DisallowedTypes;
use doc::Documentation;
use double_parens::DoubleParens;
use drop_forget_ref::DropForgetRef;
use duplicate_mod::DuplicateMod;
use else_if_without_else::ElseIfWithoutElse;
use empty_drop::EmptyDrop;
use empty_enum::EmptyEnum;
use empty_with_brackets::EmptyWithBrackets;
use endian_bytes::EndianBytes;
use entry::HashMapPass;
use enum_clike::UnportableVariant;
use equatable_if_let::PatternEquality;
use error_impl_error::ErrorImplError;
use escape::BoxedLocal;
use eta_reduction::EtaReduction;
use excessive_bools::ExcessiveBools;
use excessive_nesting::ExcessiveNesting;
use exhaustive_items::ExhaustiveItems;
use exit::Exit;
use explicit_write::ExplicitWrite;
use extra_unused_type_parameters::ExtraUnusedTypeParameters;
use fallible_impl_from::FallibleImplFrom;
use field_scoped_visibility_modifiers::FieldScopedVisibilityModifiers;
use float_literal::FloatLiteral;
use floating_point_arithmetic::FloatingPointArithmetic;
use format::UselessFormat;
use format_args::FormatArgs;
use format_impl::FormatImpl;
use format_push_string::FormatPushString;
use formatting::Formatting;
use four_forward_slashes::FourForwardSlashes;
use from_over_into::FromOverInto;
use from_raw_with_void_ptr::FromRawWithVoidPtr;
use from_str_radix_10::FromStrRadix10;
use functions::Functions;
use future_not_send::FutureNotSend;
use if_let_mutex::IfLetMutex;
use if_not_else::IfNotElse;
use if_then_some_else_none::IfThenSomeElseNone;
use ignored_unit_patterns::IgnoredUnitPatterns;
use impl_hash_with_borrow_str_and_bytes::ImplHashWithBorrowStrBytes;
use implicit_hasher::ImplicitHasher;
use implicit_return::ImplicitReturn;
use implicit_saturating_add::ImplicitSaturatingAdd;
use implicit_saturating_sub::ImplicitSaturatingSub;
use implied_bounds_in_impls::ImpliedBoundsInImpls;
use incompatible_msrv::IncompatibleMsrv;
use inconsistent_struct_constructor::InconsistentStructConstructor;
use index_refutable_slice::IndexRefutableSlice;
use indexing_slicing::IndexingSlicing;
use ineffective_open_options::IneffectiveOpenOptions;
use infinite_iter::InfiniteIter;
use inherent_impl::MultipleInherentImpl;
use inherent_to_string::InherentToString;
use init_numbered_fields::NumberedFields;
use inline_fn_without_body::InlineFnWithoutBody;
use instant_subtraction::InstantSubtraction;
use int_plus_one::IntPlusOne;
use integer_division_remainder_used::IntegerDivisionRemainderUsed;
use invalid_upcast_comparisons::InvalidUpcastComparisons;
use item_name_repetitions::ItemNameRepetitions;
use items_after_statements::ItemsAfterStatements;
use items_after_test_module::ItemsAfterTestModule;
use iter_not_returning_iterator::IterNotReturningIterator;
use iter_over_hash_type::IterOverHashType;
use iter_without_into_iter::IterWithoutIntoIter;
use large_const_arrays::LargeConstArrays;
use large_enum_variant::LargeEnumVariant;
use large_futures::LargeFuture;
use large_include_file::LargeIncludeFile;
use large_stack_arrays::LargeStackArrays;
use large_stack_frames::LargeStackFrames;
use legacy_numeric_constants::LegacyNumericConstants;
use len_zero::LenZero;
use let_if_seq::LetIfSeq;
use let_underscore::LetUnderscore;
use let_with_type_underscore::UnderscoreTyped;
use lifetimes::Lifetimes;
use lines_filter_map_ok::LinesFilterMapOk;
use literal_representation::{DecimalLiteralRepresentation, LiteralDigitGrouping};
use loops::Loops;
use macro_metavars_in_unsafe::ExprMetavarsInUnsafe;
use macro_use::MacroUseImports;
use main_recursion::MainRecursion;
use manual_assert::ManualAssert;
use manual_async_fn::ManualAsyncFn;
use manual_bits::ManualBits;
use manual_clamp::ManualClamp;
use manual_float_methods::ManualFloatMethods;
use manual_hash_one::ManualHashOne;
use manual_is_ascii_check::ManualIsAsciiCheck;
use manual_main_separator_str::ManualMainSeparatorStr;
use manual_non_exhaustive::{ManualNonExhaustiveEnum, ManualNonExhaustiveStruct};
use manual_range_patterns::ManualRangePatterns;
use manual_rem_euclid::ManualRemEuclid;
use manual_retain::ManualRetain;
use manual_rotate::ManualRotate;
use manual_slice_size_calculation::ManualSliceSizeCalculation;
use manual_string_new::ManualStringNew;
use manual_strip::ManualStrip;
use manual_unwrap_or_default::ManualUnwrapOrDefault;
use map_unit_fn::MapUnit;
use match_result_ok::MatchResultOk;
use matches::Matches;
use mem_replace::MemReplace;
use methods::Methods;
use min_ident_chars::MinIdentChars;
use minmax::MinMaxPass;
use misc::LintPass;
use misc_early::MiscEarlyLints;
use mismatching_type_param_order::TypeParamMismatch;
use missing_assert_message::MissingAssertMessage;
use missing_asserts_for_indexing::MissingAssertsForIndexing;
use missing_const_for_fn::MissingConstForFn;
use missing_const_for_thread_local::MissingConstForThreadLocal;
use missing_doc::MissingDoc;
use missing_enforced_import_rename::ImportRename;
use missing_fields_in_debug::MissingFieldsInDebug;
use missing_inline::MissingInline;
use missing_trait_methods::MissingTraitMethods;
use mixed_read_write_in_expression::EvalOrderDependence;
use module_style::ModStyle;
use multi_assignments::MultiAssignments;
use multiple_bound_locations::MultipleBoundLocations;
use multiple_unsafe_ops_per_block::MultipleUnsafeOpsPerBlock;
use mut_key::MutableKeyType;
use mut_mut::MutMut;
use mut_reference::UnnecessaryMutPassed;
use mutable_debug_assertion::DebugAssertWithMutCall;
use mutex_atomic::Mutex;
use needless_arbitrary_self_type::NeedlessArbitrarySelfType;
use needless_bool::{BoolComparison, NeedlessBool};
use needless_borrowed_ref::NeedlessBorrowedRef;
use needless_borrows_for_generic_args::NeedlessBorrowsForGenericArgs;
use needless_continue::NeedlessContinue;
use needless_else::NeedlessElse;
use needless_for_each::NeedlessForEach;
use needless_if::NeedlessIf;
use needless_late_init::NeedlessLateInit;
use needless_maybe_sized::NeedlessMaybeSized;
use needless_parens_on_range_literals::NeedlessParensOnRangeLiterals;
use needless_pass_by_ref_mut::NeedlessPassByRefMut;
use needless_pass_by_value::NeedlessPassByValue;
use needless_question_mark::NeedlessQuestionMark;
use needless_update::NeedlessUpdate;
use neg_cmp_op_on_partial_ord::NoNegCompOpForPartialOrd;
use neg_multiply::NegMultiply;
use new_without_default::NewWithoutDefault;
use no_effect::NoEffect;
use no_mangle_with_rust_abi::NoMangleWithRustAbi;
use non_canonical_impls::NonCanonicalImpls;
use non_copy_const::NonCopyConst;
use non_expressive_names::NonExpressiveNames;
use non_octal_unix_permissions::NonOctalUnixPermissions;
use non_send_fields_in_send_ty::NonSendFieldInSendTy;
use nonstandard_macro_braces::MacroBraces;
use octal_escapes::OctalEscapes;
use only_used_in_recursion::OnlyUsedInRecursion;
use operators::arithmetic_side_effects::ArithmeticSideEffects;
use operators::Operators;
use option_env_unwrap::OptionEnvUnwrap;
use option_if_let_else::OptionIfLetElse;
use panic_in_result_fn::PanicInResultFn;
use panic_unimplemented::PanicUnimplemented;
use panicking_overflow_checks::PanickingOverflowChecks;
use partial_pub_fields::PartialPubFields;
use partialeq_ne_impl::PartialEqNeImpl;
use partialeq_to_none::PartialeqToNone;
use pass_by_ref_or_value::PassByRefOrValue;
use pathbuf_init_then_push::PathbufThenPush;
use pattern_type_mismatch::PatternTypeMismatch;
use permissions_set_readonly_false::PermissionsSetReadonlyFalse;
use precedence::Precedence;
use ptr::Ptr;
use ptr_offset_with_cast::PtrOffsetWithCast;
use pub_underscore_fields::PubUnderscoreFields;
use pub_use::PubUse;
use question_mark::QuestionMark;
use question_mark_used::QuestionMarkUsed;
use ranges::Ranges;
use raw_strings::RawStrings;
use rc_clone_in_vec_init::RcCloneInVecInit;
use read_zero_byte_vec::ReadZeroByteVec;
use redundant_async_block::RedundantAsyncBlock;
use redundant_clone::RedundantClone;
use redundant_closure_call::RedundantClosureCall;
use redundant_else::RedundantElse;
use redundant_field_names::RedundantFieldNames;
use redundant_locals::RedundantLocals;
use redundant_pub_crate::RedundantPubCrate;
use redundant_slicing::RedundantSlicing;
use redundant_static_lifetimes::RedundantStaticLifetimes;
use redundant_type_annotations::RedundantTypeAnnotations;
use ref_option_ref::RefOptionRef;
use ref_patterns::RefPatterns;
use reference::DerefAddrOf;
use regex::Regex;
use repeat_vec_with_capacity::RepeatVecWithCapacity;
use reserve_after_initialization::ReserveAfterInitialization;
use return_self_not_must_use::ReturnSelfNotMustUse;
use returns::Return;
use same_name_method::SameNameMethod;
use self_named_constructors::SelfNamedConstructors;
use semicolon_block::SemicolonBlock;
use semicolon_if_nothing_returned::SemicolonIfNothingReturned;
use serde_api::SerdeApi;
use set_contains_or_insert::SetContainsOrInsert;
use shadow::Shadow;
use significant_drop_tightening::SignificantDropTightening;
use single_call_fn::SingleCallFn;
use single_char_lifetime_names::SingleCharLifetimeNames;
use single_component_path_imports::SingleComponentPathImports;
use single_range_in_vec_init::SingleRangeInVecInit;
use size_of_in_element_count::SizeOfInElementCount;
use size_of_ref::SizeOfRef;
use slow_vector_initialization::SlowVectorInit;
use std_instead_of_core::StdReexports;
use string_patterns::StringPatterns;
use strings::{StrToString, StringAdd, StringLitAsBytes, StringToString, TrimSplitWhitespace};
use strlen_on_c_strings::StrlenOnCStrings;
use suspicious_operation_groupings::SuspiciousOperationGroupings;
use suspicious_trait_impl::SuspiciousImpl;
use suspicious_xor_used_as_pow::ConfusingXorAndPow;
use swap::Swap;
use swap_ptr_to_ref::SwapPtrToRef;
use tabs_in_doc_comments::TabsInDocComments;
use temporary_assignment::TemporaryAssignment;
use tests_outside_test_module::TestsOutsideTestModule;
use to_digit_is_some::ToDigitIsSome;
use to_string_trait_impl::ToStringTraitImpl;
use trailing_empty_array::TrailingEmptyArray;
use trait_bounds::TraitBounds;
use transmute::Transmute;
use tuple_array_conversions::TupleArrayConversions;
use types::Types;
use unconditional_recursion::UnconditionalRecursion;
use undocumented_unsafe_blocks::UndocumentedUnsafeBlocks;
use unicode::Unicode;
use uninhabited_references::UninhabitedReferences;
use uninit_vec::UninitVec;
use unit_return_expecting_ord::UnitReturnExpectingOrd;
use unit_types::UnitTypes;
use unnamed_address::UnnamedAddress;
use unnecessary_box_returns::UnnecessaryBoxReturns;
use unnecessary_map_on_constructor::UnnecessaryMapOnConstructor;
use unnecessary_owned_empty_strings::UnnecessaryOwnedEmptyStrings;
use unnecessary_self_imports::UnnecessarySelfImports;
use unnecessary_struct_initialization::UnnecessaryStruct;
use unnecessary_wraps::UnnecessaryWraps;
use unnested_or_patterns::UnnestedOrPatterns;
use unsafe_removed_from_name::UnsafeNameRemoval;
use unused_async::UnusedAsync;
use unused_io_amount::UnusedIoAmount;
use unused_peekable::UnusedPeekable;
use unused_result_ok::UnusedResultOk;
use unused_rounding::UnusedRounding;
use unused_self::UnusedSelf;
use unused_unit::UnusedUnit;
use unwrap::Unwrap;
use unwrap_in_result::UnwrapInResult;
use upper_case_acronyms::UpperCaseAcronyms;
use use_self::UseSelf;
use useless_conversion::UselessConversion;
use utils::author::Author;
use utils::dump_hir::DumpHir;
use utils::format_args_collector::FormatArgsCollector;
use vec::UselessVec;
use vec_init_then_push::VecInitThenPush;
use visibility::Visibility;
use wildcard_imports::WildcardImports;
use write::Write;
use zero_div_zero::ZeroDiv;
use zero_repeat_side_effects::ZeroRepeatSideEffects;
use zero_sized_map_values::ZeroSizedMapValues;

macro_rules! declare_early_pass {
    (
        [$v:vis $name:ident($($ctor_args:tt)*), [$(
            $pass:ident: $ctor:expr,
        )*]],
        $methods:tt
    ) => {
        #[allow(non_snake_case)]
        $v struct $name {
            $($pass: $pass,)*
        }

        impl $name {
            $v fn new($($ctor_args)*) -> Self {
                Self {
                    $($pass: $ctor,)*
                }
            }

            // $v fn get_lints() -> rustc_lint::LintVec {
            //     let mut lints = Vec::new();
            //     $(lints.extend_from_slice(&$pass::get_lints());)*
            //     lints
            // }
        }

        impl rustc_lint::EarlyLintPass for $name {
            rustc_lint::expand_combined_early_lint_pass_methods!([$($pass),*], $methods);
        }

        #[allow(rustc::lint_pass_impl_without_macro)]
        impl rustc_lint::LintPass for $name {
            fn name(&self) -> &'static str {
                panic!()
            }
        }
    }
}

macro_rules! declare_late_pass {
    (
        [$v:vis $name:ident<$tcx:lifetime>($($ctor_args:tt)*), [$(
            $pass:ident$(<$pass_lt:lifetime>)?: $ctor:expr,
        )*]],
        $methods:tt
    ) => {
        #[allow(non_snake_case)]
        $v struct $name<$tcx> {
            $($pass: $pass$(<$pass_lt>)?,)*
            __marker: core::marker::PhantomData<&'tcx ()>,
        }

        impl<$tcx> $name<$tcx> {
            $v fn new($($ctor_args)*) -> Self {
                Self {
                    $($pass: $ctor,)*
                    __marker: core::marker::PhantomData,
                }
            }

            // $v fn get_lints() -> rustc_lint::LintVec {
            //     let mut lints = Vec::new();
            //     $(lints.extend_from_slice(&$pass::get_lints());)*
            //     lints
            // }
        }

        impl<$tcx> rustc_lint::LateLintPass<$tcx> for $name<$tcx> {
            rustc_lint::expand_combined_late_lint_pass_methods!([$($pass),*], $methods);
        }

        #[allow(rustc::lint_pass_impl_without_macro)]
        impl rustc_lint::LintPass for $name<'_> {
            fn name(&self) -> &'static str {
                panic!()
            }
        }
    }
}

rustc_lint::early_lint_methods!(
    declare_early_pass,
    [
        ClippyEarlyLintPass(conf: &'static Conf, format_args: &FormatArgsStorage),
        [
            AlmostCompleteRange: AlmostCompleteRange::new(conf),
            ByteCharSlice: ByteCharSlice,
            CfgNotTest: CfgNotTest,
            CollapsibleIf: CollapsibleIf,
            CrateInMacroDef: CrateInMacroDef,
            DecimalLiteralRepresentation: DecimalLiteralRepresentation::new(conf),
            DerefAddrOf: DerefAddrOf,
            DisallowedScriptIdents: DisallowedScriptIdents::new(conf),
            DoubleParens: DoubleParens,
            DuplicateMod: DuplicateMod::default(),
            ElseIfWithoutElse: ElseIfWithoutElse,
            EmptyWithBrackets: EmptyWithBrackets,
            ExcessiveNesting: ExcessiveNesting::new(conf),
            FieldScopedVisibilityModifiers: FieldScopedVisibilityModifiers,
            FormatArgsCollector: FormatArgsCollector::new(format_args.clone()),
            Formatting: Formatting,
            InlineAsmX86AttSyntax: InlineAsmX86AttSyntax,
            InlineAsmX86IntelSyntax: InlineAsmX86IntelSyntax,
            IntPlusOne: IntPlusOne,
            LiteralDigitGrouping: LiteralDigitGrouping::new(conf),
            MacroBraces: MacroBraces::new(conf),
            ManualNonExhaustiveStruct: ManualNonExhaustiveStruct::new(conf),
            MiscEarlyLints: MiscEarlyLints,
            ModStyle: ModStyle,
            MultiAssignments: MultiAssignments,
            MultipleBoundLocations: MultipleBoundLocations,
            NeedlessArbitrarySelfType: NeedlessArbitrarySelfType,
            NeedlessContinue: NeedlessContinue,
            NeedlessElse: NeedlessElse,
            NonExpressiveNames: NonExpressiveNames::new(conf),
            OctalEscapes: OctalEscapes,
            OptionEnvUnwrap: OptionEnvUnwrap,
            PartialPubFields: PartialPubFields,
            Precedence: Precedence,
            PubUse: PubUse,
            RawStrings: RawStrings::new(conf),
            RedundantElse: RedundantElse,
            RedundantFieldNames: RedundantFieldNames::new(conf),
            RedundantStaticLifetimes: RedundantStaticLifetimes::new(conf),
            RefPatterns: RefPatterns,
            SingleCharLifetimeNames: SingleCharLifetimeNames,
            SingleComponentPathImports: SingleComponentPathImports::default(),
            SuspiciousOperationGroupings: SuspiciousOperationGroupings,
            TabsInDocComments: TabsInDocComments,
            UnnecessarySelfImports: UnnecessarySelfImports,
            UnnestedOrPatterns: UnnestedOrPatterns::new(conf),
            UnsafeNameRemoval: UnsafeNameRemoval,
            UnusedRounding: UnusedRounding,
            UnusedUnit: UnusedUnit,
            Visibility: Visibility,
        ]
    ]
);

rustc_lint::late_lint_methods!(
    declare_late_pass,
    [
        ClippyLateLintPass<'tcx>(tcx: TyCtxt<'tcx>, conf: &'static Conf, format_args: &FormatArgsStorage),
        [
            AbsolutePaths: AbsolutePaths::new(conf),
            ApproxConstant: ApproxConstant::new(conf),
            ArcWithNonSendSync: ArcWithNonSendSync,
            ArithmeticSideEffects: ArithmeticSideEffects::new(conf),
            AsConversions: AsConversions,
            AssertionsOnConstants: AssertionsOnConstants,
            AssertionsOnResultStates: AssertionsOnResultStates,
            AssigningClones: AssigningClones::new(conf),
            AsyncYieldsAsync: AsyncYieldsAsync,
            Attributes: Attributes::new(conf),
            Author: Author,
            AwaitHolding: AwaitHolding::new(tcx, conf),
            BlocksInConditions: BlocksInConditions,
            BoolAssertComparison: BoolAssertComparison,
            BoolComparison: BoolComparison,
            BoolToIntWithIf: BoolToIntWithIf,
            BorrowDerefRef: BorrowDerefRef,
            BoxDefault: BoxDefault,
            BoxedLocal: BoxedLocal::new(conf),
            Cargo: Cargo::new(conf),
            Casts: Casts::new(conf),
            CheckedConversions: CheckedConversions::new(conf),
            CognitiveComplexity: CognitiveComplexity::new(conf),
            CollectionIsNeverRead: CollectionIsNeverRead,
            ComparisonChain: ComparisonChain,
            ConfusingXorAndPow: ConfusingXorAndPow,
            CopyAndPaste<'tcx>: CopyAndPaste::new(tcx, conf),
            CopyIterator: CopyIterator,
            CreateDir: CreateDir,
            DbgMacro: DbgMacro::new(conf),
            DebugAssertWithMutCall: DebugAssertWithMutCall,
            Default: Default::default(),
            DefaultConstructedUnitStructs: DefaultConstructedUnitStructs,
            DefaultIterEmpty: DefaultIterEmpty,
            DefaultNumericFallback: DefaultNumericFallback,
            DefaultUnionRepresentation: DefaultUnionRepresentation,
            Dereferencing<'tcx>: Dereferencing::default(),
            DerivableImpls: DerivableImpls::new(conf),
            Derive: Derive,
            DisallowedMacros: DisallowedMacros::new(tcx, conf),
            DisallowedMethods: DisallowedMethods::new(tcx, conf),
            DisallowedNames: DisallowedNames::new(conf),
            DisallowedTypes: DisallowedTypes::new(tcx, conf),
            Documentation: Documentation::new(conf),
            DropForgetRef: DropForgetRef,
            DumpHir: DumpHir,
            EmptyDrop: EmptyDrop,
            EmptyEnum: EmptyEnum,
            EndianBytes: EndianBytes,
            ErrorImplError: ErrorImplError,
            EtaReduction: EtaReduction,
            EvalOrderDependence: EvalOrderDependence,
            ExcessiveBools: ExcessiveBools::new(conf),
            ExhaustiveItems: ExhaustiveItems,
            Exit: Exit,
            ExplicitWrite: ExplicitWrite::new(format_args.clone()),
            ExprMetavarsInUnsafe: ExprMetavarsInUnsafe::new(conf),
            ExtraUnusedTypeParameters: ExtraUnusedTypeParameters::new(conf),
            FallibleImplFrom: FallibleImplFrom,
            FloatingPointArithmetic: FloatingPointArithmetic,
            FloatLiteral: FloatLiteral,
            FormatArgs: FormatArgs::new(conf, format_args.clone()),
            FormatImpl: FormatImpl::new(format_args.clone()),
            FormatPushString: FormatPushString,
            FourForwardSlashes: FourForwardSlashes,
            FromOverInto: FromOverInto::new(conf),
            FromRawWithVoidPtr: FromRawWithVoidPtr,
            FromStrRadix10: FromStrRadix10,
            Functions: Functions::new(tcx, conf),
            FutureNotSend: FutureNotSend,
            HashMapPass: HashMapPass,
            IfLetMutex: IfLetMutex,
            IfNotElse: IfNotElse,
            IfThenSomeElseNone: IfThenSomeElseNone::new(conf),
            IgnoredUnitPatterns: IgnoredUnitPatterns,
            ImplHashWithBorrowStrBytes: ImplHashWithBorrowStrBytes,
            ImplicitHasher: ImplicitHasher,
            ImplicitReturn: ImplicitReturn,
            ImplicitSaturatingAdd: ImplicitSaturatingAdd,
            ImplicitSaturatingSub: ImplicitSaturatingSub,
            ImpliedBoundsInImpls: ImpliedBoundsInImpls,
            ImportRename: ImportRename::new(tcx, conf),
            IncompatibleMsrv: IncompatibleMsrv::new(conf),
            InconsistentStructConstructor: InconsistentStructConstructor,
            IndexingSlicing: IndexingSlicing::new(conf),
            IndexRefutableSlice: IndexRefutableSlice::new(conf),
            IneffectiveOpenOptions: IneffectiveOpenOptions,
            InfiniteIter: InfiniteIter,
            InherentToString: InherentToString,
            InlineFnWithoutBody: InlineFnWithoutBody,
            InstantSubtraction: InstantSubtraction::new(conf),
            IntegerDivisionRemainderUsed: IntegerDivisionRemainderUsed,
            InvalidUpcastComparisons: InvalidUpcastComparisons,
            ItemNameRepetitions: ItemNameRepetitions::new(conf),
            ItemsAfterStatements: ItemsAfterStatements,
            ItemsAfterTestModule: ItemsAfterTestModule,
            IterNotReturningIterator: IterNotReturningIterator,
            IterOverHashType: IterOverHashType,
            IterWithoutIntoIter: IterWithoutIntoIter,
            LargeConstArrays: LargeConstArrays::new(conf),
            LargeEnumVariant: LargeEnumVariant::new(conf),
            LargeFuture: LargeFuture::new(conf),
            LargeIncludeFile: LargeIncludeFile::new(conf),
            LargeStackArrays: LargeStackArrays::new(conf),
            LargeStackFrames: LargeStackFrames::new(conf),
            LegacyNumericConstants: LegacyNumericConstants::new(conf),
            LenZero: LenZero,
            LetIfSeq: LetIfSeq,
            LetUnderscore: LetUnderscore,
            Lifetimes: Lifetimes,
            LinesFilterMapOk: LinesFilterMapOk,
            LintPass: LintPass,
            Loops: Loops::new(conf),
            MacroUseImports: MacroUseImports::default(),
            MainRecursion: MainRecursion::default(),
            ManualAssert: ManualAssert,
            ManualAsyncFn: ManualAsyncFn,
            ManualBits: ManualBits::new(conf),
            ManualClamp: ManualClamp::new(conf),
            ManualFloatMethods: ManualFloatMethods,
            ManualHashOne: ManualHashOne::new(conf),
            ManualIsAsciiCheck: ManualIsAsciiCheck::new(conf),
            ManualMainSeparatorStr: ManualMainSeparatorStr::new(conf),
            ManualNonExhaustiveEnum: ManualNonExhaustiveEnum::new(conf),
            ManualRangePatterns: ManualRangePatterns,
            ManualRemEuclid: ManualRemEuclid::new(conf),
            ManualRetain: ManualRetain::new(conf),
            ManualRotate: ManualRotate,
            ManualSliceSizeCalculation: ManualSliceSizeCalculation,
            ManualStringNew: ManualStringNew,
            ManualStrip: ManualStrip::new(conf),
            ManualUnwrapOrDefault: ManualUnwrapOrDefault,
            MapUnit: MapUnit,
            Matches: Matches::new(conf),
            MatchResultOk: MatchResultOk,
            MemReplace: MemReplace::new(conf),
            Methods: Methods::new(conf, format_args.clone()),
            MinIdentChars: MinIdentChars::new(conf),
            MinMaxPass: MinMaxPass,
            MissingAssertMessage: MissingAssertMessage,
            MissingAssertsForIndexing: MissingAssertsForIndexing,
            MissingConstForFn: MissingConstForFn::new(conf),
            MissingConstForThreadLocal: MissingConstForThreadLocal::new(conf),
            MissingDoc: MissingDoc::new(conf),
            MissingFieldsInDebug: MissingFieldsInDebug,
            MissingInline: MissingInline,
            MissingTraitMethods: MissingTraitMethods,
            MultipleInherentImpl: MultipleInherentImpl,
            MultipleUnsafeOpsPerBlock: MultipleUnsafeOpsPerBlock,
            MutableKeyType<'tcx>: MutableKeyType::new(tcx, conf),
            Mutex: Mutex,
            MutMut: MutMut,
            NeedlessBool: NeedlessBool,
            NeedlessBorrowedRef: NeedlessBorrowedRef,
            NeedlessBorrowsForGenericArgs<'tcx>: NeedlessBorrowsForGenericArgs::new(conf),
            NeedlessForEach: NeedlessForEach,
            NeedlessIf: NeedlessIf,
            NeedlessLateInit: NeedlessLateInit,
            NeedlessMaybeSized: NeedlessMaybeSized,
            NeedlessParensOnRangeLiterals: NeedlessParensOnRangeLiterals,
            NeedlessPassByRefMut<'tcx>: NeedlessPassByRefMut::new(conf),
            NeedlessPassByValue: NeedlessPassByValue,
            NeedlessQuestionMark: NeedlessQuestionMark,
            NeedlessUpdate: NeedlessUpdate,
            NegMultiply: NegMultiply,
            NewWithoutDefault: NewWithoutDefault::default(),
            NoEffect: NoEffect::default(),
            NoMangleWithRustAbi: NoMangleWithRustAbi,
            NonCanonicalImpls: NonCanonicalImpls,
            NonCopyConst<'tcx>: NonCopyConst::new(tcx, conf),
            NoNegCompOpForPartialOrd: NoNegCompOpForPartialOrd,
            NonminimalBool: NonminimalBool,
            NonOctalUnixPermissions: NonOctalUnixPermissions,
            NonSendFieldInSendTy: NonSendFieldInSendTy::new(conf),
            NumberedFields: NumberedFields,
            OnlyUsedInRecursion: OnlyUsedInRecursion::default(),
            Operators: Operators::new(conf),
            OptionIfLetElse: OptionIfLetElse,
            PanicInResultFn: PanicInResultFn,
            PanickingOverflowChecks: PanickingOverflowChecks,
            PanicUnimplemented: PanicUnimplemented::new(conf),
            PartialEqNeImpl: PartialEqNeImpl,
            PartialeqToNone: PartialeqToNone,
            PassByRefOrValue: PassByRefOrValue::new(tcx, conf),
            PathbufThenPush<'tcx>: PathbufThenPush::default(),
            PatternEquality: PatternEquality,
            PatternTypeMismatch: PatternTypeMismatch,
            PermissionsSetReadonlyFalse: PermissionsSetReadonlyFalse,
            Ptr: Ptr,
            PtrOffsetWithCast: PtrOffsetWithCast,
            PubUnderscoreFields: PubUnderscoreFields::new(conf),
            QuestionMark: QuestionMark::new(conf),
            QuestionMarkUsed: QuestionMarkUsed,
            Ranges: Ranges::new(conf),
            RcCloneInVecInit: RcCloneInVecInit,
            ReadZeroByteVec: ReadZeroByteVec,
            RedundantAsyncBlock: RedundantAsyncBlock,
            RedundantClone: RedundantClone,
            RedundantClosureCall: RedundantClosureCall,
            RedundantLocals: RedundantLocals,
            RedundantPubCrate: RedundantPubCrate::default(),
            RedundantSlicing: RedundantSlicing,
            RedundantTypeAnnotations: RedundantTypeAnnotations,
            RefOptionRef: RefOptionRef,
            Regex: Regex::default(),
            RepeatVecWithCapacity: RepeatVecWithCapacity,
            ReserveAfterInitialization: ReserveAfterInitialization::default(),
            Return: Return,
            ReturnSelfNotMustUse: ReturnSelfNotMustUse,
            SameNameMethod: SameNameMethod,
            SelfNamedConstructors: SelfNamedConstructors,
            SemicolonBlock: SemicolonBlock::new(conf),
            SemicolonIfNothingReturned: SemicolonIfNothingReturned,
            SerdeApi: SerdeApi,
            SetContainsOrInsert: SetContainsOrInsert,
            Shadow: Shadow::default(),
            SignificantDropTightening<'tcx>: SignificantDropTightening::default(),
            SingleCallFn: SingleCallFn::new(conf),
            SingleRangeInVecInit: SingleRangeInVecInit,
            SizeOfInElementCount: SizeOfInElementCount,
            SizeOfRef: SizeOfRef,
            SlowVectorInit: SlowVectorInit,
            StdReexports: StdReexports::new(conf),
            StringAdd: StringAdd,
            StringLitAsBytes: StringLitAsBytes,
            StringPatterns: StringPatterns::new(conf),
            StringToString: StringToString,
            StrlenOnCStrings: StrlenOnCStrings,
            StrToString: StrToString,
            SuspiciousImpl: SuspiciousImpl,
            Swap: Swap,
            SwapPtrToRef: SwapPtrToRef,
            TemporaryAssignment: TemporaryAssignment,
            TestsOutsideTestModule: TestsOutsideTestModule,
            ToDigitIsSome: ToDigitIsSome,
            ToStringTraitImpl: ToStringTraitImpl,
            TrailingEmptyArray: TrailingEmptyArray,
            TraitBounds: TraitBounds::new(conf),
            Transmute: Transmute::new(conf),
            TrimSplitWhitespace: TrimSplitWhitespace,
            TupleArrayConversions: TupleArrayConversions::new(conf),
            TypeParamMismatch: TypeParamMismatch,
            Types: Types::new(conf),
            UnconditionalRecursion: UnconditionalRecursion::default(),
            UnderscoreTyped: UnderscoreTyped,
            UndocumentedUnsafeBlocks: UndocumentedUnsafeBlocks::new(conf),
            Unicode: Unicode,
            UninhabitedReferences: UninhabitedReferences,
            UninitVec: UninitVec,
            UnitReturnExpectingOrd: UnitReturnExpectingOrd,
            UnitTypes: UnitTypes,
            UnnamedAddress: UnnamedAddress,
            UnnecessaryBoxReturns: UnnecessaryBoxReturns::new(conf),
            UnnecessaryMapOnConstructor: UnnecessaryMapOnConstructor,
            UnnecessaryMutPassed: UnnecessaryMutPassed,
            UnnecessaryOwnedEmptyStrings: UnnecessaryOwnedEmptyStrings,
            UnnecessaryStruct: UnnecessaryStruct,
            UnnecessaryWraps: UnnecessaryWraps::new(conf),
            UnportableVariant: UnportableVariant,
            UnusedAsync: UnusedAsync::default(),
            UnusedIoAmount: UnusedIoAmount,
            UnusedPeekable: UnusedPeekable,
            UnusedResultOk: UnusedResultOk,
            UnusedSelf: UnusedSelf::new(conf),
            Unwrap: Unwrap,
            UnwrapInResult: UnwrapInResult,
            UpperCaseAcronyms: UpperCaseAcronyms::new(conf),
            UselessConversion: UselessConversion::default(),
            UselessFormat: UselessFormat::new(format_args.clone()),
            UselessVec: UselessVec::new(conf),
            UseSelf: UseSelf::new(conf),
            VecInitThenPush: VecInitThenPush::default(),
            WildcardImports: WildcardImports::new(conf),
            Write: Write::new(conf, format_args.clone()),
            ZeroDiv: ZeroDiv,
            ZeroRepeatSideEffects: ZeroRepeatSideEffects,
            ZeroSizedMapValues: ZeroSizedMapValues,
        ]
    ]
);

use clippy_config::{get_configuration_metadata, Conf};
use clippy_utils::macros::FormatArgsStorage;
use rustc_data_structures::fx::FxHashSet;
use rustc_lint::{Lint, LintId};
use rustc_middle::ty::TyCtxt;

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
    store.register_pre_expansion_pass(move || Box::new(attrs::EarlyAttributes::new(conf)));
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
        mdconf.retain(|cconf| cconf.lints.contains(&&*name));
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
pub fn register_lints(store: &mut rustc_lint::LintStore, conf: &'static Conf) {
    register_categories(store);

    for (old_name, new_name) in deprecated_lints::RENAMED {
        store.register_renamed(old_name, new_name);
    }
    for (name, reason) in deprecated_lints::DEPRECATED {
        store.register_removed(name, reason);
    }

    #[cfg(feature = "internal")]
    {
        if std::env::var("ENABLE_METADATA_COLLECTION").eq(&Ok("1".to_string())) {
            store.register_late_pass(|_| Box::new(utils::internal_lints::metadata_collector::MetadataCollector::new()));
            return;
        }
    }

    // all the internal lints
    #[cfg(feature = "internal")]
    {
        store.register_early_pass(|| {
            Box::new(utils::internal_lints::unsorted_clippy_utils_paths::UnsortedClippyUtilsPaths)
        });
        store.register_early_pass(|| Box::new(utils::internal_lints::produce_ice::ProduceIce));
        store.register_late_pass(|_| Box::new(utils::internal_lints::collapsible_calls::CollapsibleCalls));
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

    let format_args1 = FormatArgsStorage::default();
    let format_args2 = format_args1.clone();

    store.register_early_pass(move || Box::new(ClippyEarlyLintPass::new(conf, &format_args1)));
    store.register_late_pass(move |tcx| Box::new(ClippyLateLintPass::new(tcx, conf, &format_args2)));

    // add lints here, do not remove this comment, it's used in `new_lint`
}
