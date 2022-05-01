use crate::prelude::*;

#[system]
#[write_component(Render)]
#[read_component(Vec2)]
pub fn city_scope(ecs : &mut SubWorld,#[resource] map : &mut WorldMap,
                    #[resource] mouse : &mut Mouse
            ) {
    <(&Vec2, &mut Render)>::query()
    .filter(component::<CityPointer>())
    .iter_mut(ecs)
    .for_each(|(pos, render)| {
        let over = map.over_city(&mouse.position());
        // println!("position {:?} is over {}", mouse.position(), over);
        if over == Vec2::new(pos.x + 7., pos.y + 35.) {
            render.to_draw = true;
            if mouse.click_once(MouseButton::Left) {
                map.select_city(&over);
                println!("datas  from city : {:?}", map.datas_from_selected_city());
            }
            mouse.released(MouseButton::Left);
        } else {
            render.to_draw = false;
        }
    });
}