pub mod react;

use react::{Out, Pond, Render, View};

#[nvim_oxi::module]
fn nomad() {
    let mut pond = Pond::new();
    let (line, _line_in) = pond.pod(Line::default());
    let (offset, _offset_in) = pond.pod(Offset::default());
    let coordinates = PrintCoordinates { line, offset };
    pond.run(coordinates);
}

#[derive(Clone, Copy, Default)]
struct Line(usize);

impl core::fmt::Display for Line {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, Default)]
struct Offset(usize);

impl core::fmt::Display for Offset {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

struct PrintCoordinates {
    line: Out<Line>,
    offset: Out<Offset>,
}

impl View for PrintCoordinates {
    fn view(&self, pond: &Pond) -> impl Render {
        let line: Line = *self.line.get(pond);
        let offset: Offset = *self.offset.get(pond);
        CoordinatesSnapshot::new(line, offset)
    }
}

struct CoordinatesSnapshot {
    line: Line,
    offset: Offset,
}

impl CoordinatesSnapshot {
    fn new(line: Line, offset: Offset) -> Self {
        Self { line, offset }
    }
}

impl Render for CoordinatesSnapshot {
    fn render(&self) {
        nvim_oxi::print!("({}, {})", self.line, self.offset);
    }
}
