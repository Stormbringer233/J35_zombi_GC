mod scene_menu;
mod scene_game;
mod scene_worldmap;
mod scene_game_over;
mod base_scene;
pub mod resources;
use scene_menu::SceneMenu;
use scene_game::SceneGame;
use scene_worldmap::SceneWorldMap;
use scene_game_over::SceneGameOver;
use resources::Resources;

#[derive(Copy, Clone, Debug)]
pub enum Scene {
    Menu,
    Game,
    WorldMap,
    GameOver
}

pub struct State {
    state : Scene,
    resources : Resources,
    scene_menu : SceneMenu,
    scene_game : SceneGame,
    scene_worldmap : SceneWorldMap,
    scene_game_over : SceneGameOver
}

impl State {
    pub fn init(resources : Resources) -> State {
        let s = State {
            state : Scene::Menu,
            scene_menu : SceneMenu::init(&resources),
            scene_worldmap : SceneWorldMap{},
            scene_game : SceneGame::init(),
            scene_game_over : SceneGameOver{},
            resources,
        };
        s
    }

    pub fn update(&mut self) {
        
        match self.state {
            Scene::Menu => {
                self.scene_menu.inputs();
                self.state = self.scene_menu.update();
                self.scene_menu.draw();
            },
            Scene::WorldMap => {
                self.scene_worldmap.inputs();
                self.state = self.scene_worldmap.update();
                self.scene_worldmap.draw();
            }
            Scene::Game => {
                self.scene_game.inputs();
                self.state = self.scene_game.update();
                self.scene_game.draw();
            },
            Scene::GameOver => {
                self.scene_game_over.inputs();
                self.state = self.scene_game_over.update();
                self.scene_game_over.draw();
            }
        }
    }

    pub fn draw(&self) {

    }
}
