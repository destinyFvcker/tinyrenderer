fn main() {
    let path = std::env::args().nth(1).unwrap_or("framebuffer.tga".into());
    let img = image::open(&path).unwrap().to_rgb8();
    println!("{path}: {}x{}", img.width(), img.height());
    for (x, y, p) in img.enumerate_pixels() {
        if p.0 != [0, 0, 0] {
            println!("non-black pixel at ({x}, {y}): {:?}", p.0);
        }
    }
}
