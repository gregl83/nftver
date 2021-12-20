use paq;
use image::{DynamicImage, Pixel};
use url::Url;

mod lib;
mod nft;

fn main() {
    // resource link or pure hash
    // nftverH - hash
    // nftverN - name
    // nftverT - tag
    // nftverD - description

    let mut base_uri = Url::parse("http://gregl83.com/").unwrap();
    let hash = paq::hash_source(".");
    let name = "nftver distinct name";
    let description = "walk talk walk talk walk talk walk talk walk.";
    let tag = "v0.2.3";

    // generate qrcode text/uri
    let mut query_pairs = match base_uri.query_pairs() {
        Some(q) => q,
        None => Vec::<(String, String)>::new()
    };
    query_pairs.push((String::from("nftverH"), hash.clone()));
    query_pairs.push((String::from("nftverN"), String::from(name)));
    query_pairs.push((String::from("nftverD"), String::from(description)));
    query_pairs.push((String::from("nftverT"), String::from(tag)));
    base_uri.set_query_from_pairs(query_pairs);

    let body = nft::generate_body(base_uri.to_string().as_str());

    let nft_width = body.width();

    let header = nft::generate_header(name, tag, nft_width);
    let footer = nft::generate_footer(hash.as_str(), description, nft_width);

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
