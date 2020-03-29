use std::collections::HashSet;
use Vec;
use crate::rover;
use std::error;
use std::error::Error;
use core::fmt;

pub struct Plateau {
    max_x_coordinate: u64,
    max_y_coordinate: u64,
    rovers: HashSet<rover::Coordinates>
}

impl Plateau {
    pub fn new(max_x_coordinate: u64, max_y_coordinate: u64) -> Plateau {
        Plateau{max_x_coordinate, max_y_coordinate, rovers: HashSet::new()}
    }

    pub fn list_rovers(&self) -> Vec<rover::Coordinates> {
        self.rovers.iter().cloned().collect()
    }

    pub fn drop_rover(&mut self, coordinates: rover::Coordinates) -> Result<(), DropError> {
        if self.rovers.insert(coordinates) {
            Ok(())
        } else {
            Err(DropError)
        }
    }

    fn can_rover_move(&self, coordinates: &rover::Coordinates) -> bool {
        !self.rovers.contains(coordinates)
    }

    pub fn move_rover(&mut self, old_coordinates: &rover::Coordinates,
                      new_coordinates: &rover::Coordinates) -> Result<(), Box<dyn Error>> {
        if !self.can_rover_move(new_coordinates) {
            Err(Box::new(CollisionError))
        } else if !self.rovers.remove(old_coordinates) {
            Err(Box::new(NotFound))
        } else {
            self.rovers.insert(new_coordinates.clone());
            Ok(())
        }

    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct DropError;

impl error::Error for DropError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for DropError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rover could not be dropped")
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct CollisionError;

impl error::Error for CollisionError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for CollisionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rover caused collision")
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct NotFound;

impl error::Error for NotFound {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rover was not found on plateau")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_rover() {
        let mut plateau = Plateau::new(5, 5);

        let coordinates = rover::Coordinates::new(0, 0);

        plateau.drop_rover(coordinates);

        assert_eq!(
            vec![coordinates],
            plateau.list_rovers()
        );
    }

    #[test]
    fn drop_rover_on_rover() {
        let mut plateau = Plateau::new(5, 5);

        let coordinates = rover::Coordinates::new(0, 0);

        if let Err(e) = plateau.drop_rover(coordinates) {
            panic!("should not return error")
        };

        if let Err(e) = plateau.drop_rover(coordinates) {
            assert_eq!(DropError, e)
        } else {
            panic!("should have returned error")
        }

        assert_eq!(
            vec![coordinates],
            plateau.list_rovers()
        );
    }

    #[test]
    fn rover_can_move() {
        let mut plateau = Plateau::new(5, 5);

        let coordinates = rover::Coordinates::new(1, 1);

        let can_move = plateau.can_rover_move(&coordinates);

        assert_eq!(
            true,
            can_move
        );
    }

    fn rover_cannot_move() {
        let mut plateau = Plateau::new(5, 5);

        let coordinates = rover::Coordinates::new(1, 1);

        if let Err(e) = plateau.drop_rover(coordinates) {
            panic!("should be able to drop rover")
        };

        let can_move = plateau.can_rover_move(&coordinates);

        assert_eq!(
            false,
            can_move
        );
    }
}