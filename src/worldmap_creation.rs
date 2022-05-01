use crate::prelude::*;
use std::collections::HashMap;


#[derive(PartialEq, Eq, Hash)]
pub struct Position(pub i32, pub i32);
impl Position {
    pub fn get(&self) -> (f32,f32) {
        (self.0 as f32, self.1 as f32)
    }
}
impl Into<Vec2> for Position {
    fn into(self) -> Vec2 {
        Vec2::new(self.0 as f32, self.1 as f32)
    }
}
impl From<Vec2> for Position {
    fn from(v2 : Vec2) -> Self {
        Position(v2.x as i32, v2.y as i32)
    }
}
// creation of the world map to put it the resources ECS
pub struct WorldMap {
    pub cities : HashMap<Position,City>,
    pub locked : bool
}

impl WorldMap {
    pub fn new() -> Self {
        let mut w = WorldMap {
            cities : HashMap::with_capacity(14),
            locked : false,
        };
        w.initialize();
        w
    }

    pub fn initialize(&mut self) {
        let c = [
            ("Ottawa", (309,292),898, 0.7 ),
            ("Wachington DC", (304,325), 601, 0.7),
            ("Mexico", (234,402), 8851, 0.5),
            ("Brasilia", (402,513), 2648, 0.4),
            ("Montevideo", (366,586), 2648, 0.4),
            ("London", (561,263), 8174, 0.8),
            ("Paris", (569,279), 2268, 0.7),
            ("Alger", (570,334), 3574, 0.3),
            ("Cairo", (667,359), 7438, 0.3),
            ("Oslo", (596,214), 575, 0.4),
            ("Moscow", (683,242), 11541, 0.8),
            ("Beijing", (948,321), 20180, 0.8),
            ("Tokyo", (1026,336), 13185, 0.6),
            ("Canberra", (1058,587), 354, 0.5),
        ];
        // using slice[] for future dataset initialization
        for ct in c {
            self.cities.insert(Position(ct.1.0, ct.1.1), City::new(ct.0, ct.2, ct.3));
        }
    }
    pub fn over_city(&self, mouse_pos : &Vec2) -> Vec2 {
        for (pos,_) in &self.cities {
            let over = self.into_scope(&Vec2::new(pos.0 as f32, pos.1 as f32), mouse_pos);
            if over == true {
                // println!("mouse at {:?} over {}", pos.get(), city.name);
                let (p0, p1) = pos.get();
                return Vec2::new(p0, p1);
            }
        }
        Vec2::ZERO
    }
    fn into_scope(&self, city_pos : &Vec2, mouse_pos : &Vec2) -> bool {
        // let d = city_pos.distance(*mouse_pos);
        // println!("p0 : {}, p1 : {}, d : {}", city_pos, mouse_pos, d);
        city_pos.distance(*mouse_pos) <= CITY_FOCUS_RADIUS as f32
    }
    pub fn select_city(&mut self, position : &Vec2) {
        let pos = Position::from(*position);
        // println!("try to select city at : {}", self.cities.get(&pos).unwrap().name);
        if self.cities.contains_key(&pos) {
            if self.locked == false {
                self.cities.get_mut(&pos).unwrap().select();
                self.locked = true;
                println!("city selected : {:?}", self.cities.get(&pos).unwrap().name);
            } else {
                self.unselect_city(position);
            }
        }
    }
    fn unselect_city(&mut self, position : &Vec2) {
        let pos = Position::from(*position);
        let city = self.cities.get(&pos).unwrap();
        if city.is_selected() == true {
            self.cities.get_mut(&pos).unwrap().unselect();
            self.locked = false;
            println!("city {} has been unselected", self.cities.get(&pos).unwrap().name);
        }
    }
    pub fn datas_from_selected_city(&self) -> Option<(&String, u32, u8, f32, f32)> {
        for (_, city) in &self.cities {
            if city.is_selected() == true {
                return Some(city.get_datas());
            }
        }
        None
    }
}
pub struct City {
    name : String,
    population : u32,
    district_size : u8, // 3, 5 or 7
    security_rate : f32,
    zombi_rate : f32,
    selected : bool,
}

impl City {
    fn new(name : &str, population : u32, security_rate : f32) -> Self {
        // size city districk from population
        let district_size = match population {
            0..=999 => 3,
            1000..=9999 => 5,
            _ => 7
        };
        City {
            name: String::from(name),
            population,
            district_size,
            security_rate,
            zombi_rate: 0f32,
            selected: false,
        }
    }
    pub fn get_datas(&self) -> (&String, u32, u8, f32, f32) {
        (&self.name, self.population, self.district_size, self.security_rate, self.zombi_rate)
    }
    fn select(&mut self) {
        self.selected = true;
    }
    fn unselect(&mut self) {
        self.selected = false;
    }
    pub fn is_selected(&self) -> bool {
        self.selected
    }
}