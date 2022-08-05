use macroquad::prelude::*;
use crate::constants::FONT_SIZE1;
use crate::mouse::MouseStates;
use crate::widgets::*;

pub struct ButtonBuilder {
    id : usize,
    caption : Option<String>,
    caption_size : Option<TextDimensions>,
    position : Vec2,
    dimensions : Vec2,
    style : Style,
    color_theme : Colortheme,
}
impl IWidgetBuilder for ButtonBuilder {
    #[allow(dead_code)]
    fn caption(mut self, caption : &str) -> Self {
        self.caption = Some(caption.to_string());
        let td = measure_text(caption, None, FONT_SIZE1, 1.0);
        if self.dimensions.length() == 0.0 { // no dimensions was provided before calling this function
            self.dimensions = Vec2::new(
                td.width + H_TEXT_PADDING * 2.0,
                td.height + V_TEXT_PADDING * 2.0
            );
        }
        self.caption_size = Some(td);
        self
    }
    #[allow(dead_code)]
    fn dimensions(mut self, width : u32, height : u32) -> Self {
        self.dimensions = Vec2::new(width as f32, height as f32);
        self
    }
    #[allow(dead_code)]
    fn id(mut self, new_id : usize) -> Self {
        self.id = new_id;
        self
    }
    #[allow(dead_code)]
    fn position(mut self, x : i32, y : i32) -> Self {
        self.position = Vec2::new(x as f32, y as f32);
        self
    }
    #[allow(dead_code)]
    fn style(mut self, style : Style) -> Self {
        self.style = style;
        self
    }
    #[allow(dead_code)]
    fn with_theme(mut self, theme : Theme) -> Self {
        self.color_theme = Colortheme::new(theme);
        self
    }
}
impl ButtonBuilder {
    pub fn build(self) -> Button {
        Button {
            cd : CommonWidgetDatas{
                id : self.id,
                caption : self.caption,
                caption_size : self.caption_size,
                position : self.position,
                dimensions : self.dimensions,
                collide_box : Rect::new(self.position.x, self.position.y, self.dimensions.x, self.dimensions.y),
                style : self.style,
                color_theme : self.color_theme,
            },
            state : State::UnFocus,
            old_state : State::UnFocus,
            unselectable : false,
        }
    }
}
pub struct Button {
    cd : CommonWidgetDatas,
    state : State,
    old_state : State,
    unselectable : bool,
}

impl Button {
    pub fn builder() -> ButtonBuilder {
        ButtonBuilder {
            id : 0usize,
            caption : Some(String::new()),
            caption_size : None,
            position : Vec2::default(),
            dimensions : Vec2::default(),
            style : Style::Flat,
            color_theme : Colortheme::new(FLAT_GREY)
        }
    }
    fn draw_debug(&self, m_state : &MouseStates) {
        draw_rectangle(5.0,5.0,500.0,50.0,Color::from_rgba(255,255,255,150));
        draw_text(format!("mouse position : {:?}", m_state.position).as_str(), 10.0,20.0,20.0,BLACK);
        // draw_text(format!("left button click : {:?}", m_state.left_btn).as_str(), 5.0,40.0,20.0,WHITE);
        // draw_text(format!("button state : {:?}\t|\told_state : {:?}", self.state, self.old_state).as_str(),
        //             10.0,70.0,20.0,BLACK);
    }
}

impl Widgets for Button {
    fn is_over(&self, point : &(f32, f32)) -> bool {
        self.cd.collide_box.contains(Vec2::new(point.0, point.1))
    }
    fn id(&self) -> usize { self.cd.id }
    fn move_widget(&mut self, delta : &Vec2) {
        self.cd.position += *delta;
        self.cd.collide_box.offset(*delta);
    }
    fn update(&mut self, m_state : &MouseStates) -> (usize, Option<WidgetType>, Option<State>) {
        let mut state : Option<State> = None;
        if !self.is_over(&m_state.position) && !m_state.left_btn{ self.unselectable = false;}
        
        match self.old_state {
            State::UnFocus => {
                if self.is_over(&m_state.position) {
                    self.old_state = self.state;
                    self.state = State::Focus
                }
                if !self.is_over(&m_state.position) && m_state.left_btn{ self.unselectable = true;}
            },
            State::Focus => {
                if !self.is_over(&m_state.position) {
                    self.old_state = self.state;
                    self.state = State::UnFocus;
                }
                if self.is_over(&m_state.position) && !m_state.left_btn {
                    self.unselectable = false;
                }
                if m_state.left_btn && !self.unselectable {
                    self.state = State::Clicked;
                    self.old_state = self.state;
                }
            },
            State::Clicked => {
                if !m_state.left_btn && self.is_over(&m_state.position) {
                    self.state = State::Focus;
                    self.old_state = State::Focus;
                    state = Some(State::Clicked);
                    // return(self.cd.id, Some(WidgetType::Button), Some(State::Clicked));
                }
                if !m_state.left_btn && !self.is_over(&m_state.position) {
                    self.state = State::UnFocus;
                }
            },
        }
        self.draw_debug(m_state);
        match &self.cd.style {
            Style::Flat => {
                self.draw_flat(self.cd.get_theme(self.state));
            },
            Style::RoundFlat => {
                self.draw_round_flat(self.cd.get_theme(self.state));
            }
            Style::Skinned => {
                self.draw_skin(); // not implemented yet !
            }
        }
        // self.draw();
        (self.cd.id, Some(WidgetType::Button), state)
    }
    // fn draw(&self) {
    //     match &self.cd.style {
    //         Style::Flat(color_theme) => {
    //             self.draw(color_theme.get_theme(self.state));
    //         },
    //         Style::Relief(_color_theme) => {

