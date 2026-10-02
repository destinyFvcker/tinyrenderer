use image::codecs::tga::TgaEncoder;
use image::ImageEncoder;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let img = image::open("framebuffer.tga").unwrap().to_rgb8();

    let f = BufWriter::new(File::create("/tmp/fb_norle.tga").unwrap());
    let enc = TgaEncoder::new(f).disable_rle();
    img.write_with_encoder(enc).unwrap();
}
