use crate::prelude::*;
mod entity_render;

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(entity_render::entity_render_system())
        .build()
}
