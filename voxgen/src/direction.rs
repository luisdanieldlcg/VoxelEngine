use vek::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Back,
    Front,
}

impl Direction {
    pub fn normalized(self) -> Vec3<i32> {
        match self {
            Direction::Up => Vec3::unit_y(),
            Direction::Down => -Vec3::unit_y(),
            Direction::Left => -Vec3::unit_x(),
            Direction::Right => Vec3::unit_x(),
            Direction::Front => Vec3::unit_z(),
            Direction::Back => -Vec3::unit_z(),
        }
    }
}
