use crate::Span;
use annotate_snippets::renderer::{DEFAULT_TERM_WIDTH, Renderer};
use annotate_snippets::{Annotation, AnnotationKind, Group, Level, Origin, Snippet};
use core::cell::{Cell, RefCell};
use core::panic::Location;
use std::borrow::Cow;
use std::io::{Read, Write as _};
use std::process;

pub struct DiagCx {
    out: RefCell<anstream::Stderr>,
    renderer: Renderer,
    has_err: Cell<bool>,
    has_phase_err: Cell<bool>,
}
impl Default for DiagCx {
    fn default() -> Self {
        let width = termize::dimensions().map_or(DEFAULT_TERM_WIDTH, |(w, _)| w);
        Self {
            out: RefCell::new(anstream::stderr()),
            renderer: Renderer::styled().term_width(width),
            has_err: Cell::new(false),
            has_phase_err: Cell::new(false),
        }
    }
}
impl Drop for DiagCx {
    fn drop(&mut self) {
        if self.has_err.get() {
            self.emit_err(&[
                Group::with_title(
                    Level::ERROR
                        .with_name("internal error")
                        .primary_title("errors were found, but it was assumed none occurred"),
                ),
                Group::with_title(Level::NOTE.secondary_title("any produced results may be incorrect")),
            ]);
            process::exit(1);
        }
    }
}
impl DiagCx {
    pub fn exit_on_err(&self) {
        if self.has_err.get() {
            process::exit(1);
        }
    }

    #[track_caller]
    pub fn exit_assume_err(&self) -> ! {
        if !self.has_err.get() {
            self.emit_err(&[
                Group::with_title(
                    Level::ERROR
                        .with_name("internal error")
                        .primary_title("errors were expected, but is was assumed one would occur"),
                ),
                mk_loc_group(Location::caller()),
            ]);
        }
        process::exit(1);
    }

    pub fn take_phase_err(&self) -> bool {
        self.has_phase_err.replace(false)
    }
}

fn sp_to_snip(kind: AnnotationKind, sp: Span<'_>) -> Snippet<'_, Annotation<'_>> {
    let line_starts = sp.file.line_starts();
    let first_line = match line_starts.binary_search(&sp.range.start) {
        Ok(x) => x,
        // Note: `Err(0)` isn't possible since `0` is always the first start.
        Err(x) => x - 1,
    };
    let start = line_starts[first_line] as usize;
    let last_line = match line_starts.binary_search(&sp.range.end) {
        Ok(x) => x,
        Err(x) => x - 1,
    };
    let end = line_starts
        .get(last_line + 1)
        .map_or(sp.file.contents.len(), |&x| x as usize);
    Snippet::source(&sp.file.contents[start..end])
        .line_start(first_line + 1)
        .path(sp.file.path.get())
        .annotation(kind.span((sp.range.start as usize - start..sp.range.end as usize - start).into()))
}

fn mk_spanned_primary<'a>(level: Level<'a>, sp: Span<'a>, msg: impl Into<Cow<'a, str>>) -> Group<'a> {
    level
        .primary_title(msg.into())
        .element(sp_to_snip(AnnotationKind::Primary, sp))
}

fn mk_spanned_secondary<'a>(level: Level<'a>, sp: Span<'a>, msg: impl Into<Cow<'a, str>>) -> Group<'a> {
    level
        .secondary_title(msg.into())
        .element(sp_to_snip(AnnotationKind::Context, sp))
}

fn mk_loc_group<'a>(loc: &Location<'a>) -> Group<'a> {
    Level::INFO.secondary_title("error created here").element(
        Origin::path(loc.file())
            .line(loc.line() as usize)
            .char_column(loc.column() as usize),
    )
}

impl DiagCx {
    pub fn emit_raw_err_from(&self, data: &mut impl Read) {
        let mut buf = [0u8; 1024];
        let mut out = self.out.borrow_mut();
        while let size = data.read(&mut buf).unwrap()
            && size != 0
        {
            out.write_all(&buf[..size]).unwrap();
        }
        self.has_err.set(true);
    }

