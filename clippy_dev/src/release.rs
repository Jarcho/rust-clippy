use clippy_internal::DiagCx;
use clippy_internal::utils::{FileUpdater, UpdateStatus, Version, parse_cargo_package};
use std::fmt::Write;

static CARGO_TOML_FILES: &[&str] = &[
    "clippy_config/Cargo.toml",
    "clippy_lints/Cargo.toml",
    "clippy_utils/Cargo.toml",
    "declare_clippy_lint/Cargo.toml",
    "Cargo.toml",
];

pub fn bump_version(dcx: &DiagCx, mut version: Version) {
    version.minor += 1;

    let mut updater = FileUpdater::new_change(dcx);
    for file in CARGO_TOML_FILES {
        updater.update_file(file, &mut |_, src, dst| {
            let package = parse_cargo_package(src);
            if package.version_range.is_empty() {
                UpdateStatus::Unchanged
            } else {
                dst.push_str(&src[..package.version_range.start]);
                write!(dst, "\"{}\"", version.toml_display()).unwrap();
                dst.push_str(&src[package.version_range.end..]);
                UpdateStatus::from_changed(src.get(package.version_range) != dst.get(package.version_range))
            }
        });
    }
}
