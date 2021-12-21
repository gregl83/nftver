use std::cmp;
use ab_glyph::{Font, point, Glyph, Point, ScaleFont, PxScaleFont, FontRef};
use image::{DynamicImage, RgbaImage, Rgba, Rgb};

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

/// ImageMargin (top, right, bottom, left)
pub struct ImageMargin(
    pub f32,
    pub f32,
    pub f32,
    pub f32
);

pub fn draw_glyphs(
    text: &str,
    scaled_font:
    PxScaleFont<&FontRef>,
    color: Rgb<u8>,
    width: f32,
    margin: ImageMargin
) -> RgbaImage {
    let top_margin = margin.0;
    let right_margin = margin.1;
    let bottom_margin = margin.2;
    let left_margin = margin.3;

    let horizontal_margin = right_margin + left_margin;
    let vertical_margin = top_margin + bottom_margin;

    // map text to glyphs
    let mut glyphs = Vec::new();
    layout_paragraph(
        scaled_font,
        point(left_margin, top_margin),
        width - horizontal_margin,
        text,
        &mut glyphs
    );

    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs.first().unwrap().position.x;
        let last_glyph = glyphs.last().unwrap();
        let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as u32
    };

    // create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(
        glyphs_width + (horizontal_margin as u32),
        glyphs_height + (vertical_margin as u32)
    ).to_rgba8();

    // loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // offset the position by the glyph bounding box
                let px = image.get_pixel_mut(x + bounds.min.x as u32, y + bounds.min.y as u32);
                // turn the coverage into an alpha value (blended with any previous)
                *px = Rgba([
                    color.0[0],
                    color.0[1],
                    color.0[2],
                    px.0[3].saturating_add((v * 255.0) as u8),
                ]);
            });
        }
    }

    image
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