use crate::prelude::*;

#[derive(Debug)]
pub struct Zombi;
pub struct Widget;
pub struct Id(pub usize);
pub struct WorldMapTexture;
#[derive(Debug)]
pub struct Render {
    pub texture: Texture2D,
    pub color: Color,
    pub to_draw: bool,
}
#[derive(Debug, Copy, Clone)]
pub enum GamePhase {
    WorldMap,
    District,
    StreetView,
}
#[derive(Debug, Copy, Clone)]
pub enum SceneState {
    Menu,
    Game(GamePhase),
}
pub struct Plot;
#[derive(Debug)]
pub struct CityPointer;
