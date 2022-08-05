use macroquad::prelude::*;

struct BasicStats {
    speed : (f32, f32),
    pv : u32,
    defense : u32,
    attack : u32,
    zombi : u32,
    detect_radius : f32,
}
pub enum GamePhase {
    Worldmap,
    District,
    Streetview
}
pub enum EntityType {
    Civil,
    Military,
    Zombi
}

enum Behaviors {
    Fairful,
    Normal,
    Brave
}

enum AttackPriority {
    Civil(String, bool),
    Military(String, bool),
    Zombi(String, bool)
}

pub struct Entity {
    entity_type : EntityType,
    behavior : Behaviors,
}