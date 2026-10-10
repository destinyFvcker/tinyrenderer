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

// 布雷森汉姆算法诞生于一个FPU还并不流行的年代，所以完全摒弃了浮点数运算
pub fn line_bresenham(
    mut ax: u32,
    mut ay: u32,
    mut bx: u32,
    mut by: u32,
    framebuffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: Rgb<u8>,
) {
    // 在核心循环之中，要绘制的这条线说实话更像是水平的而不是垂直的（也就是说
    // 每次的Delta不会超过1）可以从两点出发进行证明：
    //
    // 1. 和x/y轴夹角为45 degree的直线的斜率是1，这个时候可以推断出ax-bx以及ay-by
    // 之间的距离是相等的，那么任何小于或者大于这个比值的时候就会发生转置，让斜率小于1
    // 所以我们理想直线的斜率肯定是满足|m| <= 1的
    //
    // 2. 由于点1.，所以Delta永远不会超过1

    // 判断并转置，这里也可以理解为在选择采样点，因为我们如果选择比较陡峭的那个轴
    // 来进行采样的话，很可能会有采样点不足的问题
    let steep = by.abs_diff(ay) > bx.abs_diff(ax);
    if steep {
        std::mem::swap(&mut ax, &mut ay);
        std::mem::swap(&mut bx, &mut by);
    }

    // 这里实际上在调换两个点的位置，我们统一从左画到右，循环也比较好处理一些
    if ax > bx {
        std::mem::swap(&mut ax, &mut bx);
        std::mem::swap(&mut ay, &mut by);
    }

    // 设定好起始值
    let mut y = ay as i32;
    // 这里引入的这个error变量一开始实际上是一个float值，用于衡量y轴是不是应该步进了
    // 我们上面讨论得到的是斜率肯定不会超过1，那么久应该在累加值四舍五入=1的时候y发生步进
    // 而现在为了不引入浮点计算，对表达式进行了一些变换处理。
    //
    // 理想的直线是斜率为 Delta y / Delta x 的只想，x每前进一个，那y就应该上升（下降）
    // Delta y / Delta x这么多个单位（后面使用dy和dx来进行指代）
    //
    // 之前的步进要求是这样的：error: f32 > 0.5
    // 那两边乘以 2dx 得到：2 * dx * error > Delta x
    //
    // 之前的error累加要求是这样的： error += dy/dx
    // 那两边乘以 2dx 得到：2 * dx *  error += 2 * dy
    //
    // 之前y步进了之后error要进行回退，也就是：error -= 1
    // 两边乘以 2dx 得到：2 * dx * error -= 2 * 2d
    //
    // 这里的 2 * dx * error我们统称为ierror，使用一个证书来进行表示就好了
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

pub fn scan_rasterization(
    ax: u32,
    ay: u32,
    bx: u32,
    by: u32,
    cx: u32,
    cy: u32,
    framebuffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: Rgb<u8>,
) {
    let (mut ax, mut ay, mut bx, mut by, mut cx, mut cy) = (
        ax as i32, ay as i32, bx as i32, by as i32, cx as i32, cy as i32,
    );

    if ay > by {
        std::mem::swap(&mut ax, &mut bx);
        std::mem::swap(&mut ay, &mut by);
    }
    if ay > cy {
        std::mem::swap(&mut ax, &mut cx);
        std::mem::swap(&mut ay, &mut cy);
    }
    if by > cy {
        std::mem::swap(&mut bx, &mut cx);
        std::mem::swap(&mut by, &mut cy);
    }

    let total_height = (cy - ay) as f32;
    if ay != by {
        let segment_height = (by - ay) as f32;
        for y in ay..=by {
            let x_a = ax + ((cx - ax) as f32 * (y - ay) as f32 / total_height).round() as i32;
            let x_b1 = ax + ((bx - ax) as f32 * (y - ay) as f32 / segment_height).round() as i32;
            for x in x_a.min(x_b1)..=x_a.max(x_b1) {
                framebuffer.put_pixel(x as u32, y as u32, color);
            }
        }
    }

    if by != cy {
        let segment_height = (cy - by) as f32;
        for y in by..=cy {
            let x_a = ax + ((cx - ax) as f32 * (y - ay) as f32 / total_height).round() as i32;
            let x_b2 = bx + ((cx - bx) as f32 * (y - by) as f32 / segment_height).round() as i32;
            for x in x_a.min(x_b2)..=x_a.max(x_b2) {
                framebuffer.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

pub fn bounding_box_rasterization(
    ax: u32,
    ay: u32,
    bx: u32,
    by: u32,
    cx: u32,
    cy: u32,
    framebuffer: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: Rgb<u8>,
) {
    let bounding_box = bounding_box(ax, ay, bx, by, cx, cy);
    let total_area = signed_triangle_area(ax, ay, bx, by, cx, cy);

    for x in bounding_box.x_left..=bounding_box.x_right {
        for y in bounding_box.y_bottom..=bounding_box.y_top {
            // alpha: B → C     ← 顺着循环
            // beta:  C → A     ← 顺着循环（C 之后绕回 A）
            // gamma: A → B     ← 顺着循环
            let alpha = signed_triangle_area(x, y, bx, by, cx, cy) / total_area;
            let beta = signed_triangle_area(x, y, cx, cy, ax, ay) / total_area;
            let gamma = signed_triangle_area(x, y, ax, ay, bx, by) / total_area;

            if alpha < 0. || beta < 0. || gamma < 0. {
                // negative barycentric coordinate => the pixel is outside the triangle
                continue;
            }
            framebuffer.put_pixel(x, y, color);
        }
    }
}

struct BoundingBox {
    x_right: u32,
    y_top: u32,
    x_left: u32,
    y_bottom: u32,
}

fn signed_triangle_area(ax: u32, ay: u32, bx: u32, by: u32, cx: u32, cy: u32) -> f32 {
    let (ax, ay, bx, by, cx, cy) = (
        ax as i32, ay as i32, bx as i32, by as i32, cx as i32, cy as i32,
    );
    return 0.5_f32 * (ax * by - ay * bx + bx * cy - by * cx + cx * ay - cy * ax) as f32;
}

fn bounding_box(ax: u32, ay: u32, bx: u32, by: u32, cx: u32, cy: u32) -> BoundingBox {
    let x_right = ax.max(bx.max(cx));
    let y_top = ay.max(by.max(cy));
    let x_left = ax.min(bx.min(cx));
    let y_bottom = ay.min(by.min(cy));

    BoundingBox {
        x_right,
        y_top,
        x_left,
        y_bottom,
    }
}
