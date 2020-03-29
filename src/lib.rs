pub struct Rover {
    coordinates: Coordinates,
}

#[derive(Debug, PartialEq)]
struct Coordinates {
    x_coordinate: i64,
    y_coordinate: i64,
}

impl Coordinates {
    fn north(&mut self) {
        self.y_coordinate += 1;
    }

    fn south(&mut self) {
        self.y_coordinate -= 1;
    }

    fn east(&mut self) {
        self.x_coordinate += 1;
    }

    fn west(&mut self) {
        self.x_coordinate -= 1;
    }
}

impl Rover {
    fn new(x_coordinate: i64, y_coordinate:i64) -> Rover {
        Rover{coordinates: Coordinates{x_coordinate, y_coordinate}}
    }

    fn get_coordinates(&self) -> &Coordinates {
        &self.coordinates
    }

    fn move_north(&mut self) {
        self.coordinates.north();
    }

    fn move_south(&mut self) {
        self.coordinates.south();
    }

    fn move_east(&mut self) {
        self.coordinates.east();
    }

    fn move_west(&mut self) {
        self.coordinates.west();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_north() {
        let mut rover = Rover::new(0, 0);

        rover.move_north();

        assert_eq!(
            Coordinates{ x_coordinate: 0, y_coordinate: 1 },
            *rover.get_coordinates()
        );
    }

    #[test]
    fn move_south() {
        let mut rover = Rover::new(0, 1);

        rover.move_south();

        assert_eq!(
            Coordinates{ x_coordinate: 0, y_coordinate: 0 },
            *rover.get_coordinates()
        );
    }

    #[test]
    fn move_east() {
        let mut rover = Rover::new(0, 0);

        rover.move_east();

        assert_eq!(
            Coordinates{ x_coordinate: 1, y_coordinate: 0 },
            *rover.get_coordinates()
        );
    }

    #[test]
    fn move_west() {
        let mut rover = Rover::new(1, 0);

        rover.move_west();

        assert_eq!(
            Coordinates{ x_coordinate: 0, y_coordinate: 0 },
            *rover.get_coordinates()
        );
    }
}