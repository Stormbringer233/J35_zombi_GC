use crate::prelude::*;

pub fn spawn_zombi(ecs: &mut World, pos: Vec2, texture: Texture2D) {
    let rotate = 0.5f32;
    ecs.push((
        Zombi,
        pos,
        rotate,
        Render {
            texture,
            color: WHITE,
            to_draw: true,
        },
    ));
}
