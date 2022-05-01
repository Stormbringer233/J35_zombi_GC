pub use macroquad::prelude::*;
use jam35_zombi::loader::Loader;
mod constants;
use constants::*;

fn window_conf() -> Conf {
    Conf { window_title: "jam35 zombi".to_owned(),
         window_width: WINDOW_W,
         window_height: WINDOW_H,
         window_resizable : false,
         ..Default::default()
    }
}
#[macroquad::main(window_conf)]
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
