use std::cmp;
use ab_glyph::{Font, point, Glyph, Point, ScaleFont};

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