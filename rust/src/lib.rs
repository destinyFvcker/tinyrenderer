use image::{ImageBuffer, Rgb};

pub fn line_naive(
    ax: u32,
    ay: u32,
    bx: u32,
    by: u32,
    framebuffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: Rgb<u8>,
) {
    let steps = bx.abs_diff(ax).max(by.abs_diff(ay));

    if steps == 0 {
        framebuffer.put_pixel(ax, ay, color);
        return;
    }

    for i in 0..=steps {
        // t 是沿线段从 a 到 b 的进度，范围 [0, 1]
        let t = i as f32 / steps as f32;
        // 线性插值（lerp）：x = (1-t)*ax + t*bx，是端点的加权平均，
        // 因此结果必然落在 [min(ax,bx), max(ax,bx)] 内——不会为负，也不会超出画布。
        // 注意必须先转 f32 再相减：位移可以是负数，u32 相减下溢会直接 panic。
        let x = ax as f32 + (bx as f32 - ax as f32) * t;
        let y = ay as f32 + (by as f32 - ay as f32) * t;
        // 浮点截断会导致漏列：比如 62 - 50*(30/50) 在 f32 下算出 31.999998，
        // 直接 `as u32`（向零截断）会落到 31，第 32 列就没人画——线中间出现缺口。
        // 由于每步移动不超过 1 列，用 round() 四舍五入可保证相邻步落点相差至多 1 列。
        // 即使浮点误差让结果略微变负，Rust 的 `as` 是饱和转换（钳到 0），
        // 不会像 C++ 那样产生未定义行为。
        framebuffer.put_pixel(x.round() as u32, y.round() as u32, color);
    }
}

pub fn line_bresenham(
    mut ax: u32,
    mut ay: u32,
    mut bx: u32,
    mut by: u32,
    framebuffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: Rgb<u8>,
) {
    let steep = by.abs_diff(ay) > bx.abs_diff(ax);
    if steep {
        std::mem::swap(&mut ax, &mut ay);
        std::mem::swap(&mut bx, &mut by);
    }

    if ax > bx {
        std::mem::swap(&mut ax, &mut bx);
        std::mem::swap(&mut ay, &mut by);
    }

    let mut y = ay as i32;
    let mut ierror: i32 = 0;
    for x in ax..=bx {
        if steep {
            framebuffer.put_pixel(y as u32, x, color);
        } else {
            framebuffer.put_pixel(x, y as u32, color);
        }
        ierror += 2 * by.abs_diff(ay) as i32;
        let need_step = ierror > (bx - ax) as i32;
        y += if by > ay { 1 } else { -1 } * need_step as i32;
        ierror -= 2 * (bx - ax) as i32 * need_step as i32;
    }
}
