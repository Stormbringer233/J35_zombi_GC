use macroquad::prelude::*;

fn draw_arc(
    cx: f32,
    cy: f32,
    angle_deb: f32,
    angle_fin: f32,
    precision: u32,
    thickness: f32,
    radius: f32,
    color: Color,
) {
    let angle_deb = angle_deb.to_radians();
    let angle_fin = angle_fin.to_radians();
    let frac_angle = (angle_fin - angle_deb) / precision as f32;
    let mut a0 = angle_deb;
    let mut a1 = angle_deb + frac_angle;
    // draw_line(center.x, cy, center.x + radius * angle_deb.cos(), cy + radius * angle_deb.sin(), 1., RED);
    // draw_line(center.x, cy, center.x + radius * angle_fin.cos(), cy + radius * angle_fin.sin(), 1., RED);
    for _ in 0..precision {
        let p0: [f32; 2] = [cx + radius * a0.cos(), cy + radius * a0.sin()];
        let p1 = [cx + radius * a1.cos(), cy + radius * a1.sin()];
        draw_line(p0[0], p0[1], p1[0], p1[1], thickness, color);
        a0 = a1;
        a1 = a0 + frac_angle;
    }
}

pub fn draw_round_rectangle_line(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radius: f32,
    thickness: f32,
    color: Color,
) {
    let angles: [f32; 4] = [0.0, 90.0, 180.0, 270.0];
    let arc_pos: [[f32; 2]; 4] = [
        [x + width - radius, y + height - radius],
        [x + radius, y + height - radius],
        [x + radius, y + radius],
        [x + width - radius, y + radius],
    ];
    let line_pos: [[f32; 4]; 4] = [
        [x + width - radius, y + height, x + radius, y + height],
        [x, y + height - radius, x, y + radius],
        [x + radius, y, x + width - radius, y],
        [x + width, y + radius, x + width, y + height - radius],
    ];
    let precision = u32::max((radius / 5.0) as u32, 2);
    let mut n = 0;
    while n < 4 {
        let a0 = angles[n];
        let a1 = angles[n] + 90.0;
        draw_arc(
            arc_pos[n][0],
            arc_pos[n][1],
            a0,
            a1,
            precision,
            thickness,
            radius,
            color,
        );
        draw_line(
            line_pos[n][0],
            line_pos[n][1],
            line_pos[n][2],
            line_pos[n][3],
            thickness,
            color,
        );
        n += 1;
    }
}
pub fn draw_round_rectangle(x: f32, y: f32, width: f32, height: f32, radius: f32, color: Color) {
    let mut n = 0;
    let circle_pos: [[f32; 2]; 4] = [
        [x + radius, y + radius],
        [x + width - radius, y + radius],
        [x + width - radius, y + height - radius],
        [x + radius, y + height - radius],
    ];
    while n < 4 {
        draw_circle(circle_pos[n][0], circle_pos[n][1], radius, color);
        n += 1;
    }
    let (w, h) = (width, height);
    let r = radius;
    draw_rectangle(x, y + radius, w, h - 2.0 * r, color);
    draw_rectangle(x + r, y, w - 2.0 * r, h, color);
}
