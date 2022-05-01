use crate::prelude::*;
mod wm_render;
mod city_scope;
mod render_city_datas;


pub fn build_worldmap_system() -> Schedule {
    Schedule::builder()
    .add_system(city_scope::city_scope_system())
    .add_system(wm_render::wm_render_system())
    .add_system(render_city_datas::render_city_datas_system())
    .build()
}