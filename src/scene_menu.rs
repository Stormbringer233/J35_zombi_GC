use macroquad::prelude::*;
use crate::Scene;
use crate::base_scene::BaseScene;


struct Line {
    pos : (f32,f32)
}
pub struct SceneMenu {
    next_scene : BaseScene,
    line : Line,
}

impl SceneMenu {
    pub fn init() -> Self {
        Self {
            next_scene : BaseScene::new(Scene::Menu),
            line : Line{pos : (20.0, 50.0)}
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
        
        self.next_scene.get_next_scene()
    }

    pub fn draw(&self) {
        // clear_background(RED);
        draw_text("Scene menu", 10.0,580.0,30.0,WHITE);
        draw_line(self.line.pos.0, self.line.pos.1, self.line.pos.0+40.0, self.line.pos.1+100.0, 15.0, BLUE);
        let mpos = mouse_position();
        draw_circle(mpos.0, mpos.1, 30.0, Color::from_rgba(100, 150, 20, 200));
        draw_text(format!("x : {}, y : {}", mpos.0, mpos.1).as_str(), 0.0,20.0,30.0,WHITE);
    }
}