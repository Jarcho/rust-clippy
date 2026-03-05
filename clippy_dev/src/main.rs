#![feature(new_range, new_range_api, os_str_slice, os_string_truncate, rustc_private)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![allow(clippy::case_sensitive_file_extension_comparisons, clippy::missing_panics_doc)]

mod dogfood;
mod edit_lints;
mod lint;
mod new_lint;
mod release;
mod serve;
mod setup;
mod sync;

use clap::Parser;
use clippy_dev::{
    Dev, DevCommand, ReleaseCommand, ReleaseSubcommand, RemoveCommand, RemoveSubcommand, SetupCommand, SetupSubcommand,
    SyncCommand, SyncSubcommand,
};
use clippy_internal::{ClippyInfo, FileUpdater, UpdateMode, fmt, new_parse_cx};
use std::env;

fn main() {
    let dev = Dev::parse();
    let clippy = ClippyInfo::search_for_manifest();
    if let Err(e) = env::set_current_dir(&clippy.path) {
        panic!("error setting current directory to `{}`: {e}", clippy.path.display());
    }

    match dev.command {
        DevCommand::Bless => {
            eprintln!("use `cargo bless` to automatically replace `.stderr` and `.fixed` files as tests are being run");
        },
        DevCommand::Dogfood {
            fix,
            allow_dirty,
            allow_staged,
            allow_no_vcs,
        } => dogfood::dogfood(fix, allow_dirty, allow_staged, allow_no_vcs),
        DevCommand::Fmt { check } => fmt::run(UpdateMode::from_check(check)),
        DevCommand::UpdateLints { check } => new_parse_cx(|cx| {
            let data = cx.parse_lint_decls();
            cx.dcx.exit_on_err();
            data.gen_decls(&mut FileUpdater::from_check(check));
        }),
        DevCommand::NewLint {
            pass,
            name,
            category,
            msrv,
        } => new_lint::create(clippy.version, &pass, &name, &category, msrv),
        DevCommand::Setup(SetupCommand { subcommand }) => match subcommand {
            SetupSubcommand::Intellij { remove, repo_path } => {
                if remove {
                    setup::intellij::remove_rustc_src();
                } else {
                    setup::intellij::setup_rustc_src(&repo_path);
                }
            },
            SetupSubcommand::GitHook { remove, force_override } => {
                if remove {
                    setup::git_hook::remove_hook();
                } else {
                    setup::git_hook::install_hook(force_override);
                }
            },
            SetupSubcommand::Toolchain {
                standalone,
                force,
                release,
                name,
            } => setup::toolchain::create(standalone, force, release, &name),
            SetupSubcommand::VscodeTasks { remove, force_override } => {
                if remove {
                    setup::vscode::remove_tasks();
                } else {
                    setup::vscode::install_tasks(force_override);
                }
            },
        },
        DevCommand::Remove(RemoveCommand { subcommand }) => match subcommand {
            RemoveSubcommand::Intellij => setup::intellij::remove_rustc_src(),
            RemoveSubcommand::GitHook => setup::git_hook::remove_hook(),
            RemoveSubcommand::VscodeTasks => setup::vscode::remove_tasks(),
        },
        DevCommand::Serve { port, lint } => serve::run(port, lint),
        DevCommand::Lint { path, edition, args } => lint::run(&path, &edition, args.iter()),
        DevCommand::RenameLint { old_name, new_name } => new_parse_cx(|cx| {
            edit_lints::rename(cx, clippy.version, &old_name, &new_name);
        }),
        DevCommand::Uplift { old_name, new_name } => new_parse_cx(|cx| {
            edit_lints::uplift(cx, clippy.version, &old_name, new_name.as_deref().unwrap_or(&old_name));
        }),
        DevCommand::Deprecate { name, reason } => {
            new_parse_cx(|cx| edit_lints::deprecate(cx, clippy.version, &name, &reason));
        },
        DevCommand::Sync(SyncCommand { subcommand }) => match subcommand {
            SyncSubcommand::UpdateNightly => sync::update_nightly(),
        },
        DevCommand::Release(ReleaseCommand { subcommand }) => match subcommand {
            ReleaseSubcommand::BumpVersion => release::bump_version(clippy.version),
        },
    }
}
