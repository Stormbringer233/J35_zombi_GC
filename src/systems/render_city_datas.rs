use crate::prelude::*;

#[system]
pub fn render_city_datas(#[resource] map : &WorldMap) {
    for (pos, city) in &map.cities {
        
    }
}