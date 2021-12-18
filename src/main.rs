use paq;
use qrcode::QrCode;
use qrcode::render::unicode;

fn main() {
    let hash = paq::hash_source(".");

    let text = format!("http://gregl83.com?nftverH={}", hash);

    // resource link or pure hash
    // nftverH - hash
    // nftverN - name
    // nftverD - description
    // nftverT - time

    let code = QrCode::new(text.as_str()).unwrap();
    let image = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("sha256: {}", hash);
    println!("{}", image);
}
