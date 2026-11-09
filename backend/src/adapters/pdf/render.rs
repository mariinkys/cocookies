//! Replays a [`Canvas`] into a PDF with krilla.

use krilla::{
    Document,
    color::rgb,
    geom::{Path, PathBuilder, Point, Size, Transform},
    image::Image,
    metadata::Metadata,
    num::NormalizedF32,
    page::PageSettings,
    paint::{Fill, FillRule, Stroke as KStroke, StrokeDash},
    surface::Surface,
};

use super::{
    canvas::{Canvas, Op, Stroke},
    fonts::Fonts,
    layout::Rgb,
};

pub struct Meta<'a> {
    pub title: &'a str,
    pub language: &'a str,
}

pub fn render(
    canvas: &Canvas,
    fonts: &Fonts,
    image: Option<&Image>,
    meta: Meta<'_>,
) -> Result<Vec<u8>, String> {
    let mut document = Document::new();
    document.set_metadata(
        Metadata::new()
            .title(meta.title.to_owned())
            .language(meta.language.to_owned())
            .creator("Cocookies".to_owned())
            .producer(concat!("Cocookies ", env!("CARGO_PKG_VERSION"), " (krilla)").to_owned()),
    );

    let settings = PageSettings::from_wh(canvas.width, canvas.height).ok_or("invalid page size")?;
    for ops in &canvas.pages {
        let mut page = document.start_page_with(settings.clone());
        let mut surface = page.surface();
        for op in ops {
            draw(&mut surface, op, fonts, image);
        }
        surface.finish();
        page.finish();
    }

    document.finish().map_err(|e| format!("{e:?}"))
}

fn draw(surface: &mut Surface<'_>, op: &Op, fonts: &Fonts, image: Option<&Image>) {
    match op {
        Op::Rect {
            x,
            y,
            w,
            h,
            radius,
            fill,
            stroke,
        } => {
            let Some(path) = rounded_rect(*x, *y, *w, *h, *radius) else {
                return;
            };
            surface.set_fill(fill.map(solid_fill));
            surface.set_stroke(stroke.map(to_stroke));
            surface.draw_path(&path);
        }
        Op::Line {
            x1,
            y1,
            x2,
            y2,
            stroke,
        } => {
            let mut pb = PathBuilder::new();
            pb.move_to(*x1, *y1);
            pb.line_to(*x2, *y2);
            let Some(path) = pb.finish() else { return };
            surface.set_fill(None);
            surface.set_stroke(Some(to_stroke(*stroke)));
            surface.draw_path(&path);
        }
        Op::Text {
            x,
            baseline,
            text,
            style,
        } => {
            let face = fonts.face(style.face);
            let glyphs = face.shape(text);
            surface.set_stroke(None);
            surface.set_fill(Some(solid_fill(style.color)));
            surface.draw_glyphs(
                Point::from_xy(*x, *baseline),
                &glyphs,
                face.font.clone(),
                text,
                style.size,
                false,
            );
        }
        Op::Image { x, y, w, h, radius } => {
            let Some(image) = image else { return };
            let (iw, ih) = image.size();
            if iw == 0 || ih == 0 {
                return;
            }
            // "object-fit: cover": scale to fill the box, centre, clip the overflow.
            let scale = (w / iw as f32).max(h / ih as f32);
            let (dw, dh) = (iw as f32 * scale, ih as f32 * scale);
            let Some(clip) = rounded_rect(*x, *y, *w, *h, *radius) else {
                return;
            };
            let Some(size) = Size::from_wh(dw, dh) else {
                return;
            };

            surface.push_clip_path(&clip, &FillRule::NonZero);
            surface.push_transform(&Transform::from_translate(
                x + (w - dw) / 2.0,
                y + (h - dh) / 2.0,
            ));
            surface.draw_image(image.clone(), size);
            surface.pop();
            surface.pop();
        }
    }
}

fn solid_fill(c: Rgb) -> Fill {
    Fill {
        paint: rgb::Color::new(c.0, c.1, c.2).into(),
        opacity: NormalizedF32::ONE,
        rule: FillRule::NonZero,
    }
}

fn to_stroke(s: Stroke) -> KStroke {
    KStroke {
        paint: rgb::Color::new(s.color.0, s.color.1, s.color.2).into(),
        width: s.width,
        dash: s.dashed.then(|| StrokeDash {
            array: vec![s.width * 3.0, s.width * 2.0],
            offset: 0.0,
        }),
        ..KStroke::default()
    }
}

/// Rectangle with circular corners (cubic Bézier approximation).
fn rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32) -> Option<Path> {
    const K: f32 = 0.552_284_8;
    let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
    let c = r * K;
    let (l, t, rt, b) = (x, y, x + w, y + h);
    let mut pb = PathBuilder::new();
    pb.move_to(l + r, t);
    pb.line_to(rt - r, t);
    pb.cubic_to(rt - r + c, t, rt, t + r - c, rt, t + r);
    pb.line_to(rt, b - r);
    pb.cubic_to(rt, b - r + c, rt - r + c, b, rt - r, b);
    pb.line_to(l + r, b);
    pb.cubic_to(l + r - c, b, l, b - r + c, l, b - r);
    pb.line_to(l, t + r);
    pb.cubic_to(l, t + r - c, l + r - c, t, l + r, t);
    pb.close();
    pb.finish()
}
