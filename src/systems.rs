use crate::prelude::*;
mod city_scope;
mod wm_render;

pub fn build_worldmap_system() -> Schedule {
    Schedule::builder()
        .add_system(city_scope::city_scope_system())
        .add_system(wm_render::wm_render_system())
        .build()
}
