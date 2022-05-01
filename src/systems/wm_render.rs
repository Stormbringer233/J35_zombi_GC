use crate::prelude::*;

#[system]
#[read_component(Render)]
#[read_component(Vec2)]
pub fn wm_render(ecs : &SubWorld) {
    <(&Vec2, &Render)>::query()
    // .filter(component::<CityPointer>())
    .iter(ecs)
    .filter(|(_, render)| render.to_draw == true)
    .for_each(|(pos, render)| {
        // println!("text : {:?}", render.texture);
        // let scalew = render.texture.width() * 2.0;
        // let scaleh = render.texture.height() * 2.0;
        draw_texture_ex(
            render.texture,
            pos.x,
            pos.y,
            render.color,
            DrawTextureParams {
                // dest_size : Some(Vec2::new(scalew, scaleh)),
                ..Default::default()
            }
        );
    });
}