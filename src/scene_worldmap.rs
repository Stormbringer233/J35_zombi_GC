use macroquad::prelude::*;
use crate::Scene;

pub struct SceneWorldMap {}
impl SceneWorldMap {

    pub fn inputs(&self) {}
    pub fn update(&mut self) -> Scene {
        
        Scene::WorldMap
    }
    pub fn draw(&self) {
        clear_background(ORANGE);
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_text("Scene world map", 10.0,580.0,30.0,BLACK);
    }
}