use crate::prelude::*;

#[system]
#[read_component(Render)]
#[read_component(Vec2)]
pub fn wm_render(ecs: &SubWorld, #[resource] map: &WorldMap) {
    <(&Vec2, &Render)>::query()
        // .filter(component::<CityPointer>())
        .iter(ecs)
        .filter(|(_, render)| render.to_draw == true)
        .for_each(|(pos, render)| {
            draw_texture_ex(
                render.texture,
                pos.x,
                pos.y,
                render.color,
                DrawTextureParams {
                    // dest_size : Some(Vec2::new(scalew, scaleh)),
                    ..Default::default()
                },
            );
        });
    let positions = map.get_selected_pos();
    if let Some(p) = positions {
        for pos in &p {
            let data = map.datas_from_selected_city(pos);
            render_popup(pos.x, pos.y, data);
        }
    };
}

fn render_popup(x: f32, y: f32, datas: (&String, u32, u8, f32, f32)) {
    // println!("city name : {}", datas.0);
    let x_pop;
    let y_pop;
    if x < (WINDOW_W - 400) as f32 {
        x_pop = x - 15.0;
        y_pop = y - 140.0;
    } else {
        x_pop = x - 250.0 + 15.0;
        y_pop = y - 140.0;
    }
    draw_rectangle(x_pop, y_pop, 250., 110., WHITE);
    draw_triangle(
        Vec2::new(x - 6.0, y - 30.0),
        Vec2::new(x + 6.0, y - 30.0),
        Vec2::new(x, y - 5.0),
        WHITE,
    );
    let fs = FONT_SIZE_POPUP as f32;
    draw_text(
        &format!("City : {}", datas.0),
        x_pop + 5.0,
        y_pop + fs,
        fs,
        BLACK,
    );
    draw_text(
        &format!("Population : {} millions", datas.1 as f32 / 1000.),
        x_pop + 5.0,
        y_pop + 2.0 * fs,
        fs,
        BLACK,
    );
    draw_text(
        &format!("Number of districts : {}", datas.2),
        x_pop + 5.0,
        y_pop + 3.0 * fs,
        fs,
        BLACK,
    );
    draw_text(
        &format!("Secutity rate : {:.2} %", datas.3 * 100.),
        x_pop + 5.0,
        y_pop + 4.0 * fs,
        fs,
        BLACK,
    );
    draw_text(
        &format!("Zombi rate : {:.2} %", datas.4 * 100.),
        x_pop + 5.0,
        y_pop + 5.0 * fs,
        fs,
        BLACK,
    );
}
