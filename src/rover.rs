#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Rover {
    bearing: Bearing,
    coordinates: Coordinates,
}

impl Rover {
    pub fn new(x_coordinate: u64, y_coordinate:u64, bearing: Bearing) -> Rover {
        Rover{bearing, coordinates: Coordinates{x_coordinate, y_coordinate}}
    }

    pub fn move_rover(&mut self) {
        self.coordinates.move_forward(self.bearing);
    }

    pub fn get_planned_move(&self) -> Coordinates {
        let mut planned_coordinates = self.coordinates;
        planned_coordinates.move_forward(self.bearing);
        planned_coordinates
    }

    pub fn get_coordinates(&self) -> &Coordinates {
        &self.coordinates
    }

    pub fn turn_right(&mut self) {
        match self.bearing {
            Bearing::North => self.bearing = Bearing::East,
            Bearing::East => self.bearing = Bearing::South,
            Bearing::South => self.bearing = Bearing::West,
            Bearing::West => self.bearing = Bearing::North,
        }
    }

    pub fn turn_left(&mut self) {
        match self.bearing {
            Bearing::North => self.bearing = Bearing::West,
            Bearing::East => self.bearing = Bearing::North,
            Bearing::South => self.bearing = Bearing::East,
            Bearing::West => self.bearing = Bearing::South,
        }
    }

    pub fn print(&self) -> String {
        let b = match self.bearing {
            Bearing::North => "N",
            Bearing::East => "E",
            Bearing::South => "S",
            Bearing::West => "W"
        };
        format!("{} {} {}", self.coordinates.x_coordinate, self.coordinates.y_coordinate, b)
    }
}

#[derive(Hash, Eq, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Bearing {
    North,
    East,
    South,
    West,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Coordinates {
    pub x_coordinate: u64,
    pub y_coordinate: u64,
}

impl Coordinates {
    pub fn new(x_coordinate: u64, y_coordinate: u64) -> Coordinates {
        Coordinates{x_coordinate, y_coordinate}
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

        rover.move_rover();

        assert_eq!(
            Rover::new(0, 1, Bearing::North),
            rover
        );
    }

    #[test]
    fn move_south() {
        let mut rover = Rover::new(0, 1, Bearing::South);

        rover.move_rover();

        assert_eq!(
            Rover::new(0, 0, Bearing::South),
            rover
        );
    }

    #[test]
    fn move_east() {
        let mut rover = Rover::new(0, 0, Bearing::East);

        rover.move_rover();

        assert_eq!(
            Rover::new(1, 0, Bearing::East),
            rover
        );
    }

    #[test]
    fn move_west() {
        let mut rover = Rover::new(1, 0, Bearing::West);

        rover.move_rover();

        assert_eq!(
            Rover::new(0, 0, Bearing::West),
            rover
        );
    }

    #[test]
    fn turn_right_from_north() {
        let mut rover = Rover::new(0, 0, Bearing::North);

        rover.turn_right();

        assert_eq!(
            Rover::new(0, 0, Bearing::East),
            rover
        );
    }

    #[test]
    fn turn_left_from_north() {
        let mut rover = Rover::new(0, 0, Bearing::North);

        rover.turn_left();

        assert_eq!(
            Rover::new(0, 0, Bearing::West),
            rover
        );
    }

    #[test]
    fn get_planned_move() {
        let mut rover = Rover::new(0, 0, Bearing::North);

        let planned_coordinates = rover.get_planned_move();

        assert_eq!(
            Coordinates::new(0, 1),
            planned_coordinates
        );

        assert_eq!(
            Rover::new(0, 0, Bearing::North),
            rover
        );
    }
}