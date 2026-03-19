#![feature(rustc_private)]

#[test]
fn internal_checks() {
    if option_env!("CLIPPY_SKIP_INTERNAL_TEST").is_some() {
        return;
    }

    let dcx = clippy_internal::DiagCx::default();
    clippy_internal::run_checks(&dcx);

    // When building rustc we only have the bootstrap rustfmt. Since this updates
    // at a different cadence than the one used in the clippy repository we can't
    // block CI based on how that version formats code without causing major sync
    // issues when the two versions format code differently.
    if option_env!("RUSTC_TEST_SUITE").is_none() {
        clippy_internal::run_rustfmt(&dcx, clippy_internal::UpdateMode::Check);
    }
    dcx.exit_on_err();
}
