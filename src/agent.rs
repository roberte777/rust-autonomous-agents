use crate::geometry::Position;
use rand::Rng;

pub struct Agent {
    pub pos: Position,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
}

impl Agent {
    pub fn new(size: (u32, u32)) -> Agent {
        let max_x = ((size.0 as f32 / 2.0).floor() - 10.0) as i32;
        let max_y = ((size.1 as f32 / 2.0).floor() - 10.0) as i32;
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-max_x..max_x);
        let y = rng.gen_range(-max_y..max_y);
        return Agent {
            pos: Position { x, y },
            width: 20.0,
            height: 20.0,
            rotation: 0.0,
        };
    }
}
