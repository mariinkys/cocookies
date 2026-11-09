//! The "default" recipe template: a port of the original HTML/CSS design to direct layout.
//! Sizes are the CSS pixel values × 0.75 (px → pt).

use super::{
    canvas::{Align, Canvas, Op, Stroke},
    fonts::{FaceId, Fonts},
    labels::Labels,
    layout::{self, Line, Rgb, Span, Style},
};
use crate::{entities::recipe::Recipe, time::format_day_month_year};

const A4: (f32, f32) = (595.28, 841.89);
const MARGIN: f32 = 42.0;

const PINK: Rgb = Rgb(0xd6, 0x33, 0x84);
const TEXT: Rgb = Rgb(0x4b, 0x2e, 0x2e);
const MUTED: Rgb = Rgb(0xa0, 0x52, 0x65);
const DESCRIPTION: Rgb = Rgb(0x5a, 0x2c, 0x2c);
const PALE: Rgb = Rgb(0xc9, 0x80, 0x98);
const BLUSH: Rgb = Rgb(0xff, 0xe4, 0xec);
const BLUSH_BORDER: Rgb = Rgb(0xf8, 0xbb, 0xd0);
const STEP_BG: Rgb = Rgb(0xfc, 0xe4, 0xec);
const ACCENT: Rgb = Rgb(0xe7, 0x54, 0x80);

const BODY: f32 = 10.5;
const LINE: f32 = 1.55;

fn style(face: FaceId, size: f32, color: Rgb) -> Style {
    Style { face, size, color }
}

/// Visual description of a content box (ingredient / step).
struct BoxStyle {
    fill: Rgb,
    border: Option<Stroke>,
    left_bar: Option<(Rgb, f32)>,
    pad_x: f32,
    pad_y: f32,
    radius: f32,
}

const INGREDIENT_BOX: BoxStyle = BoxStyle {
    fill: BLUSH,
    border: None,
    left_bar: Some((ACCENT, 3.75)),
    pad_x: 10.5,
    pad_y: 6.0,
    radius: 4.5,
};

const STEP_BOX: BoxStyle = BoxStyle {
    fill: STEP_BG,
    border: Some(Stroke {
        color: BLUSH_BORDER,
        width: 0.75,
        dashed: false,
    }),
    left_bar: None,
    pad_x: 10.5,
    pad_y: 7.5,
    radius: 4.5,
};

impl BoxStyle {
    fn inner_width(&self, outer: f32) -> f32 {
        outer - 2.0 * self.pad_x - self.left_bar.map_or(0.0, |(_, w)| w)
    }
}

pub fn layout(recipe: &Recipe, labels: &Labels, fonts: &Fonts, has_image: bool) -> Canvas {
    let mut c = Canvas::new(A4.0, A4.1, MARGIN);
    header(&mut c, recipe, labels, fonts, has_image);

    c.gap(18.0);
    dashed_rule(&mut c, 1.5);
    c.y += 4.0;

    // Ingredients
    let ingredient_lines: Vec<Vec<Line>> = recipe
        .ingredients
        .iter()
        .map(|i| {
            let text = ingredient_text(i.quantity, i.unit.as_deref(), &i.name, i.notes.as_deref());
            let w = INGREDIENT_BOX.inner_width(c.content_width());
            layout::wrap(
                fonts,
                &[Span::new(&text, style(FaceId::Regular, BODY, TEXT))],
                w,
                LINE,
            )
        })
        .collect();
    let first = ingredient_lines
        .first()
        .map_or(20.0, |l| box_height(l, &INGREDIENT_BOX));
    heading(&mut c, fonts, labels.ingredients, first);
    if ingredient_lines.is_empty() {
        empty_note(&mut c, fonts, labels.no_ingredients);
    }
    for lines in &ingredient_lines {
        boxed(&mut c, lines, &INGREDIENT_BOX);
        c.gap(5.25);
    }

    // Steps
    let step_lines: Vec<Vec<Line>> = recipe
        .steps
        .iter()
        .map(|s| {
            let number = format!("{} {}. ", labels.step, s.step_number);
            let duration = s.duration.map(|d| format!(" ({d} {})", labels.minutes));
            let mut spans = vec![
                Span::new(&number, style(FaceId::Bold, BODY, PINK)),
                Span::new(&s.instructions, style(FaceId::Regular, BODY, TEXT)),
            ];
            if let Some(d) = &duration {
                spans.push(Span::new(d, style(FaceId::Italic, 9.0, MUTED)));
            }
            layout::wrap(fonts, &spans, STEP_BOX.inner_width(c.content_width()), LINE)
        })
        .collect();
    let first = step_lines
        .first()
        .map_or(20.0, |l| box_height(l, &STEP_BOX));
    heading(&mut c, fonts, labels.instructions, first);
    if step_lines.is_empty() {
        empty_note(&mut c, fonts, labels.no_steps);
    }
    for lines in &step_lines {
        boxed(&mut c, lines, &STEP_BOX);
        c.gap(6.75);
    }

    footer(&mut c, recipe, labels, fonts);
    c
}

