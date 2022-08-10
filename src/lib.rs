mod components;
mod constants;
mod gui_creation;
pub mod loader;
mod map_pointer;
mod mouse;
mod scene_game;
mod scene_game_over;
mod scene_menu;
mod spawner;
mod systems;
mod widgets;
mod worldmap_creation;

mod prelude {
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub use macroquad::prelude::*;

    pub use crate::components::*;
    pub use crate::constants::*;
    pub use crate::gui_creation::*;
    pub use crate::loader::Loader;
    pub use crate::map_pointer::*;
    pub use crate::mouse::*;
    pub use crate::scene_game::SceneGame;
    pub use crate::scene_game_over::SceneGameOver;
    pub use crate::scene_menu::SceneMenu;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::widgets::*;
    pub use crate::worldmap_creation::*;
}
use prelude::*;

pub struct State {
    ecs: World,
    resources: Resources,
    scene_menu: SceneMenu,
    scene_game: SceneGame,
    // scene_game_over: SceneGameOver,
}

impl State {
    pub fn init(loader: Loader) -> State {
        let mut ecs = legion::World::default();
        let mut resources = Resources::default();
        State::init_scenes(&mut ecs, &mut resources, &loader);

        let s = State {
            ecs,
            resources,
            scene_menu: SceneMenu::init(&loader),
            scene_game: SceneGame::init(&loader),
            // scene_game_over: SceneGameOver {},
        };
        s
    }
    fn init_scenes(ecs: &mut World, resources: &mut Resources, loader: &Loader) {
        let worldmap = WorldMap::new();
        create_map(ecs, &worldmap, &loader);
        create_wm_widgets(ecs);
        resources.insert(worldmap);
        resources.insert(Mouse::init());
        resources.insert(SceneState::Game(GamePhase::WorldMap));
    }

    pub fn update(&mut self) {
        self.resources.insert(get_frame_time());
        let state = &self.resources.get::<SceneState>().unwrap().clone();
        match state {
            SceneState::Menu => {
                self.scene_menu.inputs();
                self.scene_menu.update();
                self.scene_menu.draw();
            }
            SceneState::Game(phase) => {
                self.scene_game.inputs();
                self.scene_game
                    .update(&mut self.ecs, &mut self.resources, &phase);
                self.scene_game.draw();
            } // Scene::GameOver => {
              //     self.scene_game_over.inputs();
              //     self.state = self.scene_game_over.update();
              //     self.scene_game_over.draw();
              // }
        }
    }
}