    //         }
    //     }
        
        // let(fg, lup, lbottom, txt) = (self.cs.0, self.cs.1,self.cs.2,self.cs.3);
        // //background rectangle
        // draw_rectangle(
        //         self.rect.x, self.rect.y, self.rect.w, self.rect.h, Color::from_rgba(fg.0, fg.1, fg.2, 255)
        //     );
        // // up lines
        // draw_line(self.rect.x, self.rect.y, self.rect.x + self.rect.w, self.rect.y, 2.0, Color::from_rgba(lup.0, lup.1, lup.2, 255));
        // draw_line(self.rect.x, self.rect.y, self.rect.x, self.rect.y + self.rect.h, 2.0, Color::from_rgba(lup.0, lup.1, lup.2, 255));
        // // bottom lines
        // draw_line(self.rect.x, self.rect.y + self.rect.h, self.rect.x + self.rect.w, self.rect.y + self.rect.h, 2.0, Color::from_rgba(lbottom.0, lbottom.1, lbottom.2, 255));
        // draw_line(self.rect.x + self.rect.w, self.rect.y, self.rect.x + self.rect.w, self.rect.y + self.rect.h, 2.0, Color::from_rgba(lbottom.0, lbottom.1, lbottom.2, 255));
        // draw_text(
        //         &self.text,
        //         self.rect.x + self.rect.w / 2.0 - self.txt_size.width/2.,
        //         self.rect.y + self.rect.h / 2.0 + self.txt_size.height/2.,
        //         30.0,
        //         Color::from_rgba(txt.0, txt.1, txt.2,255)
        //     );
    }

impl MacroDraw for Button {
    fn draw_flat(&self, theme : (FullComposite, FullComposite, FullComposite, FullComposite)) {
        let (bg, fg, lt, lb) = (theme.0, theme.1, theme.2, theme.3);
        let (x,y) = (self.cd.position.x, self.cd.position.y);
        let (w,h) = (self.cd.dimensions.x, self.cd.dimensions.y);
        // background
        draw_rectangle(
            x, y, w, h,
            Color::from_rgba(bg.0, bg.1, bg.2, bg.3)
        );
        //liner
        draw_rectangle_liner(x, y, w, h, Color::from_rgba(lt.0, lt.1, lt.2, lt.3), Color::from_rgba(lb.0, lb.1, lb.2, lb.3));
        // text
        let td = self.cd.caption_size();
        let txt : &str = match self.cd.caption() {
            Some(t) => t,
            None => &""
        };
        draw_centered_text(txt, x, y, w, h, td, Color::from_rgba(fg.0, fg.1, fg.2, fg.3));
        // println!("pos : {:?} - button dim : {:?} - txt width : {}", self.cd.position,self.cd.dimensions, td.width);
        
    }

    fn draw_round_flat(&self, theme : (FullComposite, FullComposite, FullComposite, FullComposite)) {
        let (bg, fg, lt, lb) = (theme.0, theme.1, theme.2, theme.3);
        let (x,y) = (self.cd.position.x, self.cd.position.y);
        let (w,h) = (self.cd.dimensions.x, self.cd.dimensions.y);
        let radius = f32::max(h / 10.0, 2.0);
        let thickness = 2f32;
        draw_round_rectangle(x, y, w, h, radius, Color::from_rgba(bg.0, bg.1, bg.2, bg.3));
        draw_round_rectangle_line(x,y,w,h,radius, thickness, Color::from_rgba(lt.0, lt.1, lt.2, lt.3));
        let td = self.cd.caption_size();
        let txt : &str = match self.cd.caption() {
            Some(t) => t,
            None => &""
        };
        draw_centered_text(txt, x, y, w, h,td, Color::from_rgba(fg.0, fg.1, fg.2, fg.3));
    }
    fn draw_skin(&self) {
        todo!();
    }
}

fn draw_rectangle_liner(x:f32, y:f32, w:f32, h:f32, color_up : Color, color_bottom : Color) {
    const THICK:f32 = 2.0;
        draw_line(x, y, x+w, y, THICK, color_up); //up
        draw_line(x+w, y, x+w, y+h, THICK, color_bottom); // right
        draw_line(x+w, y+h, x, y+h, THICK, color_bottom); // bottom
        draw_line(x, y+h, x, y, THICK, color_up); // left
}

fn draw_centered_text(text:&str, x:f32, y:f32, w:f32, h:f32, td : &TextDimensions, color : Color) {
    draw_text(
        text,
        x + (w - td.width) / 2.0,
        y + (h + td.height) / 2.0,
        FONT_SIZE1 as f32,
        color
    );
}