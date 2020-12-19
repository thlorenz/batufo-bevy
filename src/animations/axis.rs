use std::fmt::Display;

#[derive(Debug)]
pub enum Direction {
    Positive, // same as counter clockwise for rotation
    Negative, // same as clockwise for rotation
}

#[derive(Debug)]
pub enum MovementAxis {
    X(Direction),
    Z(Direction),
}

impl Display for MovementAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (sign, axis) = match self {
            MovementAxis::X(Direction::Positive) => ("+", "x"),
            MovementAxis::X(Direction::Negative) => ("-", "x"),
            MovementAxis::Z(Direction::Positive) => ("+", "z"),
            MovementAxis::Z(Direction::Negative) => ("-", "z"),
        };
        write!(f, "{}{}", sign, axis)
    }
}

impl MovementAxis {
    pub fn from_move_xz<T>((start_x, start_z): (T, T), (end_x, end_z): (T, T)) -> MovementAxis
    where
        T: PartialOrd,
    {
        if start_x < end_x {
            MovementAxis::X(Direction::Positive)
        } else if start_x > end_x {
            MovementAxis::X(Direction::Negative)
        } else if start_z < end_z {
            MovementAxis::Z(Direction::Positive)
        } else if start_z > end_z {
            MovementAxis::Z(Direction::Negative)
        } else {
            panic!("Either xs or ys need to differ to consitute a move")
        }
    }
}
pub enum RotationAxis {
    X(Direction),
    Z(Direction),
}

impl RotationAxis {
    pub fn from_movement_axis(move_axis: &MovementAxis) -> RotationAxis {
        match move_axis {
            MovementAxis::X(Direction::Positive) => RotationAxis::Z(Direction::Negative),
            MovementAxis::X(Direction::Negative) => RotationAxis::Z(Direction::Positive),
            MovementAxis::Z(Direction::Positive) => RotationAxis::X(Direction::Negative),
            MovementAxis::Z(Direction::Negative) => RotationAxis::X(Direction::Positive),
        }
    }
}