fn header(c: &mut Canvas, recipe: &Recipe, labels: &Labels, fonts: &Fonts, has_image: bool) {
    let width = c.content_width();
    let left = c.left();

    if has_image {
        let (w, h) = (165.0, 135.0);
        let x = left + (width - w) / 2.0;
        c.push(Op::Image {
            x,
            y: c.y,
            w,
            h,
            radius: 7.5,
        });
        c.push(Op::Rect {
            x,
            y: c.y,
            w,
            h,
            radius: 7.5,
            fill: None,
            stroke: Some(Stroke {
                color: BLUSH_BORDER,
                width: 1.5,
                dashed: false,
            }),
        });
        c.y += h + 12.0;
    }

    let title = layout::wrap(
        fonts,
        &[Span::new(&recipe.title, style(FaceId::Regular, 22.5, PINK))],
        width,
        1.3,
    );
    c.lines(&title, left, width, Align::Center);
    c.y += 4.0;

    let mut meta = Vec::new();
    if recipe.servings > 0 {
        meta.push(format!("{} {}", recipe.servings, labels.servings));
    }
    if let Some(p) = recipe.prep_time.filter(|p| *p > 0) {
        meta.push(format!("{p} {}", labels.prep));
    }
    if let Some(k) = recipe.cook_time.filter(|k| *k > 0) {
        meta.push(format!("{k} {}", labels.cook));
    }
    if !meta.is_empty() {
        let meta = meta.join("  ·  ");
        let lines = layout::wrap(
            fonts,
            &[Span::new(&meta, style(FaceId::Italic, 9.75, MUTED))],
            width,
            LINE,
        );
        c.lines(&lines, left, width, Align::Center);
        c.y += 6.0;
    }

    let pills: Vec<&str> = [recipe.category.as_ref(), recipe.difficulty.as_ref()]
        .into_iter()
        .flatten()
        .map(|t| t.name.as_str())
        .collect();
    if !pills.is_empty() {
        pills_row(c, fonts, &pills);
        c.y += 8.0;
    }

    if let Some(description) = recipe
        .description
        .as_deref()
        .filter(|d| !d.trim().is_empty())
    {
        let lines = layout::wrap(
            fonts,
            &[Span::new(
                description,
                style(FaceId::Italic, 9.75, DESCRIPTION),
            )],
            width,
            LINE,
        );
        c.lines(&lines, left, width, Align::Center);
    }
}

