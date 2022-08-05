use crate::prelude::*;

#[system]
#[read_component(Widget)]
#[read_component(Id)]
#[write_component(button::Button)]
pub fn wm_gui(ecs : &mut SubWorld,
    #[resource] mouse: &mut Mouse) {
    <(&Id, &mut button::Button)>::query()
    .iter_mut(ecs)
    .for_each(|(id, btn)| {
        match id {
            Id(0) => {
                let state = btn.update(&mouse.states());
                if let Some(State::Clicked) = state.2 {
                    println!("return to menu")
                }
            },
            Id(1) => {
                let state = btn.update(&mouse.states());
                    if let Some(State::Clicked) = state.2 {
                        println!("go to district")
                    }
                },
            Id(2) => {
                let state = btn.update(&mouse.states());
                if let Some(State::Clicked) = state.2 {
                    println!("Quit !");
                }
            },
            _ => ()
        }
    });
}