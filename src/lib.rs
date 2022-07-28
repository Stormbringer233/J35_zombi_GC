mod base_scene;
mod components;
mod constants;
pub mod loader;
mod map_pointer;
mod mouse;
mod scene_game;
mod scene_game_over;
mod scene_menu;
mod scene_worldmap;
mod spawner;
mod systems;
mod gui;
mod worldmap_creation;

mod prelude {
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub use macroquad::prelude::*;

    pub use crate::gui::*;
    pub use crate::components::*;
    pub use crate::constants::*;
    pub use crate::loader::Loader;
    pub use crate::map_pointer::*;
    pub use crate::mouse::*;
    pub use crate::scene_game::SceneGame;
    pub use crate::scene_game_over::SceneGameOver;
    pub use crate::scene_menu::SceneMenu;
    pub use crate::scene_worldmap::SceneWorldMap;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::worldmap_creation::*;
}
use prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum Scene {
    Menu,
    Game,
    GameOver,
}

pub struct State {
    ecs : World,
    resources : Resources,
    state: Scene,
    scene_menu: SceneMenu,
    scene_game: SceneGame,
    scene_game_over: SceneGameOver,
}

impl State {
    pub fn init(loader: Loader) -> State {
        let mut ecs = legion::World::default();
        let mut resources = Resources::default();
        State::init_scenes(&mut ecs, &mut resources, &loader);

        let s  = State {
            ecs,
            resources,
            state: Scene::Game,
            scene_menu: SceneMenu::init(&loader),
            scene_game: SceneGame::init(&loader),
            scene_game_over: SceneGameOver {},
        };
        s
    }
    fn init_scenes(ecs : &mut World, resources : &mut Resources, loader : &Loader) {
        let worldmap = WorldMap::new();
        create_map(ecs, &worldmap, &loader);
        resources.insert(worldmap);
        resources.insert(Mouse::init());
    }

    pub fn update(&mut self) {
        match self.state {
            Scene::Menu => {
                self.scene_menu.inputs();
                self.state = self.scene_menu.update();
                self.scene_menu.draw();
            }
            Scene::Game => {
                self.scene_game.inputs();
                self.state = self.scene_game.update(&mut self.ecs, &mut self.resources);
                self.scene_game.draw();
            }
            Scene::GameOver => {
                self.scene_game_over.inputs();
                self.state = self.scene_game_over.update();
                self.scene_game_over.draw();
            }
        }
    }


}
