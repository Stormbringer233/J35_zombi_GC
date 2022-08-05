use crate::prelude::*;
mod city_scope;
mod wm_render;
mod wm_gui;

pub fn build_worldmap_system() -> Schedule {
    Schedule::builder()
        .add_system(city_scope::city_scope_system())
        .add_system(wm_render::wm_render_system())
        .add_system(wm_gui::wm_gui_system())
        .build()
}

pub fn build_district_system() -> Schedule {
    Schedule::builder()

    .build()
}

pub fn build_streetview_system() -> Schedule {
    Schedule::builder()

    .build()
}