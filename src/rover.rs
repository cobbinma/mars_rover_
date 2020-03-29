#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Rover {
    bearing: Bearing,
    coordinates: Coordinates,
}

impl Rover {
    pub fn new(x_coordinate: i64, y_coordinate:i64, bearing: Bearing) -> Rover {
        Rover{bearing, coordinates: Coordinates{x_coordinate, y_coordinate}}
    }

    pub fn get_coordinates(&self) -> Coordinates {
        self.coordinates
    }

    pub fn move_north(&mut self) {
        self.coordinates.north();
    }

    pub fn move_south(&mut self) {
        self.coordinates.south();
    }

    pub fn move_east(&mut self) {
        self.coordinates.east();
    }

    pub fn move_west(&mut self) {
        self.coordinates.west();
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
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Bearing {
    North,
    East,
    South,
    West,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Coordinates {
    pub x_coordinate: i64,
    pub y_coordinate: i64,
}

impl Coordinates {
    pub fn new(x_coordinate: i64, y_coordinate: i64) -> Coordinates {
        Coordinates{x_coordinate, y_coordinate}
    }

    pub fn north(&mut self) {
        self.y_coordinate += 1;
    }

    pub fn south(&mut self) {
        self.y_coordinate -= 1;
    }

    pub fn east(&mut self) {
        self.x_coordinate += 1;
    }

    pub fn west(&mut self) {
        self.x_coordinate -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_north() {
        let mut rover = Rover::new(0, 0, Bearing::North);

        rover.move_north();

        assert_eq!(
            Rover::new(0, 1, Bearing::North),
            rover
        );
    }

    #[test]
    fn move_south() {
        let mut rover = Rover::new(0, 1, Bearing::North);

        rover.move_south();

        assert_eq!(
            Rover::new(0, 0, Bearing::North),
            rover
        );
    }

    #[test]
    fn move_east() {
        let mut rover = Rover::new(0, 0, Bearing::North);

        rover.move_east();

        assert_eq!(
            Rover::new(1, 0, Bearing::North),
            rover
        );
    }

    #[test]
    fn move_west() {
        let mut rover = Rover::new(1, 0, Bearing::North);

        rover.move_west();

        assert_eq!(
            Rover::new(0, 0, Bearing::North),
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
}