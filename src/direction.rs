use vek::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT,
    BACK,
    FRONT,
}

impl Direction {
    pub fn to_vec(self) -> Vec3<i32> {
        match self {
            Direction::TOP => Vec3::new(0, 1, 0),
            Direction::BOTTOM => Vec3::new(0, -1, 0),
            Direction::LEFT => Vec3::new(-1, 0, 0),
            Direction::RIGHT => Vec3::new(1, 0, 0),
            Direction::FRONT => Vec3::new(0, 0, 1),
            Direction::BACK => Vec3::new(0, 0, -1),
        }
    }
}
