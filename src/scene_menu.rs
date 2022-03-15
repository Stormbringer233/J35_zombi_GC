use macroquad::prelude::*;
use crate::Scene;

struct Line {
    pos : (f32,f32)
}
pub struct SceneMenu {
    next_scene : Scene,
    line : Line,
}

impl SceneMenu {
    pub fn init() -> Self {
        Self {
            next_scene : Scene::Menu,
            line : Line{pos : (20.0, 50.0)}
        }
    }

    pub fn inputs(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            println!("mouse button left down");
            self.next_scene = Scene::Game;
        }
    }

    pub fn update(&mut self) -> Scene {
        self.line.pos.0 += 1.0;
        
        match self.next_scene {
            Scene::Game => Scene::Game,
            Scene::GameOver => Scene::GameOver,
            _ => Scene::Menu
        }
    }

    pub fn draw(&self) {
        // clear_background(RED);
        draw_text("Scene menu", 10.0,580.0,30.0,WHITE);
        draw_line(self.line.pos.0, self.line.pos.1, self.line.pos.0+40.0, self.line.pos.1+100.0, 15.0, BLUE);
        let mpos = mouse_position();
        draw_circle(mpos.0, mpos.1, 30.0, Color::from_rgba(100, 150, 20, 150));
        draw_text(format!("x : {}, y : {}", mpos.0, mpos.1).as_str(), 0.0,20.0,30.0,WHITE);
    }
}