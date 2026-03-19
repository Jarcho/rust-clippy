#![feature(
    bool_to_result,
    closure_track_caller,
    exit_status_error,
    macro_metavar_expr_concat,
    new_range,
    new_range_api,
    pattern,
    rustc_private
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![allow(
    clippy::case_sensitive_file_extension_comparisons,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

extern crate rustc_arena;
extern crate rustc_data_structures;
#[expect(unused_extern_crates, reason = "required to link to rustc crates")]
extern crate rustc_driver;
extern crate rustc_lexer;
extern crate termize;

pub mod ir;
pub mod lex;
pub mod utils;

mod check;
mod diag;
mod fmt;
mod generate;
mod parse;

pub use self::check::run_checks;
pub use self::diag::DiagCx;
pub use self::fmt::{fmt_syms_file, run_rustfmt};
pub use self::generate::gen_sorted_lints_file;
pub use self::parse::{ParseCx, new_parse_cx};
pub use self::utils::{ClippyInfo, FileUpdater, SourceFile, Span, UpdateMode};
