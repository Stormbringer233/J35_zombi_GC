pub type LightComposite = (u8, u8, u8);


pub struct Theme(
    pub (
        (LightComposite, LightComposite, LightComposite, LightComposite),   
        (LightComposite, LightComposite, LightComposite, LightComposite),   
        (LightComposite, LightComposite, LightComposite, LightComposite),   
    )
);

// LightComposite color theme with (
    // (theme for unfocus state) -> (background, foreground, top_line, bottom_line)
    // (theme for focus state)
    // (theme for clicked state)
//)
pub const FLAT_GREY : Theme = Theme((
    ((200,200,200),(50,50,50),(200,200,200),(200,200,200)),
    ((50,50,50),(255,255,255),(50,50,50),(50,50,50)),
    ((50,50,50),(255,255,255),(50,50,50),(50,50,50)),
));
pub const FLAT_BLUE : Theme = Theme((
    ((255,255,255),(0,0,255),(0,0,255),(0,0,255)),
    ((0,0,255),(255,255,255),(0,0,255),(0,0,255)),
    ((0,0,255),(255,255,255),(0,0,255),(0,0,255)),
));

pub const FLAT_SMOOTH_BLUE : Theme = Theme((
    ((104,187,227), (255,255,255),(104,187,227),(104,187,227)),
    ((19,139,217),(255,255,255),(19,139,217), (19,139,217)),
    ((19,139,217),(255,255,255),(19,139,217), (19,139,217)),
));

pub const FLAT_RED_EXIT : Theme = Theme((
    ((255,255,255),(100,100,100),(255,255,255),(255,255,255)),
    ((255,0,0),(255,255,255),(255,0,0),(255,0,0)),
    ((255,0,0),(255,255,255),(255,0,0),(255,0,0)),
));
