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
    pub fn normalized(self) -> Vec3<i32> {
        match self {
            Direction::TOP => Vec3::unit_y(),
            Direction::BOTTOM => -Vec3::unit_y(),
            Direction::LEFT => -Vec3::unit_x(),
            Direction::RIGHT => Vec3::unit_x(),
            Direction::FRONT => Vec3::unit_z(),
            Direction::BACK => -Vec3::unit_z(),
        }
    }
}
