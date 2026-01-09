// Graphics utilities module for rendering primitives
#![allow(dead_code)]

use chinchilib::rgb::RGBA8;

pub fn draw_circle(
    frame: &mut [u8],
    width: usize,
    cx: usize,
    cy: usize,
    radius: usize,
    color: RGBA8,
) {
    for y in 0..=radius * 2 {
        for x in 0..=radius * 2 {
            let dx = x as i32 - radius as i32;
            let dy = y as i32 - radius as i32;
            if dx * dx + dy * dy <= (radius as i32) * (radius as i32) {
                let px = (cx as i32 + dx) as usize;
                let py = (cy as i32 + dy) as usize;
                set_pixel(frame, width, px, py, color);
            }
        }
    }
}

pub fn draw_line(
    frame: &mut [u8],
    width: usize,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    color: RGBA8,
) {
    let dx = (x1 as i32 - x0 as i32).abs();
    let dy = (y1 as i32 - y0 as i32).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = (dx - dy) as i32;

    let mut x = x0 as i32;
    let mut y = y0 as i32;

    loop {
        set_pixel(frame, width, x as usize, y as usize, color);

        if x as usize == x1 && y as usize == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy as i32 {
            err -= dy as i32;
            x += sx;
        }
        if e2 < dx as i32 {
            err += dx as i32;
            y += sy;
        }
    }
}

pub fn draw_rectangle(
    frame: &mut [u8],
    width: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    color: RGBA8,
) {
    for dy in 0..h {
        for dx in 0..w {
            set_pixel(frame, width, x + dx, y + dy, color);
        }
    }
}

pub fn set_pixel(frame: &mut [u8], width: usize, x: usize, y: usize, color: RGBA8) {
    if x < width && y * width + x < frame.len() / 4 {
        let idx = (y * width + x) * 4;
        if idx + 3 < frame.len() {
            frame[idx] = color.r;
            frame[idx + 1] = color.g;
            frame[idx + 2] = color.b;
            frame[idx + 3] = color.a;
        }
    }
}
