use crate::DiagCx;
use crate::generate::gen_sorted_lints_file;
use crate::ir::{ConfDef, ParsedLints};
use crate::utils::{
    ErrAction, FileUpdater, UpdateMode, UpdateStatus, VecBuf, expect_action, run_with_output, split_args_for_threads,
    walk_dir_no_dot_or_target,
};
use std::fmt::Write;
use std::process::{Command, Stdio};

/// Format the symbols list
pub fn fmt_syms_file(updater: &mut FileUpdater<'_>) {
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
            UpdateStatus::from_changed(text != new_text)
        },
    );
}

/// Runs rustfmt on all files.
pub fn run_rustfmt(dcx: &DiagCx, mode: UpdateMode) {
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
                check_failed = true;
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

impl ParsedLints<'_> {
    /// Formats and sorts lint and lint pass declarations.
    pub fn fmt_decl_files(&mut self, updater: &mut FileUpdater<'_>) {
        let copy: &mut dyn FnMut(&str, &mut String) = &mut |src, dst| dst.push_str(src);

        #[expect(clippy::mutable_key_type)]
        let mut lints = self.lints.mk_by_file_map();
        let mut ranges = VecBuf::with_capacity(256);
        for passes in self.lint_passes.iter_by_file_mut() {
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
    }
}

impl ConfDef<'_> {
    /// Formats and sorts the config declaration.
    pub fn fmt_def_file(&mut self, updater: &mut FileUpdater<'_>) {
        updater.update_loaded_file(self.decl_sp.file, &mut |_, src, dst| {
            dst.push_str(&src[..self.decl_sp.range.start as usize]);
            self.gen_mac(src, dst);

            dst.push_str(&src[self.decl_sp.range.end as usize..]);
            UpdateStatus::from_changed(src != dst)
        });
    }
}
