use std::cmp;
use qrcode::QrCode;
use image::{Luma, DynamicImage, Rgba, ImageBuffer, RgbaImage};
use ab_glyph::{Font, FontRef, point, ScaleFont, PxScale};

use crate::lib;

fn draw_title(title: &str, width: f32) -> RgbaImage {
    let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-B.ttf")).unwrap();
    let color = (0, 0, 0);
    let scale = PxScale::from(22.0); // font size
    let scaled_font = font.as_scaled(scale);

    // map title to glyphs
    let mut glyphs = Vec::new();
    lib::layout_paragraph(scaled_font, point(20.0, 20.0), width - 40.0, title, &mut glyphs);

    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
    let min_x = glyphs.first().unwrap().position.x;
    let last_glyph = glyphs.last().unwrap();
    let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as u32
    };

    // create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 20).to_rgba8();

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
                    color.0,
                    color.1,
                    color.2,
                    px.0[3].saturating_add((v * 255.0) as u8),
                ]);
            });
        }
    }

    image
}

fn draw_tag(tag: &str, width: f32) -> RgbaImage {
    let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-C.ttf")).unwrap();
    let color = (0, 0, 0);
    let scale = PxScale::from(22.0); // font size
    let scaled_font = font.as_scaled(scale);

    // map title to glyphs
    let mut glyphs = Vec::new();
    lib::layout_paragraph(scaled_font, point(20.0, 20.0), width - 40.0, tag, &mut glyphs);

    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
    let min_x = glyphs.first().unwrap().position.x;
    let last_glyph = glyphs.last().unwrap();
    let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as u32
    };

    // create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 20).to_rgba8();

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
                    color.0,
                    color.1,
                    color.2,
                    px.0[3].saturating_add((v * 255.0) as u8),
                ]);
            });
        }
    }

    image
}

fn draw_description(description: &str, width: f32) -> RgbaImage {
    let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-C.ttf")).unwrap();
    let color = (0, 0, 0);
    let scale = PxScale::from(18.0); // font size
    let scaled_font = font.as_scaled(scale);

    // map title to glyphs
    let mut glyphs = Vec::new();
    lib::layout_paragraph(scaled_font, point(20.0, 0.0), width - 40.0, description, &mut glyphs);

    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs.first().unwrap().position.x;
        let last_glyph = glyphs.last().unwrap();
        let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as u32
    };

    // create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height).to_rgba8();

    // loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // Draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                let px = image.get_pixel_mut(x + bounds.min.x as u32, y + bounds.min.y as u32);
                // Turn the coverage into an alpha value (blended with any previous)
                *px = Rgba([
                    color.0,
                    color.1,
                    color.2,
                    px.0[3].saturating_add((v * 255.0) as u8),
                ]);
            });
        }
    }

    image
}

fn draw_hash(hash: &str, width: f32) -> RgbaImage {
    let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-B.ttf")).unwrap();
    let color = (0, 0, 0);
    let scale = PxScale::from(12.0); // font size
    let scaled_font = font.as_scaled(scale);

    // map title to glyphs
    let mut glyphs = Vec::new();
    lib::layout_paragraph(scaled_font, point(20.0, 20.0), width - 40.0, hash, &mut glyphs);

    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs.first().unwrap().position.x;
        let last_glyph = glyphs.last().unwrap();
        let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as u32
    };

    // create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();

    // loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // Draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                let px = image.get_pixel_mut(x + bounds.min.x as u32, y + bounds.min.y as u32);
                // Turn the coverage into an alpha value (blended with any previous)
                *px = Rgba([
                    color.0,
                    color.1,
                    color.2,
                    px.0[3].saturating_add((v * 255.0) as u8),
                ]);
            });
        }
    }

    image
}

pub fn generate_header(title: &str, tag: &str, width: u32) -> RgbaImage {
    let width_float = width as f32;

    let title_width: f32 = width_float * 0.7;
    let tag_width: f32 = width_float - title_width;

    let image_title = draw_title(title, title_width);
    let image_tag = draw_tag(tag, tag_width);

    let height = cmp::max(image_title.height(), image_tag.height());
    let mut header = DynamicImage::new_rgba8(width, height).to_rgba8();

    for (x, y, source_pixel) in image_title.enumerate_pixels() {
        let mut target_pixel = header.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }

    let x_offset = width - image_tag.width();
    for (x, y, source_pixel) in image_tag.enumerate_pixels() {
        let x = x_offset + x;
        let mut target_pixel = header.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }

    return header
}

pub fn generate_body(qrcode_text: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let code = QrCode::new(qrcode_text).unwrap();
    code.render::<Luma<u8>>().build()
}

pub fn generate_footer(hash: &str, description: &str, width: u32) -> RgbaImage {
    let width_float = width as f32;
    let image_description = draw_description(description, width_float);
    let image_hash = draw_hash(hash, width_float);

    let height = image_description.height() + image_hash.height();
    let mut footer = DynamicImage::new_rgba8(width, height).to_rgba8();

    for (x, y, source_pixel) in image_description.enumerate_pixels() {
        let mut target_pixel = footer.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }

    let y_offset = image_description.height();
    for (x, y, source_pixel) in image_hash.enumerate_pixels() {
        let y = y_offset + y;
        let mut target_pixel = footer.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }

    return footer
}