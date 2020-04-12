use crate::Command;
use core::fmt;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Rover {
    bearing: Bearing,
    coordinates: Coordinates,
}

impl Rover {
    pub fn new(x_coordinate: u64, y_coordinate: u64, bearing: Bearing) -> Rover {
        Rover {
            bearing,
            coordinates: Coordinates {
                x_coordinate,
                y_coordinate,
            },
        }
    }

    pub fn planned_move(&self) -> Coordinates {
        let mut planned_coordinates = self.coordinates;
        planned_coordinates.move_forward(self.bearing);
        planned_coordinates
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }

    pub fn execute_command(&mut self, turn: &Command) {
        match (self.bearing, turn) {
            (Bearing::North, Command::RightTurn) => self.bearing = Bearing::East,
            (Bearing::North, Command::LeftTurn) => self.bearing = Bearing::West,
            (Bearing::East, Command::RightTurn) => self.bearing = Bearing::South,
            (Bearing::East, Command::LeftTurn) => self.bearing = Bearing::North,
            (Bearing::South, Command::RightTurn) => self.bearing = Bearing::West,
            (Bearing::South, Command::LeftTurn) => self.bearing = Bearing::East,
            (Bearing::West, Command::RightTurn) => self.bearing = Bearing::North,
            (Bearing::West, Command::LeftTurn) => self.bearing = Bearing::South,
            (_, Command::MoveForward) => self.coordinates.move_forward(self.bearing),
        };
    }
}

impl fmt::Display for Rover {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.coordinates.x_coordinate, self.coordinates.y_coordinate, self.bearing
        )
    }
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Bearing {
    North,
    East,
    South,
    West,
}

impl FromStr for Bearing {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Bearing::North),
            "E" => Ok(Bearing::East),
            "S" => Ok(Bearing::South),
            "W" => Ok(Bearing::South),
            _ => Err(String::from("could not parse bearing")),
        }
    }
}

impl fmt::Display for Bearing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = match self {
            Bearing::North => "N",
            Bearing::East => "E",
            Bearing::South => "S",
            Bearing::West => "W",
        };
        write!(f, "{}", b)
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Coordinates {
    pub x_coordinate: u64,
    pub y_coordinate: u64,
}

impl Coordinates {
    pub fn new(x_coordinate: u64, y_coordinate: u64) -> Coordinates {
        Coordinates {
            x_coordinate,
            y_coordinate,
        }
    }

    pub fn move_forward(&mut self, bearing: Bearing) {
        match bearing {
            Bearing::North => self.y_coordinate += 1,
            Bearing::South => self.y_coordinate -= 1,
            Bearing::East => self.x_coordinate += 1,
            Bearing::West => self.x_coordinate -= 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_north() {
        let mut rover = Rover::new(0, 0, Bearing::North);

        rover.execute_command(&Command::MoveForward);

        assert_eq!(Rover::new(0, 1, Bearing::North), rover);
    }

    #[test]
    fn move_south() {
        let mut rover = Rover::new(0, 1, Bearing::South);

        rover.execute_command(&Command::MoveForward);

        assert_eq!(Rover::new(0, 0, Bearing::South), rover);
    }

    #[test]
    fn move_east() {
        let mut rover = Rover::new(0, 0, Bearing::East);

        rover.execute_command(&Command::MoveForward);

        assert_eq!(Rover::new(1, 0, Bearing::East), rover);
    }

    #[test]
    fn move_west() {
        let mut rover = Rover::new(1, 0, Bearing::West);

        rover.execute_command(&Command::MoveForward);

        assert_eq!(Rover::new(0, 0, Bearing::West), rover);
    }

    #[test]
    fn turn_right_from_north() {
        let mut rover = Rover::new(0, 0, Bearing::North);

        rover.execute_command(&Command::RightTurn);

        assert_eq!(Rover::new(0, 0, Bearing::East), rover);
    }

    #[test]
    fn turn_left_from_north() {
        let mut rover = Rover::new(0, 0, Bearing::North);

        rover.execute_command(&Command::LeftTurn);

        assert_eq!(Rover::new(0, 0, Bearing::West), rover);
    }

    #[test]
    fn get_planned_move() {
        let rover = Rover::new(0, 0, Bearing::North);

        let planned_coordinates = rover.planned_move();

        assert_eq!(Coordinates::new(0, 1), planned_coordinates);

        assert_eq!(Rover::new(0, 0, Bearing::North), rover);
    }
}
