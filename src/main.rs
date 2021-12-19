use std::cmp;
use paq;
use qrcode::QrCode;
use image::{Luma, DynamicImage, Rgba, Pixel};
use ab_glyph::{Font, FontRef, point, Glyph, Point, ScaleFont, PxScale};

mod lib;

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

fn main() {
    let hash = paq::hash_source(".");
    let name = "nftver distinct name";

    let text = format!("http://gregl83.com?nftverH={}&nftverN={}", hash, name);

    // resource link or pure hash
    // nftverH - hash
    // nftverN - name
    // nftverD - description
    // nftverT - time

    let code = QrCode::new(text.as_str()).unwrap();
    let image_qrcode = code.render::<Luma<u8>>().build();

    image_qrcode.save("./qrcode.png").unwrap();


    println!("sha256: {}", hash);

    let max_width = image_qrcode.width() as f32;



    // ---------------
    let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-B.ttf")).unwrap();

    // The font size to use
    let scale = PxScale::from(16.0);

    let scaled_font = font.as_scaled(scale);

    let mut glyphs = Vec::new();
    layout_paragraph(scaled_font, point(20.0, 20.0), max_width - 40.0, name, &mut glyphs);

    // Use a dark red colour
    let colour = (0, 0, 0);

    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs.first().unwrap().position.x;
        let last_glyph = glyphs.last().unwrap();
        let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as u32
    };

    // Create a new rgba image with some padding
    let mut image_name = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();

    let mut image_header = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();
    for p in image_header.pixels_mut() {
        p.0 = [255, 255, 255, 255];
    }

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // Draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                let px = image_name.get_pixel_mut(x + bounds.min.x as u32, y + bounds.min.y as u32);
                // Turn the coverage into an alpha value (blended with any previous)
                *px = Rgba([
                    colour.0,
                    colour.1,
                    colour.2,
                    px.0[3].saturating_add((v * 255.0) as u8),
                ]);
            });
        }
    }

    for (x, y, source_pixel) in image_name.enumerate_pixels() {
        let mut target_pixel = image_header.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }

    // Save the image to a png file
    image_header.save("header.png").unwrap();
    
    // -------------------
    
    
    let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-B.ttf")).unwrap();

    // The font size to use
    let scale = PxScale::from(10.0);

    let scaled_font = font.as_scaled(scale);

    let mut glyphs = Vec::new();
    layout_paragraph(scaled_font, point(20.0, 20.0), max_width - 40.0, hash.as_str(), &mut glyphs);

    // Use a dark red colour
    let colour = (0, 0, 0);

    // work out the layout size
    let glyphs_height = scaled_font.height().ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs.first().unwrap().position.x;
        let last_glyph = glyphs.last().unwrap();
        let max_x = last_glyph.position.x + scaled_font.h_advance(last_glyph.id);
        (max_x - min_x).ceil() as u32
    };

    // Create a new rgba image with some padding
    let mut image_details = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();

    let mut image_footer = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba8();
    for p in image_footer.pixels_mut() {
        p.0 = [255, 255, 255, 255];
    }

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(outlined) = scaled_font.outline_glyph(glyph) {
            let bounds = outlined.px_bounds();
            // Draw the glyph into the image per-pixel by using the draw closure
            outlined.draw(|x, y, v| {
                // Offset the position by the glyph bounding box
                let px = image_details.get_pixel_mut(x + bounds.min.x as u32, y + bounds.min.y as u32);
                // Turn the coverage into an alpha value (blended with any previous)
                *px = Rgba([
                    colour.0,
                    colour.1,
                    colour.2,
                    px.0[3].saturating_add((v * 255.0) as u8),
                ]);
            });
        }
    }

    for (x, y, source_pixel) in image_details.enumerate_pixels() {
        let mut target_pixel = image_footer.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }

    // Save the image to a png file
    image_footer.save("footer.png").unwrap();
    
    // -----------------



    let nft_width = cmp::max(image_header.width(), image_qrcode.width());
    let nft_height = image_header.height() + image_qrcode.height() + image_footer.height();
    let mut nft_draw_offset: u32 = 0;

    let mut image_nft = DynamicImage::new_rgba8(nft_width, nft_height).to_rgba8();
    for p in image_nft.pixels_mut() {
        p.0 = [255, 255, 255, 255];
    }

    // write header
    for (x, y, source_pixel) in image_header.enumerate_pixels() {
        //println!("WRITING {} {}", x, y);
        let mut target_pixel = image_nft.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }
    nft_draw_offset += image_header.height();

    // write qr code
    for (x, y, source_pixel_luma) in image_qrcode.enumerate_pixels() {
        let y_offset = y + nft_draw_offset;
        let source_pixel = source_pixel_luma.to_rgba();
        let mut target_pixel = image_nft.get_pixel_mut(x, y_offset);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }
    nft_draw_offset += image_qrcode.height();

    // write footer
    for (x, y, source_pixel) in image_footer.enumerate_pixels() {
        let y_offset = y + nft_draw_offset;
        let source_pixel = source_pixel.to_rgba();
        let mut target_pixel = image_nft.get_pixel_mut(x, y_offset);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }
    nft_draw_offset += image_footer.height();

    image_nft.save("nft.png").unwrap();
}
