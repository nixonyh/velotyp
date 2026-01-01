#![allow(
    missing_debug_implementations,
    reason = "This is just an example."
)]

use chrono::{Datelike, Timelike};
use typst::comemo::Track;
use typst::diag::{FileResult, SourceResult};
use typst::engine::{Engine, Route, Sink, Traced};
use typst::foundations::{Bytes, Content, Datetime, StyleChain};
use typst::introspection::{Introspector, Locator};
use typst::layout::{Abs, Axes, Frame, Region, Size};
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, ROUTINES, World};
use typst_kit::fonts::{FontSlot, Fonts};

pub struct TypstWorld {
    /// Typst's standard library.
    library: LazyHash<Library>,
    /// Metadata about discovered fonts.
    book: LazyHash<FontBook>,
    /// Locations of and storage for lazily loaded fonts.
    fonts: Vec<FontSlot>,
}

pub fn layout_content(
    world: &dyn World,
    content: &Content,
) -> SourceResult<Frame> {
    let introspector = Introspector::default();
    let constraint = typst::comemo::Constraint::new();

    let mut sink = Sink::new();
    let traced = Traced::default();

    let mut engine = Engine {
        routines: &ROUTINES,
        world: world.track(),
        introspector: introspector.track_with(&constraint),
        traced: traced.track(),
        sink: sink.track_mut(),
        route: Route::default(),
    };

    (ROUTINES.layout_frame)(
        &mut engine,
        content,
        Locator::root(),
        StyleChain::default(),
        Region::new(Size::splat(Abs::inf()), Axes::splat(false)),
    )
}

impl TypstWorld {
    pub fn new() -> Self {
        let fonts = Fonts::searcher().search();

        Self {
            library: LazyHash::new(Library::builder().build()),
            book: LazyHash::new(fonts.book),
            fonts: fonts.fonts,
        }
    }
}

impl Default for TypstWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl World for TypstWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        unreachable!()
    }

    fn source(&self, _id: FileId) -> FileResult<Source> {
        unreachable!()
    }

    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        unreachable!()
    }

    #[doc = " Try to access the font with the given index in the font book."]
    fn font(&self, index: usize) -> Option<Font> {
        // comemo's validation may invoke this function with an invalid index. This is
        // impossible in typst-cli but possible if a custom tool mutates the fonts.
        self.fonts.get(index)?.get()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let now = chrono::Local::now();

        let naive = match offset {
            None => now.naive_local(),
            Some(o) => now.naive_utc() + chrono::Duration::hours(o),
        };

        Datetime::from_ymd_hms(
            naive.date().year(),
            naive.date().month().try_into().ok()?,
            naive.date().day().try_into().ok()?,
            naive.time().hour().try_into().ok()?,
            naive.time().minute().try_into().ok()?,
            naive.time().second().try_into().ok()?,
        )
    }
}
