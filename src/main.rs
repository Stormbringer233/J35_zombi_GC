use macroquad::prelude::*;
use jam35_zombi::resources::Resources;

#[macroquad::main("jam35 zombi")]
async fn main() {
    //jam35_zombi::running();
    let mut resources = Resources::new("assets");
    resources.load_resources().await;
    // let t : Texture2D = load_texture("assets/worldmap.png").await.unwrap();
    let mut state = jam35_zombi::State::init(resources);
    loop {
        state.update();
        // draw_texture(t, 150.,150.,WHITE);
        next_frame().await
    }
}
