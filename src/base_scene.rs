use crate::Scene;

pub struct BaseScene {
    next_scene : Scene,
    base_scene : Scene,
}

impl BaseScene {
    pub fn new(base_scene : Scene) -> Self {
        BaseScene {
            next_scene : base_scene,
            base_scene : base_scene,
        }
    }
    pub fn set_next_scene(&mut self, next_scene : Scene) {
        self.next_scene = next_scene;
    }
    pub fn reset_scene(&mut self) {
        self.next_scene = self.base_scene;
    }
    pub fn get_next_scene(&self) -> Scene {
        match self.next_scene {
            Scene::Menu => Scene::Menu,
            Scene::Game => Scene::Game,
            Scene::WorldMap => Scene::WorldMap,
            Scene::GameOver => Scene::GameOver
        }
    }
}