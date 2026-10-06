use core::panic;
use std::{fs::File, io::BufWriter};

use image::{Rgb, RgbImage, codecs::tga::TgaEncoder};

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const RED: Rgb<u8> = Rgb([255, 0, 0]);

fn main() -> anyhow::Result<()> {
    let (models, _) = tobj::load_obj(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../obj/african_head/african_head.obj"
        ),
        &tobj::GPU_LOAD_OPTIONS,
    )?;

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;
    let mut framebuffer = RgbImage::new(WIDTH, HEIGHT);

    let calculate_pixel = |x: f32, y: f32| -> (u32, u32) {
        (
            ((x + 1.) / 2. * (WIDTH - 1) as f32) as u32,
            ((y + 1.) / 2. * (HEIGHT - 1) as f32) as u32,
        )
    };

    for m in models.iter() {
        let mesh = &m.mesh;
        let mut face_indices = mesh.indices.chunks_exact(3);

        while let Some(&[i1, i2, i3]) = face_indices.next() {
            let (i1, i2, i3) = (i1 as usize, i2 as usize, i3 as usize);
            let [x1, y1, _] = mesh.positions[i1 * 3..i1 * 3 + 3] else {
                panic!("expected 3 elements")
            };
            let [x2, y2, _] = mesh.positions[i2 * 3..i2 * 3 + 3] else {
                panic!("expected 3 elements")
            };
            let [x3, y3, _] = mesh.positions[i3 * 3..i3 * 3 + 3] else {
                panic!("expected 3 elements")
            };

            let (x1, y1) = calculate_pixel(x1, y1);
            let (x2, y2) = calculate_pixel(x2, y2);
            let (x3, y3) = calculate_pixel(x3, y3);

            tinyrenderer::line_naive(x1, y1, x2, y2, &mut framebuffer, RED);
            tinyrenderer::line_naive(x2, y2, x3, y3, &mut framebuffer, RED);
            tinyrenderer::line_naive(x3, y3, x1, y1, &mut framebuffer, RED);

            framebuffer.put_pixel(x1, y1, WHITE);
            framebuffer.put_pixel(x2, y2, WHITE);
            framebuffer.put_pixel(x3, y3, WHITE);
        }
    }

    // image crate 原点是左上角，tinyrenderer 习惯左下角原点，写出时翻转一下
    let flipped = image::imageops::flip_vertical(&framebuffer);

    // macOS 预览/ImageIO 不支持 RLE 压缩的 TGA，这里禁用压缩
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/target/framebuffer_african_head_wireframe.tga"
    );
    let file = BufWriter::new(File::create(path)?);
    flipped.write_with_encoder(TgaEncoder::new(file).disable_rle())?;

    Ok(())
}
