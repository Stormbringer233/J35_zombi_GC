use macroquad::prelude::*;
use crate::Scene;
use crate::base_scene::BaseScene;
use crate::loader::Loader;

pub struct SceneGame {
    next_scene : BaseScene,
}
impl SceneGame {
    pub fn init(loader : &Loader) -> Self {
        SceneGame {
            next_scene : BaseScene::new(Scene::Game),
        }
    }
    pub fn inputs(&mut self) {
        self.next_scene.reset_scene();
        if is_key_down(KeyCode::Space) {
            println!("press space into Game");
            self.next_scene.set_next_scene(Scene::Menu);
        }
    }
    pub fn update(&mut self) -> Scene {
        
        self.next_scene.get_next_scene()
    }
    pub fn draw(&self) {
        clear_background(YELLOW);
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_text("Scene game", 10.0,580.0,30.0,BLACK);
    }
}