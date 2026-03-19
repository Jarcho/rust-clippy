#![feature(rustc_private)]

#[test]
fn check() {
    std::env::set_current_dir("..").unwrap();
    let dcx = clippy_internal::DiagCx::default();
    clippy_internal::run_checks(&dcx);
    clippy_internal::run_rustfmt(&dcx, clippy_internal::UpdateMode::Check);
    dcx.exit_on_err();
}
