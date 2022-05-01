use crate::prelude::*;

pub fn create_map(ecs : &mut World, map : &WorldMap, loader : &Loader) {
    ecs.push(
        (
            WorldMapTexture,
            Vec2::new(0.0,0.0),
            Render {
                texture : loader.get_texture_2d("worldmap"),
                color : WHITE,
                to_draw : true
            }
        )
    );
    for (position, _) in &map.cities {
        let (p0, p1) = position.get();
        let pos = Vec2::new(p0- 6., p1 - 4.);
        ecs.push(
            (
                Plot,
                pos,
                Render {
                    texture : loader.get_texture_2d("plot"),
                    color : WHITE,
                    to_draw : true,
                }
            )
        );
        let pos = Vec2::new(p0 - 7., p1 - 35.);
        ecs.push(
            (
                CityPointer,
                pos,
                Render {
                    texture : loader.get_texture_2d("map_pointer"),
                    color : WHITE,
                    to_draw : false,
                }
            )
        );
    }
}

