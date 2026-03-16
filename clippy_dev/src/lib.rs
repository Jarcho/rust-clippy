// `clap::Args` with a `Vec` doesn't compile with `feature(new_range)` so we
// have to separate this into a different crate than everything else.

use clap::{Args, Parser, Subcommand};

fn lint_name(name: &str) -> Result<String, String> {
    let name = name.replace('-', "_");
    if let Some((pre, _)) = name.split_once("::") {
        Err(format!("lint name should not contain the `{pre}` prefix"))
    } else if name
        .bytes()
        .any(|x| !matches!(x, b'_' | b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z'))
    {
        Err("lint name contains invalid characters".to_owned())
    } else {
        Ok(name)
    }
}

#[derive(Parser)]
#[command(name = "dev", about)]
pub struct Dev {
    #[command(subcommand)]
    pub command: DevCommand,
}

#[derive(Subcommand)]
pub enum DevCommand {
    /// Bless the test output changes
    Bless,
    /// Runs the dogfood test
    Dogfood {
        #[arg(long)]
        /// Apply the suggestions when possible
        fix: bool,
        #[arg(long, requires = "fix")]
        /// Fix code even if the working directory has changes
        allow_dirty: bool,
        #[arg(long, requires = "fix")]
        /// Fix code even if the working directory has staged changes
        allow_staged: bool,
        #[arg(long, requires = "fix")]
        /// Fix code even if a VCS was not detected
        allow_no_vcs: bool,
    },
    /// Run rustfmt on all projects and tests
    Fmt {
        #[arg(long)]
        /// Use the rustfmt --check option
        check: bool,
    },
    #[command(name = "update_lints")]
    /// Updates lint registration and information from the source code
    ///
    /// Makes sure that: {n}
    /// * the lint count in README.md is correct {n}
    /// * the changelog contains markdown link references at the bottom {n}
    /// * all lint groups include the correct lints {n}
    /// * lint modules in `clippy_lints/*` are visible in `src/lib.rs` via `pub mod` {n}
    /// * all lints are registered in the lint store
    UpdateLints {
        #[arg(long)]
        /// Checks that `cargo dev update_lints` has been run. Used on CI.
        check: bool,
    },
    #[command(name = "new_lint")]
    /// Create a new lint and run `cargo dev update_lints`
    NewLint {
        #[arg(short, long, default_value = "late")]
        /// Specify whether the lint runs during the early or late pass
        pass: String,
        #[arg(
            short,
            long,
            value_parser = lint_name,
        )]
        /// Name of the new lint in snake case, ex: `fn_too_long`
        name: String,
        #[arg(
            short,
            long,
            value_parser = [
                "style",
                "correctness",
                "suspicious",
                "complexity",
                "perf",
                "pedantic",
                "restriction",
                "cargo",
                "nursery",
            ],
            default_value = "nursery",
        )]
        /// What category the lint belongs to
        category: String,
        #[arg(long)]
        /// Add MSRV config code to the lint
        msrv: bool,
    },
    /// Support for setting up your personal development environment
    Setup(SetupCommand),
    /// Support for removing changes done by the setup command
    Remove(RemoveCommand),
    /// Launch a local 'ALL the Clippy Lints' website in a browser
    Serve {
        #[arg(short, long, default_value = "8000")]
        /// Local port for the http server
        port: u16,
        #[arg(long)]
        /// Which lint's page to load initially (optional)
        lint: Option<String>,
    },
    #[expect(clippy::doc_markdown)]
    /// Manually run clippy on a file or package
    ///
    /// ## Examples
    ///
    /// Lint a single file: {n}
    ///     cargo dev lint tests/ui/attrs.rs
    ///
    /// Lint a package directory: {n}
    ///     cargo dev lint tests/ui-cargo/wildcard_dependencies/fail {n}
    ///     cargo dev lint ~/my-project
    ///
    /// Run rustfix: {n}
    ///     cargo dev lint ~/my-project -- --fix
    ///
    /// Set lint levels: {n}
    ///     cargo dev lint file.rs -- -W clippy::pedantic {n}
    ///     cargo dev lint ~/my-project -- -- -W clippy::pedantic
    Lint {
        /// The Rust edition to use
        #[arg(long, default_value = "2024")]
        edition: String,
        /// The path to a file or package directory to lint
        path: String,
        /// Pass extra arguments to cargo/clippy-driver
        args: Vec<String>,
    },
    #[command(name = "rename_lint")]
    /// Rename a lint
    RenameLint {
        /// The name of the lint to rename
        #[arg(value_parser = lint_name)]
        old_name: String,
        #[arg(value_parser = lint_name)]
        /// The new name of the lint
        new_name: String,
    },
    /// Deprecate the given lint
    Deprecate {
        /// The name of the lint to deprecate
        #[arg(value_parser = lint_name)]
        name: String,
        #[arg(long, short)]
        /// The reason for deprecation
        reason: String,
    },
    /// Sync between the rust repo and the Clippy repo
    Sync(SyncCommand),
    /// Manage Clippy releases
    Release(ReleaseCommand),
    /// Marks a lint as uplifted into rustc and removes its code
    Uplift {
        /// The name of the lint to uplift
        #[arg(value_parser = lint_name)]
        old_name: String,
        /// The name of the lint in rustc
        #[arg(value_parser = lint_name)]
        new_name: Option<String>,
    },
}

