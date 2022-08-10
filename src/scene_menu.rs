use crate::prelude::*;

struct Line {
    pos: (f32, f32),
}
pub struct SceneMenu {
    line: Line,
    tex: Texture2D,
    r: f32,
}

impl SceneMenu {
    pub fn init(loader: &Loader) -> Self {
        Self {
            line: Line { pos: (20.0, 50.0) },
            tex: loader.get_texture_2d("policeman"),
            r: 0f32,
        }
    }

    pub fn inputs(&mut self) {}

    pub fn update(&mut self) {
        self.line.pos.0 += 1.0;
        self.r += 0.02;
    }

    pub fn draw(&mut self) {
        // clear_background(RED);
        let pos = (150f32, 100f32);
        let pos_rot = (pos.0 + 110.0, pos.1 + 240.0);
        draw_text("Scene menu", 10.0, 580.0, 30.0, WHITE);
        draw_line(
            self.line.pos.0,
            self.line.pos.1,
            self.line.pos.0 + 40.0,
            self.line.pos.1 + 100.0,
            15.0,
            BLUE,
        );
        let mpos = mouse_position();
        draw_circle(mpos.0, mpos.1, 30.0, Color::from_rgba(100, 150, 20, 200));
        draw_text(
            format!("x : {}, y : {}", mpos.0, mpos.1).as_str(),
            0.0,
            20.0,
            30.0,
            WHITE,
        );

        draw_texture_ex(
            self.tex,
            pos.0,
            pos.1,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                source: Some(Rect::new(50., 10., 150., 180.)),
                rotation: self.r,
                flip_x: false,
                flip_y: false,
                pivot: Some(Vec2::new(pos_rot.0, pos_rot.1)),
            },
        );

        draw_line(
            pos_rot.0 - 20.0,
            pos_rot.1,
            pos_rot.0 + 20.0,
            pos_rot.1,
            1.0,
            WHITE,
        );
        draw_line(
            pos_rot.0,
            pos_rot.1 - 20.0,
            pos_rot.0,
            pos_rot.1 + 20.0,
            1.0,
            WHITE,
        );
    }
}
