use crate::Scene;
use macroquad::prelude::*;

pub struct SceneGameOver {}
impl SceneGameOver {
    pub fn inputs(&self) {}
    pub fn update(&mut self) -> Scene {
        Scene::GameOver
    }

    pub fn draw(&self) {
        clear_background(BLACK);
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_text("Scene game over", 10.0, 580.0, 30.0, BLACK);
    }
}
