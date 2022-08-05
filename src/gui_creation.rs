use crate::prelude::*;
use crate::widgets::base_widget::IWidgetBuilder;

pub fn create_wm_widgets(ecs : &mut World) {
    // create worldmap buttons
    ecs.push((
        Widget,
        Id(0),
        button::Button::builder()
            .position(10, WINDOW_H - 50)
            .style(Style::RoundFlat)
            .with_theme(FLAT_BLUE)
            // .dimensions(250, 75)
            .caption("Return to menu")
            .build()
    ));
    ecs.push((
        Widget,
        Id(1),
        button::Button::builder()
            .position(WINDOW_W - 280,WINDOW_H - 50)
            .with_theme(FLAT_BLUE)
            .caption("Go to District")
            .build()
    ));
    ecs.push((
        Widget,
        Id(2),
        button::Button::builder()
            .position(WINDOW_W - 40 - 10,10)
            .dimensions(40,40)
            .with_theme(FLAT_RED_EXIT)
            .caption("X").build()
    ));
}