    fn emit_err(&self, groups: &[Group<'_>]) {
        let mut s = self.renderer.render(groups);
        s.push('\n');
        self.out.borrow_mut().write_all(s.as_bytes()).unwrap();
        self.has_err.set(true);
    }

    #[track_caller]
    pub fn emit_spanned_err<'a>(&self, sp: Span<'a>, msg: impl Into<Cow<'a, str>>) {
        self.emit_err(&[
            mk_spanned_primary(Level::ERROR, sp, msg.into()),
            mk_loc_group(Location::caller()),
        ]);
    }

    pub fn emit_spanned_err_loc<'a>(&self, sp: Span<'a>, msg: impl Into<Cow<'a, str>>, loc: &Location<'_>) {
        self.emit_err(&[mk_spanned_primary(Level::ERROR, sp, msg.into()), mk_loc_group(loc)]);
    }

    #[track_caller]
    pub fn emit_spanless_err<'a>(&self, msg: impl Into<Cow<'a, str>>) {
        self.emit_err(&[
            Group::with_title(Level::ERROR.primary_title(msg.into())),
            mk_loc_group(Location::caller()),
        ]);
    }

    #[track_caller]
    pub fn emit_spanless_err_with_help<'a>(&self, msg: impl Into<Cow<'a, str>>, help_msg: impl Into<Cow<'a, str>>) {
        self.emit_err(&[
            Group::with_title(Level::ERROR.primary_title(msg)),
            Group::with_title(Level::HELP.secondary_title(help_msg)),
            mk_loc_group(Location::caller()),
        ]);
    }

    #[track_caller]
    pub fn emit_path_err<'a>(&self, path: impl Into<Cow<'a, str>>, msg: impl Into<Cow<'a, str>>) {
        self.emit_err(&[
            Level::ERROR.primary_title(msg).element(Origin::path(path)),
            mk_loc_group(Location::caller()),
        ]);
    }

    #[track_caller]
    pub fn emit_path_err_with_help<'a>(
        &self,
        path: impl Into<Cow<'a, str>>,
        msg: impl Into<Cow<'a, str>>,
        help_msg: impl Into<Cow<'a, str>>,
    ) {
        self.emit_err(&[
            Level::ERROR.primary_title(msg).element(Origin::path(path)),
            Group::with_title(Level::HELP.secondary_title(help_msg)),
            mk_loc_group(Location::caller()),
        ]);
    }

    #[track_caller]
    pub fn emit_already_deprecated(&self, name: &str) {
        self.emit_spanless_err(format!("lint `{name}` is already deprecated"));
    }

    #[track_caller]
    pub fn emit_duplicate_lint(&self, sp: Span<'_>, first_sp: Span<'_>) {
        self.emit_err(&[
            mk_spanned_primary(Level::ERROR, sp, "duplicate lint name declared"),
            mk_spanned_secondary(Level::NOTE, first_sp, "previous declaration here"),
            mk_loc_group(Location::caller()),
        ]);
    }

    #[track_caller]
    pub fn emit_not_clippy_lint_name(&self, sp: Span<'_>) {
        self.emit_err(&[
            mk_spanned_primary(Level::ERROR, sp, "not a clippy lint name"),
            Group::with_title(Level::HELP.secondary_title("add the `clippy::` tool prefix")),
            mk_loc_group(Location::caller()),
        ]);
    }

    #[track_caller]
    pub fn emit_unknown_lint(&self, name: &str) {
        self.emit_spanless_err(format!("unknown lint `{name}`"));
    }

    #[track_caller]
    pub fn emit_update_err(&self, path: Cow<'_, str>, fix_tool: &str) {
        self.emit_path_err_with_help(path, "file is out of date", format!("run `{fix_tool}` to fix"));
    }
}
