use crate::prelude::*;

#[system]
#[read_component(Vec2)] // position
#[read_component(Render)] // texture
#[write_component(f32)] // angle
pub fn entity_render(ecs : &mut SubWorld) {
    <(&Vec2, &Render, &mut f32)>::query()
    .iter_mut(ecs)
    .for_each(|(pos, render, rotate)| {
        *rotate += 0.01;
        draw_texture_ex(
            render.texture,
            pos.x,
            pos.y,
            render.color,
            DrawTextureParams{
                rotation : *rotate,
                ..Default::default()
            }
    )});
}