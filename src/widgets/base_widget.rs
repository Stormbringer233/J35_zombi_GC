use crate::widgets::*;

pub trait IWidgetBuilder {
    fn caption(self, caption: &str) -> Self;
    fn dimensions(self, width: u32, height: u32) -> Self;
    fn id(self, id: usize) -> Self;
    fn position(self, x: i32, y: i32) -> Self;
    fn style(self, style: Style) -> Self;
    fn with_theme(self, theme: Theme) -> Self;
}
pub struct WidgetColor {
    background: FullComposite,
    foreground: FullComposite,
    top_line: FullComposite,
    bottom_line: FullComposite,
}
impl WidgetColor {
    pub fn new(
        compo: (
            LightComposite,
            LightComposite,
            LightComposite,
            LightComposite,
        ),
    ) -> Self {
        let background: FullComposite = (compo.0 .0, compo.0 .1, compo.0 .2, 255u8);
        let foreground: FullComposite = (compo.1 .0, compo.1 .1, compo.1 .2, 255u8);
        let top_line: FullComposite = (compo.2 .0, compo.2 .1, compo.2 .2, 255u8);
        let bottom_line: FullComposite = (compo.3 .0, compo.3 .1, compo.3 .2, 255u8);
        WidgetColor {
            background,
            foreground,
            top_line,
            bottom_line,
        }
    }
    pub fn composits(&self) -> (FullComposite, FullComposite, FullComposite, FullComposite) {
        (
            self.background,
            self.foreground,
            self.top_line,
            self.bottom_line,
        )
    }
}
pub struct ColorState {
    unfocus: WidgetColor,
    focus: WidgetColor,
    clicked: WidgetColor,
}

pub struct Colortheme(ColorState);
impl Colortheme {
    pub fn new(theme: Theme) -> Self {
        Colortheme(ColorState {
            unfocus: WidgetColor::new(theme.0 .0),
            focus: WidgetColor::new(theme.0 .1),
            clicked: WidgetColor::new(theme.0 .2),
        })
    }
    pub fn get_theme(
        &self,
        state: State,
    ) -> (FullComposite, FullComposite, FullComposite, FullComposite) {
        match state {
            State::UnFocus => self.0.unfocus.composits(),
            State::Focus => self.0.focus.composits(),
            State::Clicked => self.0.clicked.composits(),
        }
    }
}
