use std::cmp;
use paq;
use qrcode::QrCode;
use image::{Luma, DynamicImage, Rgba, Pixel};
use ab_glyph::{Font, FontRef, point, Glyph, Point, ScaleFont, PxScale};
use url::Url;

mod lib;
mod nft;

// fn temp() {
//
//
//
//     let header = card::generate_body(text.as_str());
//
//
//     let max_width = image_qrcode.width() as f32;
//
//     // -------------------
//
//     // -----------------
//
//
//
//     let nft_width = cmp::max(image_header.width(), image_qrcode.width());
//     let nft_height = image_header.height() + image_qrcode.height() + image_footer.height();
//     let mut nft_draw_offset: u32 = 0;
//
//     let mut image_nft = DynamicImage::new_rgba8(nft_width, nft_height).to_rgba8();
//     for p in image_nft.pixels_mut() {
//         p.0 = [255, 255, 255, 255];
//     }
//
//     // write header
//     for (x, y, source_pixel) in image_header.enumerate_pixels() {
//         //println!("WRITING {} {}", x, y);
//         let mut target_pixel = image_nft.get_pixel_mut(x, y);
//         target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
//     }
//     nft_draw_offset += image_header.height();
//
//     // write qr code
//     for (x, y, source_pixel_luma) in image_qrcode.enumerate_pixels() {
//         let y_offset = y + nft_draw_offset;
//         let source_pixel = source_pixel_luma.to_rgba();
//         let mut target_pixel = image_nft.get_pixel_mut(x, y_offset);
//         target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
//     }
//     nft_draw_offset += image_qrcode.height();
//
//     // write footer
//     for (x, y, source_pixel) in image_footer.enumerate_pixels() {
//         let y_offset = y + nft_draw_offset;
//         let source_pixel = source_pixel.to_rgba();
//         let mut target_pixel = image_nft.get_pixel_mut(x, y_offset);
//         target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
//     }
//     nft_draw_offset += image_footer.height();
//
//     image_nft.save("nft.png").unwrap();
// }

fn main() {
    // resource link or pure hash
    // nftverH - hash
    // nftverN - name
    // nftverT - tag
    // nftverD - description

    let mut base_uri = Url::parse("http://gregl83.com/").unwrap();
    let hash = paq::hash_source(".");
    let name = "nftver distinct name";
    let tag = "v0.2.3";

    // generate qrcode text/uri
    let mut query_pairs = match base_uri.query_pairs() {
        Some(q) => q,
        None => Vec::<(String, String)>::new()
    };
    query_pairs.push((String::from("nftverH"), hash.clone()));
    query_pairs.push((String::from("nftverN"), String::from(name)));
    base_uri.set_query_from_pairs(query_pairs);

    let body = nft::generate_body(base_uri.to_string().as_str());

    let nft_width = body.width();

    let header = nft::generate_header(name, tag, nft_width);
    let footer = nft::generate_footer(hash.as_str(), nft_width);

    let nft_height = header.height() + body.height() + footer.height();
    let mut nft = DynamicImage::new_rgba8(nft_width, nft_height).to_rgba8();
    for p in nft.pixels_mut() {
        p.0 = [255, 255, 255, 255];
    }

    let mut nft_draw_y_offset = 0;

    // draw nft
    for (x, y, source_pixel) in header.enumerate_pixels() {
        let y = nft_draw_y_offset + y;
        let mut target_pixel = nft.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }
    nft_draw_y_offset += header.height();
    for (x, y, source_pixel_luma) in body.enumerate_pixels() {
        let source_pixel = source_pixel_luma.to_rgba();
        let y = nft_draw_y_offset + y;
        let mut target_pixel = nft.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }
    nft_draw_y_offset += body.height();
    for (x, y, source_pixel) in footer.enumerate_pixels() {
        let y = nft_draw_y_offset + y;
        let mut target_pixel = nft.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }

    // save image
    nft.save("./nft.png").unwrap();
}
