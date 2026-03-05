use crate::generate::gen_sorted_lints_file;
use crate::utils::{
    ErrAction, FileUpdater, UpdateMode, UpdateStatus, VecBuf, expect_action, run_with_output, split_args_for_threads,
    walk_dir_no_dot_or_target,
};
use crate::{DiagCx, new_parse_cx};
use std::fmt::Write;
use std::process::{Command, Stdio};

/// Format the symbols list
fn fmt_syms(updater: &mut FileUpdater<'_>) {
    updater.update_file(
        "clippy_utils/src/sym.rs",
        &mut |_, text: &str, new_text: &mut String| {
            let (pre, conf) = text.split_once("generate! {\n").expect("can't find generate! call");
            let (conf, post) = conf.split_once("\n}\n").expect("can't find end of generate! call");
            let mut lines = conf
                .lines()
                .map(|line| {
                    let line = line.trim();
                    line.strip_suffix(',').unwrap_or(line).trim_end()
                })
                .collect::<Vec<_>>();
            lines.sort_unstable();
            write!(
                new_text,
                "{pre}generate! {{\n    {},\n}}\n{post}",
                lines.join(",\n    "),
            )
            .unwrap();
            if text == new_text {
                UpdateStatus::Unchanged
            } else {
                UpdateStatus::Changed
            }
        },
    );
}

fn run_rustfmt(dcx: &DiagCx, mode: UpdateMode) {
    let mut rustfmt_path = String::from_utf8(run_with_output(
        "rustup which rustfmt",
        Command::new("rustup").args(["which", "rustfmt"]),
    ))
    .expect("invalid rustfmt path");
    rustfmt_path.truncate(rustfmt_path.trim_end().len());

    let args: Vec<_> = walk_dir_no_dot_or_target(".")
        .filter_map(|e| {
            let e = expect_action(e, ErrAction::Read, ".");
            e.path()
                .as_os_str()
                .as_encoded_bytes()
                .ends_with(b".rs")
                .then(|| e.into_path().into_os_string())
        })
        .collect();

    let mut children: Vec<_> = split_args_for_threads(
        32,
        || {
            let mut cmd = Command::new(&rustfmt_path);
            if mode.is_check() {
                cmd.arg("--check");
            }
            cmd.stdout(Stdio::null())
                .stdin(Stdio::null())
                .stderr(Stdio::piped())
                .args(["--unstable-features", "--skip-children", "--color=always"]);
            cmd
        },
        args.iter(),
    )
    .map(|mut cmd| expect_action(cmd.spawn(), ErrAction::Run, "rustfmt"))
    .collect();

    let mut check_failed = false;
    for child in &mut children {
        let status = expect_action(child.wait(), ErrAction::Run, "rustfmt");
        match (mode, status.exit_ok()) {
            (UpdateMode::Check | UpdateMode::Change, Ok(())) => {},
            (UpdateMode::Check, Err(_)) => {
                if let Some(mut stderr) = child.stderr.take() {
                    dcx.emit_raw_err_from(&mut stderr);
                }
                check_failed = false;
            },
            (UpdateMode::Change, Err(_)) => {
                if let Some(mut stderr) = child.stderr.take() {
                    dcx.emit_raw_err_from(&mut stderr);
                }
            },
        }
    }
    if check_failed {
        dcx.emit_spanless_err_with_help("formatting check failed", "run `cargo dev fmt` to fix");
    }
}

// the "main" function of cargo dev fmt
pub fn run(dcx: &DiagCx, mode: UpdateMode) {
    new_parse_cx(dcx, |cx| {
        let mut lint_data = cx.parse_lint_decls();
        let mut conf_data = cx.parse_conf_mac();
        cx.dcx.exit_on_err();

        let mut updater = FileUpdater::new(cx.dcx, mode, "cargo dev fmt");
        fmt_syms(&mut updater);

        let copy: &mut dyn FnMut(&str, &mut String) = &mut |src, dst| dst.push_str(src);

        #[expect(clippy::mutable_key_type)]
        let mut lints = lint_data.lints.mk_by_file_map();
        let mut ranges = VecBuf::with_capacity(256);
        for passes in lint_data.lint_passes.iter_by_file_mut() {
            let file = passes[0].decl_sp.file;
            let mut lints = lints.remove(file);
            let lints = lints.as_deref_mut().unwrap_or_default();
            updater.update_loaded_file(file, &mut |_, src, dst| {
                gen_sorted_lints_file(src, dst, lints, passes, &mut ranges, copy);
                UpdateStatus::from_changed(src != dst)
            });
        }
        for (&file, lints) in &mut lints {
            updater.update_loaded_file(file, &mut |_, src, dst| {
                gen_sorted_lints_file(src, dst, lints, &mut [], &mut ranges, copy);
                UpdateStatus::from_changed(src != dst)
            });
        }

        updater.update_loaded_file(conf_data.decl_sp.file, &mut |_, src, dst| {
            conf_data.gen_file(src, dst);
            UpdateStatus::from_changed(src != dst)
        });

        run_rustfmt(cx.dcx, mode);
        cx.dcx.exit_on_err();
    });
}
