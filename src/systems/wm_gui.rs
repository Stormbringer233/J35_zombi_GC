use crate::prelude::*;

#[system]
#[read_component(Widget)]
#[read_component(Id)]
#[write_component(button::Button)]
pub fn wm_gui(
    ecs: &mut SubWorld,
    #[resource] mouse: &mut Mouse,
    #[resource] game_state: &mut SceneState,
    #[resource] time : &f32
) {
    draw_text(format!("time : {}", time).as_str(), 10.0,150.0,30.,BLACK);
    <(&Id, &mut button::Button)>::query()
        .iter_mut(ecs)
        .for_each(|(id, btn)| match id {
            Id(0) => {
                let state = btn.update(&mouse.states());
                if let Some(State::Clicked) = state.2 {
                    println!("return to menu");
                    *game_state = SceneState::Menu;
                }
            }
            Id(1) => {
                let state = btn.update(&mouse.states());
                if let Some(State::Clicked) = state.2 {
                    println!("go to district");
                    *game_state = SceneState::Game(GamePhase::District);
                }
            }
            Id(2) => {
                let state = btn.update(&mouse.states());
                if let Some(State::Clicked) = state.2 {
                    println!("Quit !");
                    *game_state = SceneState::Menu;
                }
            }
            _ => (),
        });
}
