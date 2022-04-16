pub use macroquad::prelude::*;
use jam35_zombi::loader::Loader;

#[macroquad::main("jam35 zombi")]
async fn main() {
    //jam35_zombi::running();
    let mut loader = Loader::new("assets");
    loader.load_resources().await;
    // let t : Texture2D = load_texture("assets/worldmap.png").await.unwrap();
    let mut state = jam35_zombi::State::init(loader);
    loop {
        state.update();
        // draw_texture(t, 150.,150.,WHITE);
        next_frame().await
    }
}
