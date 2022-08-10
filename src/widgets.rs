use crate::prelude::*;

pub mod base_widget;
pub mod button;
pub mod color_themes;
pub mod round_rectangle;

use self::base_widget::{Colortheme, IWidgetBuilder};
pub use self::color_themes::*;
use self::round_rectangle::*;

pub type FullComposite = (u8, u8, u8, u8);
const H_TEXT_PADDING: f32 = 40.0;
const V_TEXT_PADDING: f32 = 10.0;

pub trait Widgets {
    fn is_over(&self, point: &(f32, f32)) -> bool;
    fn id(&self) -> usize;
    fn move_widget(&mut self, delta: &Vec2);
    fn update(&mut self, mouse_state: &MouseStates) -> (usize, Option<WidgetType>, Option<State>);
    // fn draw(&self);
}
pub trait MacroDraw {
    fn draw_flat(&self, theme: (FullComposite, FullComposite, FullComposite, FullComposite));
    fn draw_round_flat(&self, theme: (FullComposite, FullComposite, FullComposite, FullComposite));
    fn draw_skin(&self);
}
pub enum Style {
    Flat,
    RoundFlat,
    Skinned,
}
pub enum WidgetType {
    Button,
    _ToggleButton(u8),
    _CheckButton(Vec<u8>),
    _RadioButton(u8),
}
#[derive(Debug, Copy, Clone)]
pub enum State {
    UnFocus,
    Focus,
    Clicked,
}

struct CommonWidgetDatas {
    id: usize,
    caption: Option<String>,
    caption_size: Option<TextDimensions>,
    position: Vec2,
    dimensions: Vec2,
    collide_box: Rect,
    style: Style,
    color_theme: Colortheme,
}
impl CommonWidgetDatas {
    pub fn caption(&self) -> Option<&str> {
        let c = self.caption.as_ref();
        Some(c.unwrap().as_str())
    }
    pub fn caption_size(&self) -> &TextDimensions {
        self.caption_size.as_ref().unwrap_or(&TextDimensions {
            width: 0.0,
            height: 0.0,
            offset_y: 0.0,
        })
    }
    pub fn get_theme(
        &self,
        state: State,
    ) -> (FullComposite, FullComposite, FullComposite, FullComposite) {
        self.color_theme.get_theme(state)
    }
}
