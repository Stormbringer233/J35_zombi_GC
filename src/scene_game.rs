use crate::prelude::*;

pub struct SceneGame {
    render_target: RenderTarget,
    camera: Camera2D,
    worldmap_system: Schedule,
    district_system: Schedule,
}
impl SceneGame {
    pub fn init(loader: &Loader) -> Self {
        // spawn_zombi(&mut ecs, Vec2::new(350.0,150.0), loader.get_texture_2d("policeman"));

        let render_target = render_target(WINDOW_W as u32, WINDOW_H as u32);
        // d.add_func(0, Box::new(|| println!("ok_btn")));
        SceneGame {
            render_target,
            camera: Camera2D {
                zoom: vec2(2.0 / WINDOW_W as f32 * SCALE, 2.0 / WINDOW_H as f32 * SCALE),
                offset: vec2(-1.0, -1.0),
                render_target: Some(render_target),
                ..Default::default()
            },
            worldmap_system: build_worldmap_system(),
            district_system: build_district_system(),
        }
    }
    pub fn inputs(&mut self) {}

    pub fn update(&mut self, ecs: &mut World, resources: &mut Resources, game_phase: &GamePhase) {
        match *game_phase {
            GamePhase::WorldMap => {
                set_camera(&self.camera);
                clear_background(LIGHTGRAY);
                self.worldmap_system.execute(ecs, resources);
                set_default_camera();
            }
            GamePhase::District => {
                set_camera(&self.camera);
                clear_background(LIGHTGRAY);
                self.district_system.execute(ecs, resources);
                set_default_camera();
            }
            GamePhase::StreetView => {}
        }
    }

    pub fn draw(&self) {
        // Draw the render target
        clear_background(LIGHTGRAY);
        draw_texture(self.render_target.texture, 0.0, 0.0, WHITE);
    }
}
