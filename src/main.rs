use clap::{App, Arg};
use paq;
use image::RgbaImage;
use url::Url;
use serde_derive::Deserialize;
use std::fs;

mod lib;
mod nft;

#[derive(Debug, Deserialize)]
struct Config {
    uri: String,
    public_key: String,
    name: String
}

fn generate(config: Config) {
    // resource link or pure hash
    // nftverH - hash
    // nftverN - name
    // nftverT - tag
    // nftverD - description

    let mut base_uri = Url::parse("http://gregl83.com/").unwrap();
    let public_key = "0x9e2f0699293fdfCB9D774Dd27F4A68E1C7007946";
    let hash = paq::hash_source(".");
    let name = "nftver distinct name";
    let description = "one two three four five six seven eight nine. \n\t- ten \n\t- eleven \n\t- twelve \n\t- thirteen \n\t- fourteen\nfifteen sixteen seventeen eighteen.";
    let tag = "v0.2.3";
    let background_color = [255, 255, 255, 255];

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

    let mut image_stack: Vec<RgbaImage> = vec![];

    let body = nft::generate_body(base_uri.to_string().as_str());
    let nft_width = body.width();

    image_stack.push(nft::generate_header(name, tag, nft_width));
    image_stack.push(body);
    image_stack.push(nft::generate_footer(hash.as_str(), public_key, description, nft_width));

    let nft = lib::stack_images(image_stack, background_color);

    // save image
    nft.save("./nft.png").unwrap();
}

fn main() {
    // todo - add error handling with messaging

    /*
        let mut base_uri = Url::parse("http://gregl83.com/").unwrap();
        let public_key = "0x9e2f0699293fdfCB9D774Dd27F4A68E1C7007946";
        let hash = paq::hash_source(".");
        let name = "nftver distinct name";

        let description = "one two three four five six seven eight nine. \n\t- ten \n\t- eleven \n\t- twelve \n\t- thirteen \n\t- fourteen\nfifteen sixteen seventeen eighteen.";
        let tag = "v0.2.3";

        let background_color = [255, 255, 255, 255];
     */

    let matches = App::new("nftver")
        .version("0.1.0")
        .about("Generate software version NFT.")
        .get_matches();


    let content = fs::read_to_string("example/nftver.toml").unwrap();

    let config: Config = toml::from_str(content.as_str()).unwrap();

    generate(config);
}
