//! A tiny page model: layout code appends drawing operations to pages (handling page breaks),
//! and [`super::render`] later replays them with krilla. Keeping the two apart makes layout pure
//! and easy to test, and keeps krilla's borrow-heavy page/surface API in one place.

use super::layout::{Line, Rgb, Style};

#[derive(Debug, Clone, Copy)]
pub struct Stroke {
    pub color: Rgb,
    pub width: f32,
    pub dashed: bool,
}

#[derive(Debug, Clone)]
pub enum Op {
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
        fill: Option<Rgb>,
        stroke: Option<Stroke>,
    },
    Line {
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        stroke: Stroke,
    },
    Text {
        x: f32,
        baseline: f32,
        text: String,
        style: Style,
    },
    /// The recipe picture, scaled to cover the box and clipped to its rounded corners.
    Image {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radius: f32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
}

pub struct Canvas {
    pub pages: Vec<Vec<Op>>,
    pub width: f32,
    pub height: f32,
    pub margin: f32,
    /// Current vertical position on the last page.
    pub y: f32,
}

impl Canvas {
    pub fn new(width: f32, height: f32, margin: f32) -> Self {
        Self {
            pages: vec![Vec::new()],
            width,
            height,
            margin,
            y: margin,
        }
    }

    pub fn left(&self) -> f32 {
        self.margin
    }

    pub fn content_width(&self) -> f32 {
        self.width - 2.0 * self.margin
    }

    pub fn page_content_height(&self) -> f32 {
        self.height - 2.0 * self.margin
    }

    pub fn remaining(&self) -> f32 {
        self.height - self.margin - self.y
    }

    fn at_page_top(&self) -> bool {
        self.y <= self.margin
    }

    pub fn new_page(&mut self) {
        self.pages.push(Vec::new());
        self.y = self.margin;
    }

    /// Starts a new page unless `height` still fits (or we are already at the top of a page).
    pub fn ensure(&mut self, height: f32) {
        if height > self.remaining() && !self.at_page_top() {
            self.new_page();
        }
    }

    /// Vertical space that collapses at the top of a page (like CSS margins after a break).
    pub fn gap(&mut self, height: f32) {
        if !self.at_page_top() {
            self.y = (self.y + height).min(self.height - self.margin);
        }
    }

    pub fn push(&mut self, op: Op) {
        self.pages
            .last_mut()
            .expect("there is always a page")
            .push(op);
    }

    /// Draws `lines` inside the horizontal band `[x, x + width]` starting at the current
    /// position, breaking pages between lines as needed.
    pub fn lines(&mut self, lines: &[Line], x: f32, width: f32, align: Align) {
        for line in lines {
            self.ensure(line.height);
            self.line_at(line, x, width, align, self.y);
            self.y += line.height;
        }
    }

    /// Draws one line with its top at `top`, without moving the cursor.
    pub fn line_at(&mut self, line: &Line, x: f32, width: f32, align: Align, top: f32) {
        let offset = match align {
            Align::Left => 0.0,
            Align::Center => (width - line.width) / 2.0,
            Align::Right => width - line.width,
        };
        for run in &line.runs {
            self.push(Op::Text {
                x: x + offset + run.x,
                baseline: top + line.baseline,
                text: run.text.clone(),
                style: run.style,
            });
        }
    }
}
