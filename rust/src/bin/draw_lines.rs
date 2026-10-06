use image::codecs::tga::TgaEncoder;
use image::{Rgb, RgbImage};
use rand::RngExt;
use std::fs::File;
use std::io::BufWriter;
use tinyrenderer::line_naive;

fn main() -> image::ImageResult<()> {
    const WIDTH: u32 = 64;
    const HEIGHT: u32 = 64;
    let mut framebuffer = RgbImage::new(WIDTH, HEIGHT);

    let mut rng = rand::rng();
    for _ in 0_u32..(1 << 24) {
        let ax = rng.random_range(0..WIDTH);
        let ay = rng.random_range(0..HEIGHT);
        let bx = rng.random_range(0..WIDTH);
        let by = rng.random_range(0..HEIGHT);
        line_naive(
            ax,
            ay,
            bx,
            by,
            &mut framebuffer,
            Rgb([rng.random(), rng.random(), rng.random()]),
        );
    }

    // image crate 原点是左上角，tinyrenderer 习惯左下角原点，写出时翻转一下
    let flipped = image::imageops::flip_vertical(&framebuffer);

    // macOS 预览/ImageIO 不支持 RLE 压缩的 TGA，这里禁用压缩
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/target/framebuffer_lines.tga");
    let file = BufWriter::new(File::create(path)?);
    flipped.write_with_encoder(TgaEncoder::new(file).disable_rle())
}
