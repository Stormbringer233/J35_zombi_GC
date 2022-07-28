pub use macroquad::prelude::*;
use crate::mouse::MouseStates;

pub trait Widgets {
    fn is_over(&self, point : &(f32, f32)) -> bool;
    fn id(&self) -> usize;
    fn move_widget(&mut self, delta : &Vec2);
    fn update(&mut self, mouse_state : &MouseStates) -> (usize, Option<WidgetType>, Option<State>);
    fn draw(&self);
}

const H_TEXT_PADDING : f32 = 40.0;
const V_TEXT_PADDING : f32 = 20.0;
struct ColorScheme((u8,u8,u8),(u8,u8,u8),(u8,u8,u8),(u8,u8,u8));

pub enum WidgetType {
    Button,
    _ToggleButton(u8),
    _CheckButton(Vec<u8>),
    _RadioButton(u8)
}
#[derive(Debug, Copy, Clone)]
pub enum State {
    UnFocus,
    Focus,
    Clicked,
}

pub struct Button {
    id : usize,
    rect : Rect,
    text : String,
    state : State,
    old_state : State,
    cs : ColorScheme,
    txt_size : TextDimensions,
    unselectable : bool,
}

impl Button {
    pub fn new(id : usize,position:Vec2, text : &str) -> Self {
        let text_size = measure_text(text, None, 30, 1.0);
        Button {
            id,
            rect : Rect::new(position.x, position.y, text_size.width + 2.0*H_TEXT_PADDING, text_size.height + 2.0*V_TEXT_PADDING),
            text : String::from(text),
            state : State::UnFocus,
            old_state : State::UnFocus,
            cs : ColorScheme((200,200,200),(250,250,250),(100,100,100),(50,50,50)),
            txt_size : text_size,
            unselectable : false,
        }
    }
}

impl Widgets for Button {
    fn is_over(&self, point : &(f32, f32)) -> bool {
        self.rect.contains(Vec2::new(point.0, point.1))
    }
    fn id(&self) -> usize { self.id }
    fn move_widget(&mut self, delta : &Vec2) {
        self.rect.offset(*delta);
    }
    fn update(&mut self, m_state : &MouseStates) -> (usize, Option<WidgetType>, Option<State>) {
        if !self.is_over(&m_state.position) && !m_state.left_btn{ self.unselectable = false;}
        
        match self.old_state {
            State::UnFocus => {
                if self.is_over(&m_state.position) {
                    self.state = State::Focus
                }
                if !self.is_over(&m_state.position) && m_state.left_btn{ self.unselectable = true;}
            },
            State::Focus => {
                if !self.is_over(&m_state.position) {self.state = State::UnFocus;}
                if self.is_over(&m_state.position) && !m_state.left_btn {self.unselectable = false;}
                if m_state.left_btn && !self.unselectable {
                    self.state = State::Clicked;
                }
            },
            State::Clicked => {
                if !m_state.left_btn && self.is_over(&m_state.position) {
                    self.state = State::UnFocus;
                    self.old_state = State::UnFocus;
                    return(self.id, Some(WidgetType::Button), Some(State::Clicked));
                }
                if !m_state.left_btn && !self.is_over(&m_state.position) {
                    self.state = State::UnFocus;
                }
            },
        }
        draw_text(format!("mouse position : {:?}", m_state.position).as_str(), 5.0,20.0,20.0,WHITE);
        // draw_text(format!("left button click : {:?}", m_state.left_btn).as_str(), 5.0,40.0,20.0,WHITE);
        draw_text(format!("button state : {:?}\told_state : {:?}", self.state, self.old_state).as_str(), 5.0,60.0,20.0,WHITE);

        self.old_state = self.state;
        match self.state {
            State::UnFocus => self.cs = ColorScheme((200,200,200),(250,250,250),(100,100,100),(50,50,50)),
            State::Focus => self.cs = ColorScheme((220,220,220),(250,250,250),(150,150,150),(100,100,100)),
            State::Clicked => self.cs = ColorScheme((100,100,100),(150,150,150),(200,200,200),(50,50,50)),
        };
        self.draw();
        (self.id, Some(WidgetType::Button), None)
    }
    fn draw(&self) {
        let(fg, lup, lbottom, txt) = (self.cs.0, self.cs.1,self.cs.2,self.cs.3);
        //background rectangle
        draw_rectangle(
                self.rect.x, self.rect.y, self.rect.w, self.rect.h, Color::from_rgba(fg.0, fg.1, fg.2, 255)
            );
        // up lines
        draw_line(self.rect.x, self.rect.y, self.rect.x + self.rect.w, self.rect.y, 2.0, Color::from_rgba(lup.0, lup.1, lup.2, 255));
        draw_line(self.rect.x, self.rect.y, self.rect.x, self.rect.y + self.rect.h, 2.0, Color::from_rgba(lup.0, lup.1, lup.2, 255));
        // bottom lines
        draw_line(self.rect.x, self.rect.y + self.rect.h, self.rect.x + self.rect.w, self.rect.y + self.rect.h, 2.0, Color::from_rgba(lbottom.0, lbottom.1, lbottom.2, 255));
        draw_line(self.rect.x + self.rect.w, self.rect.y, self.rect.x + self.rect.w, self.rect.y + self.rect.h, 2.0, Color::from_rgba(lbottom.0, lbottom.1, lbottom.2, 255));
        draw_text(
                &self.text,
                self.rect.x + self.rect.w / 2.0 - self.txt_size.width/2.,
                self.rect.y + self.rect.h / 2.0 + self.txt_size.height/2.,
                30.0,
                Color::from_rgba(txt.0, txt.1, txt.2,255)
            );
    }

    
}