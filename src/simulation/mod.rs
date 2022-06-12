use crate::physics::Position;
use rand::Rng;
pub fn generate_random_pos(size: (u32, u32)) -> Position {
    let max_x = (size.0 as f32 / 2.0).floor();
    let max_y = (size.1 as f32 / 2.0).floor();
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-max_x..max_x);
    let y = rng.gen_range(-max_y..max_y);
    return Position { x, y };
}