#[derive(Args)]
pub struct SetupCommand {
    #[command(subcommand)]
    pub subcommand: SetupSubcommand,
}

#[derive(Subcommand)]
pub enum SetupSubcommand {
    /// Alter dependencies so Intellij Rust can find rustc internals
    Intellij {
        #[arg(long)]
        /// Remove the dependencies added with 'cargo dev setup intellij'
        remove: bool,
        #[arg(long, short, conflicts_with = "remove")]
        /// The path to a rustc repo that will be used for setting the dependencies
        repo_path: String,
    },
    /// Add a pre-commit git hook that formats your code to make it look pretty
    GitHook {
        #[arg(long)]
        /// Remove the pre-commit hook added with 'cargo dev setup git-hook'
        remove: bool,
        #[arg(long, short)]
        /// Forces the override of an existing git pre-commit hook
        force_override: bool,
    },
    /// Install a rustup toolchain pointing to the local clippy build
    ///
    /// This creates a toolchain with symlinks pointing at
    /// `target/.../{clippy-driver,cargo-clippy}`, rebuilds of the project will be reflected in the
    /// created toolchain unless `--standalone` is passed
    Toolchain {
        #[arg(long, short)]
        /// Create a standalone toolchain by copying the clippy binaries instead
        /// of symlinking them
        ///
        /// Use this for example to create a toolchain, make a small change and then make another
        /// toolchain with a different name in order to easily compare the two
        standalone: bool,
        #[arg(long, short)]
        /// Override an existing toolchain
        force: bool,
        #[arg(long, short)]
        /// Point to --release clippy binary
        release: bool,
        #[arg(long, short, default_value = "clippy")]
        /// Name of the toolchain
        name: String,
    },
    /// Add several tasks to vscode for formatting, validation and testing
    VscodeTasks {
        #[arg(long)]
        /// Remove the tasks added with 'cargo dev setup vscode-tasks'
        remove: bool,
        #[arg(long, short)]
        /// Forces the override of existing vscode tasks
        force_override: bool,
    },
}

#[derive(Args)]
pub struct RemoveCommand {
    #[command(subcommand)]
    pub subcommand: RemoveSubcommand,
}

#[derive(Subcommand)]
pub enum RemoveSubcommand {
    /// Remove the dependencies added with 'cargo dev setup intellij'
    Intellij,
    /// Remove the pre-commit git hook
    GitHook,
    /// Remove the tasks added with 'cargo dev setup vscode-tasks'
    VscodeTasks,
}

#[derive(Args)]
pub struct SyncCommand {
    #[command(subcommand)]
    pub subcommand: SyncSubcommand,
}

#[derive(Subcommand)]
pub enum SyncSubcommand {
    #[command(name = "update_nightly")]
    /// Update nightly version in `rust-toolchain.toml` and `clippy_utils`
    UpdateNightly,
}

#[derive(Args)]
pub struct ReleaseCommand {
    #[command(subcommand)]
    pub subcommand: ReleaseSubcommand,
}

#[derive(Subcommand)]
pub enum ReleaseSubcommand {
    #[command(name = "bump_version")]
    /// Bump the version in the Cargo.toml files
    BumpVersion,
}
