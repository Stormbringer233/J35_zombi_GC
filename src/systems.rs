use crate::prelude::*;
mod city_scope;
mod test_district;
mod wm_gui;
mod wm_render;

pub fn build_worldmap_system() -> Schedule {
    Schedule::builder()
        .add_system(city_scope::city_scope_system())
        .add_system(wm_render::wm_render_system())
        .add_system(wm_gui::wm_gui_system())
        .build()
}

pub fn build_district_system() -> Schedule {
    Schedule::builder()
        .add_system(test_district::test_district_system())
        .build()
}

pub fn build_streetview_system() -> Schedule {
    Schedule::builder().build()
}
