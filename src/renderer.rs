// Author: Alberto González Palomo https://sentido-labs.com
// ©2019 Alberto González Palomo https://sentido-labs.com
// Released under the MIT license: https://opensource.org/licenses/MIT
#![allow(clippy::unusual_byte_groupings)]
use skia_safe::{
    gradient_shader, Color, Matrix, Paint, PaintJoin, PaintStyle, Path, Point, TileMode,
};
use std::cmp::min;

const PI: f32 = std::f32::consts::PI;
const DEGREES_IN_RADIANS: f32 = PI / 180.0;
const PEN_SIZE: f32 = 1.0;

fn point_in_circle(center: (f32, f32), radius: f32, radians: f32) -> (f32, f32) {
    (
        center.0 + radius * radians.cos(),
        center.1 - radius * radians.sin(),
    )
}

pub fn render_frame(canvas: &mut skia_safe::canvas::Canvas) {
    use skia_safe::shader::shaders::color;
    let mut path = Path::new();
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_stroke_width(PEN_SIZE.max(canvas.image_info().dimensions().width as f32 / 360.0));
    paint.set_style(PaintStyle::Stroke);
    paint.set_stroke_join(PaintJoin::Bevel);
    let c = (5.0, 5.0);
    let r = 10.0;
    paint.set_color(Color::WHITE);
    path.move_to((100, 100));
    path.line_to((200, 200));
    path.close();
    canvas.draw_path(&path, &paint);
    canvas.save();
}

#[allow(clippy::many_single_char_names)]
fn triangle(
    canvas: &mut skia_safe::canvas::Canvas,
    center: (i32, i32),
    radius: i32,
    degrees: f32,
    vertex: Option<i32>,
    color: Color,
    wankel: bool,
) {
    let c = (center.0 as f32, center.1 as f32);
    let r = radius as f32;
    let b = r * 0.9;
    let delta = 120.0 * DEGREES_IN_RADIANS;
    let side = r / ((PI - delta) / 2.0).cos() * 2.0;

    let mut alpha = degrees * DEGREES_IN_RADIANS;
    let mut path = Path::new();
    let mut paint = Paint::default();
    match vertex {
        Some(index) => {
            let a = (degrees + (120 * index) as f32) * DEGREES_IN_RADIANS;
            let center = point_in_circle(c, r, a);
            let radii = match index {
                0 | 2 => {
                    if wankel {
                        (0.36 * side, 0.404 * side)
                    } else {
                        (0.30 * side, 0.60 * side)
                    }
                }
                1 => {
                    if wankel {
                        (0.404 * side, 0.50 * side)
                    } else {
                        (0.420 * side, 0.50 * side)
                    }
                }
                i => panic!("Invalid vertex index {i} for triangle."),
            };
            gradient(&mut paint, center, radii, (color, Color::from(0x00_0000ff)))
        }
        None => {
            paint.set_anti_alias(true);
            paint.set_stroke_width(
                PEN_SIZE.max(canvas.image_info().dimensions().width as f32 / 360.0),
            );
            paint.set_style(PaintStyle::Stroke);
            paint.set_stroke_join(PaintJoin::Bevel);
            // Highlight reflection on the top triangle edge:
            paint.set_shader(gradient_shader::radial(
                (c.0, c.1 - 0.5 * r),
                0.5 * r,
                [Color::from(0xff_ffffff), color].as_ref(),
                None,
                TileMode::Clamp,
                None,
                None,
            ));
        }
    };
    for i in 0..4 {
        let v = point_in_circle(c, r, alpha);
        if i == 0 {
            path.move_to(v);
        } else if wankel {
            path.cubic_to(
                point_in_circle(c, b, alpha - 2.0 * delta / 3.0),
                point_in_circle(c, b, alpha - delta / 3.0),
                v,
            );
        } else {
            path.line_to(v);
        }
        alpha += delta;
    }
    path.close();
    canvas.draw_path(&path, &paint);
}

fn gradient(paint: &mut Paint, center: (f32, f32), radii: (f32, f32), colors: (Color, Color)) {
    let mut matrix = Matrix::scale((1.0, radii.1 / radii.0));
    matrix.post_translate((center.0, center.1));
    paint.set_shader(gradient_shader::radial(
        (0.0, 0.0),
        radii.0,
        [colors.0, colors.1].as_ref(),
        None,
        TileMode::Clamp,
        None,
        &matrix,
    ));
}
