use core::ops::{Add, AddAssign, Neg, Sub};

pub mod axis_order;
pub mod measurement;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        *self = *self + rhs;
    }
}

impl Sub<Position> for Position {
    type Output = PositionDelta;

    fn sub(self, rhs: Position) -> Self::Output {
        PositionDelta {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<PositionDelta> for Position {
    type Output = Position;

    fn sub(self, rhs: PositionDelta) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PositionDelta {
    pub x: i32,
    pub y: i32,
}

impl Add<PositionDelta> for Position {
    type Output = Position;

    fn add(self, rhs: PositionDelta) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<PositionDelta> for Position {
    fn add_assign(&mut self, rhs: PositionDelta) {
        *self = *self + rhs;
    }
}

impl Neg for PositionDelta {
    type Output = PositionDelta;

    fn neg(self) -> Self::Output {
        PositionDelta {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeasuredSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
    pub position: Position,
    pub size: MeasuredSize,
}

impl BoundingBox {
    pub fn contains(&self, position: Position) -> bool {
        position.x >= self.position.x
            && position.y >= self.position.y
            && position.x <= self.position.x + self.size.width as i32
            && position.y <= self.position.y + self.size.height as i32
    }

    pub fn translate(self, by: PositionDelta) -> BoundingBox {
        BoundingBox {
            position: self.position + by,
            size: self.size,
        }
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            position: Position { x: 0, y: 0 },
            size: MeasuredSize {
                width: 0,
                height: 0,
            },
        }
    }
}
