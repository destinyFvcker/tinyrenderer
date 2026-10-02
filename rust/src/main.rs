use image::codecs::tga::TgaEncoder;
use image::{ImageEncoder, Rgb, RgbImage};
use std::fs::File;
use std::io::BufWriter;

// 注意：image crate 用 RGB 顺序，C++ 版的 TGAColor 是 BGRA 顺序
const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const RED: Rgb<u8> = Rgb([255, 0, 0]);
const BLUE: Rgb<u8> = Rgb([64, 128, 255]);
const YELLOW: Rgb<u8> = Rgb([255, 200, 0]);

fn main() -> image::ImageResult<()> {
    const WIDTH: u32 = 64;
    const HEIGHT: u32 = 64;
    let mut framebuffer = RgbImage::new(WIDTH, HEIGHT);

    let (ax, ay) = (7, 3);
    let (bx, by) = (12, 37);
    let (cx, cy) = (62, 53);

    framebuffer.put_pixel(ax, ay, WHITE);
    framebuffer.put_pixel(bx, by, WHITE);
    framebuffer.put_pixel(cx, cy, WHITE);

    // image crate 原点是左上角，tinyrenderer 习惯左下角原点，写出时翻转一下
    let flipped = image::imageops::flip_vertical(&framebuffer);

    // macOS 预览/ImageIO 不支持 RLE 压缩的 TGA，这里禁用压缩
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/framebuffer.tga");
    let file = BufWriter::new(File::create(path)?);
    flipped.write_with_encoder(TgaEncoder::new(file).disable_rle())
}
