use crate::prelude::*;

pub struct Mouse {
    clicked : bool,
    drag : bool,
    time : f64,
    last_time : f64,
    click_duration : f64,
    drag_time : f64,
    // wheel ...
}

impl Mouse {
    pub fn init() -> Self {
        Mouse {
            clicked : false,
            drag : false,
            time : f64::default(),
            last_time : f64::default(),
            click_duration : f64::default(),
            drag_time : 5f64,
        }
    }
    pub fn position(&self) -> Vec2 {
        Vec2::from(mouse_position())
    }
    pub fn clicked(&self, button : MouseButton) -> bool {
        is_mouse_button_down(button)
    }
    pub fn released(&mut self, button : MouseButton) -> bool {
        let up = is_mouse_button_released(button);
        if up == true && self.clicked == true {
            self.clicked = false;
            self.drag = false;
            self.click_duration = 0.0;
            return true;
        }
        false
    }
    pub fn click_once(&mut self, button : MouseButton) -> bool {
        let down = self.clicked(button);
        if down == true && self.clicked == false {
            self.clicked = true;
            return true;
        }
        false
    }
    pub fn is_drag(&mut self) -> bool {
        let current_time = get_time();
        if is_mouse_button_down(MouseButton::Left) {
            self.click_duration += current_time - self.last_time;
            if self.click_duration >= self.drag_time {
                self.drag = true;
                return true;
            }
        }
        self.last_time = current_time;
        false
    }

}