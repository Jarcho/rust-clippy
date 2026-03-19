use crate::{DiagCx, FileUpdater, UpdateMode, fmt_syms_file, new_parse_cx};

/// Runs all internal checks across the clippy repository which do not require
/// building clippy itself.
///
/// To determine if a check failed use the diagnostic context after calling this.
pub fn run_checks(dcx: &DiagCx) {
    new_parse_cx(dcx, |cx| {
        let cx = &mut **cx;

        let mut lint_data = cx.parse_lint_decls();
        let has_lint_err = cx.dcx.take_phase_err();
        let mut conf_data = cx.parse_conf_mac();
        let has_conf_err = cx.dcx.take_phase_err();

        let mut updater = FileUpdater::new(cx.dcx, UpdateMode::Check, "cargo dev update_lints");
        if !has_lint_err {
            lint_data.gen_decls(&mut updater);
        }

        updater.fix_tool = "cargo dev fmt";
        if !has_lint_err {
            lint_data.fmt_decl_files(&mut updater);
        }
        if !has_conf_err {
            conf_data.fmt_def_file(&mut updater);
        }
        fmt_syms_file(&mut updater);
    });
}