fn pills_row(c: &mut Canvas, fonts: &Fonts, labels: &[&str]) {
    let st = style(FaceId::Regular, 9.0, MUTED);
    let (pad_x, pad_y, gap) = (9.0, 2.25, 4.5);
    let pills: Vec<(Line, f32)> = labels
        .iter()
        .filter_map(|l| {
            layout::wrap(fonts, &[Span::new(l, st)], c.content_width() / 2.0, 1.4)
                .into_iter()
                .next()
        })
        .map(|line| {
            let w = line.width + 2.0 * pad_x;
            (line, w)
        })
        .collect();
    let height = pills.iter().map(|(l, _)| l.height).fold(0.0, f32::max) + 2.0 * pad_y;
    let total: f32 =
        pills.iter().map(|(_, w)| w).sum::<f32>() + gap * (pills.len().saturating_sub(1)) as f32;

    c.ensure(height);
    let mut x = c.left() + (c.content_width() - total) / 2.0;
    let top = c.y;
    for (line, w) in &pills {
        c.push(Op::Rect {
            x,
            y: top,
            w: *w,
            h: height,
            radius: height / 2.0,
            fill: Some(BLUSH),
            stroke: Some(Stroke {
                color: BLUSH_BORDER,
                width: 0.75,
                dashed: false,
            }),
        });
        c.line_at(line, x + pad_x, line.width, Align::Left, top + pad_y);
        x += w + gap;
    }
    c.y += height;
}

/// Section title with a dashed underline, kept on the same page as the first `next` points of
/// content that follow it.
fn heading(c: &mut Canvas, fonts: &Fonts, text: &str, next: f32) {
    let lines = layout::wrap(
        fonts,
        &[Span::new(text, style(FaceId::Regular, 16.5, PINK))],
        c.content_width(),
        1.3,
    );
    let height = layout::height(&lines) + 4.5 + 1.5 + 10.5;
    c.gap(22.0);
    c.ensure(height + next.min(c.page_content_height() / 3.0));
    let (left, width) = (c.left(), c.content_width());
    c.lines(&lines, left, width, Align::Left);
    c.y += 4.5;
    dashed_rule(c, 1.5);
    c.y += 10.5;
}

fn dashed_rule(c: &mut Canvas, width: f32) {
    let y = c.y + width / 2.0;
    c.push(Op::Line {
        x1: c.left(),
        y1: y,
        x2: c.left() + c.content_width(),
        y2: y,
        stroke: Stroke {
            color: BLUSH_BORDER,
            width,
            dashed: true,
        },
    });
    c.y += width;
}

fn empty_note(c: &mut Canvas, fonts: &Fonts, text: &str) {
    let lines = layout::wrap(
        fonts,
        &[Span::new(text, style(FaceId::Italic, 9.75, PALE))],
        c.content_width(),
        LINE,
    );
    let (left, width) = (c.left(), c.content_width());
    c.lines(&lines, left, width, Align::Left);
}

fn box_height(lines: &[Line], st: &BoxStyle) -> f32 {
    layout::height(lines) + 2.0 * st.pad_y
}

/// Draws `lines` inside a box. Boxes are never split unless they are taller than a page, in
/// which case they continue on the next page(s) as separate boxes.
fn boxed(c: &mut Canvas, lines: &[Line], st: &BoxStyle) {
    let mut rest = lines;
    let total = box_height(rest, st);
    if total <= c.page_content_height() {
        c.ensure(total);
    } else {
        // It will be split anyway: start right here if at least one line fits.
        c.ensure(2.0 * st.pad_y + rest.first().map_or(0.0, |l| l.height));
    }
    while !rest.is_empty() {
        // How many lines fit on this page (at least one, to always make progress).
        let mut used = 2.0 * st.pad_y;
        let mut n = 0;
        for line in rest {
            if n > 0 && used + line.height > c.remaining() {
                break;
            }
            used += line.height;
            n += 1;
        }
        let (chunk, tail) = rest.split_at(n);
        draw_box(c, chunk, st, used);
        rest = tail;
        if !rest.is_empty() {
            c.new_page();
        }
    }
}

