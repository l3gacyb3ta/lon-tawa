pub const WIDTH: usize = 1512;
pub const HEIGHT: usize = 720;
pub const PITCH: usize = 1;

#[inline(always)]
pub fn plot_pixel(buffer: &mut [u32], x: usize, y: usize, color: u32) {
    if x > (WIDTH as usize - 1) || y > (HEIGHT as usize - 1) {
        return;
    }

    let offset_into_line = PITCH * x;
    buffer[(y * (WIDTH as usize) * PITCH) + offset_into_line] = color;
}

pub fn symetric_pixel(
    frame: &mut [u32],
    x0: f64,
    y0: f64,
    cx: f64,
    cy: f64,
    color: u32,
    top_right: bool,
    top_left: bool,
    bottom_right: bool,
    bottom_left: bool,
) {
    if top_left {
        plot_pixel(
            frame,
            (cx - x0).floor() as usize,
            (cy - y0).floor() as usize,
            color
        );
        plot_pixel(
            frame,
            (cx - y0).floor() as usize,
            (cy - x0).floor() as usize,
            color
        );
    }
    if bottom_right {
        plot_pixel(
            frame,
            (cx + y0).floor() as usize,
            (cy + x0).floor() as usize,
            color,
        );
        plot_pixel(
            frame,
            (cx + x0).floor() as usize,
            (cy + y0).floor() as usize,
            color,
        );
    }
    if bottom_left {
        plot_pixel(
            frame,
            (cx - x0).floor() as usize,
            (cy + y0).floor() as usize,
            color,
        );
        plot_pixel(
            frame,
            (cx - y0).floor() as usize,
            (cy + x0).floor() as usize,
            color,
        );
    }
    if top_right {
        plot_pixel(
            frame,
            (cx + y0).floor() as usize,
            (cy - x0).floor() as usize,
            color,
        );
        plot_pixel(
            frame,
            (cx + x0).floor() as usize,
            (cy - y0).floor() as usize,
            color,
        );
    }
}

pub fn plot_arc(
    frame: &mut [u32],
    x0: f64,
    y0: f64,
    radius: f64,
    color: u32,
    top_right: bool,
    top_left: bool,
    bottom_right: bool,
    bottom_left: bool,
) {
    let mut t1 = radius / 16.;
    let mut x = radius;
    let mut y = 0.;
    let mut t2: f64;

    while !(x < y) {
        symetric_pixel(
            frame,
            x,
            y,
            x0,
            y0,
            color,
            top_right,
            top_left,
            bottom_right,
            bottom_left,
        );

        y += 1.;
        t1 = t1 + y;
        t2 = t1 - x;
        if t2 >= 0. {
            t1 = t2;
            x -= 1.;
        }
    }
}