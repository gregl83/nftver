use std::cmp;
use ab_glyph::{Font, point, Glyph, Point, ScaleFont};
use image::{DynamicImage, RgbaImage};

/// Simple paragraph layout for glyphs into `target`.
///
/// This is for testing and examples.
pub fn layout_paragraph<F, SF>(
    font: SF,
    position: Point,
    max_width: f32,
    text: &str,
    target: &mut Vec<Glyph>,
) where
    F: Font,
    SF: ScaleFont<F>,
{
    let v_advance = font.height() + font.line_gap();
    let mut caret = position + point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;
    for c in text.chars() {
        if c.is_control() {
            if c == '\n' {
                caret = point(position.x, caret.y + v_advance);
                last_glyph = None;
            }
            continue;
        }
        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = caret;

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        if !c.is_whitespace() && caret.x > position.x + max_width {
            caret = point(position.x, caret.y + v_advance);
            glyph.position = caret;
            last_glyph = None;
        }

        target.push(glyph);
    }
}

// stack images into single canvas with background
pub fn stack_images(stack: Vec<RgbaImage>, background_color: [u8; 4]) -> RgbaImage {
    let mut canvas_width = 0;
    let mut canvas_height = 0;

    for image in &stack {
        if image.width() > canvas_width {
            canvas_width = image.width();
        }
        canvas_height += image.height();
    }

    let mut canvas = DynamicImage::new_rgba8(canvas_width, canvas_height).to_rgba8();
    for pixel in canvas.pixels_mut() {
        pixel.0 = background_color;
    }

    let mut y_offset = 0;
    for image in &stack {
        for (x, y, source_pixel) in image.enumerate_pixels() {
            let y = y_offset + y;
            let mut target_pixel = canvas.get_pixel_mut(x, y);
            target_pixel.0 = merge_rgba(source_pixel.0, target_pixel.0);
        }
        y_offset += image.height();
    }

    canvas
}

// merges rgba pixels source onto target (has transparency)
pub fn merge_rgba(source: [u8; 4], target: [u8; 4]) -> [u8; 4] {
    let mut merged: [u8; 4] = [0,0,0,0];

    let sr = source[0] as f32;
    let sg = source[1] as f32;
    let sb = source[2] as f32;
    let sa = source[3] as f32;

    let tr = target[0] as f32;
    let tg = target[1] as f32;
    let tb = target[2] as f32;
    let ta = target[3] as f32;

    let max = u8::MAX as f32;

    let a_out = sa + (ta * (max - sa) / max);
    let r_out = (sr * sa + tr * ta * (max - sa) / max) / a_out;
    let g_out = (sg * sa + tg * ta * (max - sa) / max) / a_out;
    let b_out = (sb * sa + tb * ta * (max - sa) / max) / a_out;

    merged[0] = cmp::min(r_out as u32, 255) as u8;
    merged[1] = cmp::min(g_out as u32, 255) as u8;
    merged[2] = cmp::min(b_out as u32, 255) as u8;
    merged[3] = cmp::min(a_out as u32, 255) as u8;

    merged
}