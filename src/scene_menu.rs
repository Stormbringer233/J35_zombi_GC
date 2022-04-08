use macroquad::prelude::*;
use crate::Scene;
use crate::base_scene::BaseScene;
use crate::resources::Resources;

struct Line {
    pos : (f32,f32)
}
pub struct SceneMenu {
    next_scene : BaseScene,
    line : Line,
    tex : Texture2D,
    r : f32,
}

impl SceneMenu {
    pub fn init(resources : &Resources) -> Self {
        Self {
            next_scene : BaseScene::new(Scene::Menu),
            line : Line{pos : (20.0, 50.0)},
            tex : resources.get_texture_2d("policeman"),
            r : 0f32,
        }
    }

    pub fn inputs(&mut self) {
        self.next_scene.reset_scene();
        if is_mouse_button_down(MouseButton::Left) {
            self.next_scene.set_next_scene(Scene::Game);
        }
    }

    pub fn update(&mut self) -> Scene {
        self.line.pos.0 += 1.0;
        self.r += 0.02;
        self.next_scene.get_next_scene()
    }

    pub fn draw(&self) {
        // clear_background(RED);
        draw_text("Scene menu", 10.0,580.0,30.0,WHITE);
        draw_line(self.line.pos.0, self.line.pos.1, self.line.pos.0+40.0, self.line.pos.1+100.0, 15.0, BLUE);
        let mpos = mouse_position();
        draw_circle(mpos.0, mpos.1, 30.0, Color::from_rgba(100, 150, 20, 200));
        draw_text(format!("x : {}, y : {}", mpos.0, mpos.1).as_str(), 0.0,20.0,30.0,WHITE);
        draw_texture_ex(self.tex, 100.0,100.0,WHITE,
        DrawTextureParams { dest_size:None , source:None, rotation: self.r,
            flip_x: false, flip_y: false, pivot: Some(Vec2::new(210.,340.)) });
        draw_line(80.0,100.0,120.0,100.0,1.0,WHITE);
        draw_line(100.0,80.0,100.0,120.0,1.0,WHITE);
    }
}