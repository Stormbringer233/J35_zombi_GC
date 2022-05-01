use crate::prelude::*;
use crate::Scene;
use crate::base_scene::BaseScene;

pub struct SceneGame {
    ecs : World,
    resources : Resources,
    render_target : RenderTarget,
    camera : Camera2D,
    next_scene : BaseScene,
    worldmap_system : Schedule,
}
impl SceneGame {
    pub fn init(loader : &Loader) -> Self {
        let mut ecs = legion::World::default();
        let mut resources = Resources::default();
        // spawn_zombi(&mut ecs, Vec2::new(350.0,150.0), loader.get_texture_2d("policeman"));
        let worldmap = WorldMap::new();
        create_map(&mut ecs, &worldmap, &loader);
        resources.insert(worldmap);
        resources.insert(Mouse::init());
        let render_target = render_target(WINDOW_W as u32, WINDOW_H as u32);
        SceneGame {
            ecs,
            resources,
            render_target,
            camera : Camera2D {
                zoom : vec2(2.0 / WINDOW_W as f32 * SCALE,2.0 / WINDOW_H as f32 * SCALE),
                offset : vec2(-1.0, -1.0),
                render_target : Some(render_target),
                ..Default::default()
            },
            next_scene : BaseScene::new(Scene::Game),
            worldmap_system : build_worldmap_system(),
        }
    }
    pub fn inputs(&mut self) {
        self.next_scene.reset_scene();
        if is_key_down(KeyCode::Space) {
            println!("press space into Game");
            self.next_scene.set_next_scene(Scene::Menu);
        }
    }
    pub fn update(&mut self) -> Scene {
        // self.resources.insert(mouse_position());
        // self.resources.insert(is_mouse_button_pressed(MouseButton::Left));
        // self.resources.insert(is_mouse_button_released(MouseButton::Left));
        // self.resources.insert(is_mouse_button_down(MouseButton::Left));
        set_camera(&self.camera);
            clear_background(LIGHTGRAY);
            self.worldmap_system.execute(&mut self.ecs, &mut self.resources);
            // self.player_system.execute(self.ecs.as_mut().unwrap(), self.resources.as_mut().unwrap());
        set_default_camera();
        self.next_scene.get_next_scene()
    }
    pub fn draw(&self) {
        clear_background(LIGHTGRAY);
        draw_texture(
            self.render_target.texture,
            0.0,
            0.0,
            WHITE
        );
    }
}