fn draw_box(c: &mut Canvas, lines: &[Line], st: &BoxStyle, height: f32) {
    let (x, y, w) = (c.left(), c.y, c.content_width());
    let mut text_x = x + st.pad_x;
    if let Some((color, bar)) = st.left_bar {
        c.push(Op::Rect {
            x,
            y,
            w,
            h: height,
            radius: st.radius,
            fill: Some(color),
            stroke: None,
        });
        c.push(Op::Rect {
            x: x + bar,
            y,
            w: w - bar,
            h: height,
            radius: st.radius,
            fill: Some(st.fill),
            stroke: None,
        });
        text_x += bar;
    } else {
        c.push(Op::Rect {
            x,
            y,
            w,
            h: height,
            radius: st.radius,
            fill: Some(st.fill),
            stroke: st.border,
        });
    }

    let inner = st.inner_width(w);
    let mut top = y + st.pad_y;
    for line in lines {
        c.line_at(line, text_x, inner, Align::Left, top);
        top += line.height;
    }
    c.y = y + height;
}

fn footer(c: &mut Canvas, recipe: &Recipe, labels: &Labels, fonts: &Fonts) {
    let st = style(FaceId::Italic, 8.25, PALE);
    let width = c.content_width();
    let third = width / 3.0;
    let created = format_day_month_year(&recipe.created_at)
        .map(|d| format!("{} {d}", labels.created))
        .unwrap_or_default();
    let updated = format_day_month_year(&recipe.updated_at)
        .map(|d| format!("{} {d}", labels.updated))
        .unwrap_or_default();

    let one_line = |text: &str| {
        layout::wrap(fonts, &[Span::new(text, st)], third, LINE)
            .into_iter()
            .next()
    };
    let cells = [
        (one_line(&created), Align::Left),
        (one_line(labels.made_with), Align::Center),
        (one_line(&updated), Align::Right),
    ];
    let height = cells
        .iter()
        .filter_map(|(l, _)| l.as_ref().map(|l| l.height))
        .fold(0.0, f32::max);

    c.gap(21.0);
    c.ensure(height + 6.75);
    dashed_rule(c, 0.75);
    c.y += 6.0;
    let top = c.y;
    for (i, (line, align)) in cells.iter().enumerate() {
        if let Some(line) = line {
            let x = c.left() + third * i as f32;
            c.line_at(line, x, third, *align, top);
        }
    }
    c.y += height;
}

/// `2 cups — flour (sifted)`, `½`-free: quantities print as `2`, `0.5`, `1.25`.
fn ingredient_text(
    quantity: Option<f64>,
    unit: Option<&str>,
    name: &str,
    notes: Option<&str>,
) -> String {
    let mut s = String::new();
    if let Some(q) = quantity {
        s.push_str(&format_quantity(q));
        if let Some(u) = unit.filter(|u| !u.is_empty()) {
            s.push(' ');
            s.push_str(u);
        }
        s.push_str(" — ");
    }
    s.push_str(name);
    if let Some(n) = notes.filter(|n| !n.is_empty()) {
        s.push_str(" (");
        s.push_str(n);
        s.push(')');
    }
    s
}

fn format_quantity(q: f64) -> String {
    if q.fract().abs() < 1e-9 {
        format!("{}", q as i64)
    } else {
        let s = format!("{q:.2}");
        s.trim_end_matches('0').trim_end_matches('.').to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_ingredients() {
        assert_eq!(
            ingredient_text(Some(2.0), Some("cups"), "flour", Some("sifted")),
            "2 cups — flour (sifted)"
        );
        assert_eq!(
            ingredient_text(Some(0.5), None, "lemon", None),
            "0.5 — lemon"
        );
        assert_eq!(ingredient_text(None, Some("g"), "salt", Some("")), "salt");
        assert_eq!(format_quantity(1.25), "1.25");
        assert_eq!(format_quantity(1.333), "1.33");
    }
}
