use macroquad::prelude::*;

#[macroquad::main("jam35 zombi")]
async fn main() {
    //jam35_zombi::running();
   let mut state = jam35_zombi::State::init();
    loop {
        state.update();

        next_frame().await
    }
}
