use std::cmp;
use std::time::SystemTime;
use qrcode::QrCode;
use image::{Luma, DynamicImage, RgbaImage, Pixel, Rgb};
use ab_glyph::{Font, FontRef, PxScale};
use chrono::prelude::{DateTime, Utc};

use crate::lib;

fn draw_title(title: &str, width: f32) -> RgbaImage {
    let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-B.ttf")).unwrap();
    let color = Rgb::<u8>::from([0, 0, 0]);
    let scale = PxScale::from(30.0); // font size
    let scaled_font = font.as_scaled(scale);

    lib::draw_glyphs(title, scaled_font, color, width, lib::ImageMargin(20.0, 20.0, 0.0, 20.0))
}

fn draw_tag(tag: &str, width: f32) -> RgbaImage {
    let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-C.ttf")).unwrap();
    let color = Rgb::<u8>::from([0, 0, 0]);
    let scale = PxScale::from(30.0); // font size
    let scaled_font = font.as_scaled(scale);

    lib::draw_glyphs(tag, scaled_font, color, width, lib::ImageMargin(20.0, 20.0, 0.0, 20.0))
}

fn draw_description(description: &str, width: f32) -> RgbaImage {
    let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-C.ttf")).unwrap();
    let color = Rgb::<u8>::from([0, 0, 0]);
    let scale = PxScale::from(24.0); // font size
    let scaled_font = font.as_scaled(scale);

    lib::draw_glyphs(description, scaled_font, color, width, lib::ImageMargin(20.0, 40.0, 20.0, 40.0))
}

fn draw_timestamp(timestamp: SystemTime, width: f32) -> RgbaImage {
    let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-C.ttf")).unwrap();
    let color = Rgb::<u8>::from([0, 0, 0]);
    let scale = PxScale::from(12.0); // font size
    let scaled_font = font.as_scaled(scale);

    let dt: DateTime<Utc> = timestamp.clone().into();
    let text = format!("{}", dt.format("%+"));

    lib::draw_glyphs(text.as_str(), scaled_font, color, width, lib::ImageMargin(0.0, 20.0, 10.0, 20.0))
}

fn draw_public_key(public_key: &str, width: f32) -> RgbaImage {
    let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-C.ttf")).unwrap();
    let color = Rgb::<u8>::from([0, 0, 0]);
    let scale = PxScale::from(12.0); // font size
    let scaled_font = font.as_scaled(scale);

    lib::draw_glyphs(public_key, scaled_font, color, width, lib::ImageMargin(0.0, 20.0, 0.0, 20.0))
}

fn draw_hash(hash: &str, width: f32) -> RgbaImage {
    let font = FontRef::try_from_slice(include_bytes!("/usr/share/fonts/truetype/ubuntu/Ubuntu-B.ttf")).unwrap();
    let color = Rgb::<u8>::from([0, 0, 0]);
    let scale = PxScale::from(10.0); // font size
    let scaled_font = font.as_scaled(scale);

    lib::draw_glyphs(hash, scaled_font, color, width, lib::ImageMargin(2.0, 20.0, 10.0, 20.0))
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

pub fn generate_body(qrcode_text: &str) -> RgbaImage {
    let code = QrCode::new(qrcode_text).unwrap();
    let image_code = code.render::<Luma<u8>>().quiet_zone(false).build();

    let width = image_code.width() + 80;
    let height = image_code.height() + 40;

    let mut body = DynamicImage::new_rgba8(width, height).to_rgba8();

    for (x, y, source_pixel_luma) in image_code.enumerate_pixels() {
        let source_pixel = source_pixel_luma.to_rgba();
        let x = 40 + x;
        let y = 20 + y;
        let mut target_pixel = body.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }

    body
}

pub fn generate_footer(hash: &str, public_key: &str, description: &str, width: u32) -> RgbaImage {
    let width_float = width as f32;
    let image_public_key = draw_public_key(public_key, width_float);
    let image_description = draw_description(description, width_float);
    let image_timestamp = draw_timestamp(SystemTime::now(), width_float);
    let image_hash = draw_hash(hash, width_float);

    let height = image_public_key.height() + image_description.height() + image_timestamp.height() + image_hash.height();
    let mut footer = DynamicImage::new_rgba8(width, height).to_rgba8();
    let mut y_offset = 0;

    let x_offset = (width - image_hash.width()) / 2;
    for (x, y, source_pixel) in image_hash.enumerate_pixels() {
        let x = x_offset + x;
        let mut target_pixel = footer.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }
    y_offset += image_hash.height();

    for (x, y, source_pixel) in image_description.enumerate_pixels() {
        let y = y_offset + y;
        let mut target_pixel = footer.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }
    y_offset += image_description.height();

    for (x, y, source_pixel) in image_public_key.enumerate_pixels() {
        let y = y_offset + y;
        let mut target_pixel = footer.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }

    let x_offset = width - image_timestamp.width();
    for (x, y, source_pixel) in image_timestamp.enumerate_pixels() {
        let x = x_offset + x;
        let y = y_offset + y;
        let mut target_pixel = footer.get_pixel_mut(x, y);
        target_pixel.0 = lib::merge_rgba(source_pixel.0, target_pixel.0);
    }

    return footer
}