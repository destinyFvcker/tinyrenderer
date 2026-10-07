use std::{fs::File, io::BufWriter};

use image::{Rgb, RgbImage, codecs::tga::TgaEncoder};
use tinyrenderer::scan_rasterization;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const RED: Rgb<u8> = Rgb([255, 0, 0]);

fn main() -> anyhow::Result<()> {
    let mut framebuffer = RgbImage::new(WIDTH, HEIGHT);

    let positions: [u32; 18] = [
        7, 45, 35, 100, 45, 60, 120, 35, 90, 5, 45, 110, 115, 83, 80, 90, 85, 120,
    ];
    let indices: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let color_indices: [usize; 3] = [0, 1, 2];
    let colors: [Rgb<u8>; 3] = [WHITE, GREEN, RED];

    let mut face_indices = indices.chunks_exact(3).zip(color_indices);
    while let Some((&[i1, i2, i3], color_index)) = face_indices.next() {
        let [x1, y1] = positions[i1 * 2..i1 * 2 + 2] else {
            panic!("expected 2 elements")
        };
        let [x2, y2] = positions[i2 * 2..i2 * 2 + 2] else {
            panic!("expected 2 elements")
        };
        let [x3, y3] = positions[i3 * 2..i3 * 2 + 2] else {
            panic!("expected 2 elements")
        };
        scan_rasterization(
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            &mut framebuffer,
            colors[color_index],
        );
    }

    // image crate 原点是左上角，tinyrenderer 习惯左下角原点，写出时翻转一下
    let flipped = image::imageops::flip_vertical(&framebuffer);

    // macOS 预览/ImageIO 不支持 RLE 压缩的 TGA，这里禁用压缩
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/target/framebuffer_tra_naive.tga"
    );
    let file = BufWriter::new(File::create(path)?);
    flipped.write_with_encoder(TgaEncoder::new(file).disable_rle())?;
    Ok(())
}
