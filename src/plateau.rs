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

    fn is_move_inbounds(&self, coordinates: &rover::Coordinates) -> bool {
        if coordinates.x_coordinate > self.max_x_coordinate || coordinates.y_coordinate > self.max_y_coordinate {
            return false
        }
        true

    }

    fn is_move_valid(&self, coordinates: &rover::Coordinates) -> Result<(), Box<dyn Error>> {
        self.can_rover_move(coordinates)?;
        if !self.is_move_inbounds(coordinates) {
            Err(Box::new(OutOfBounds))
        } else {
            Ok(())
        }
    }

    pub fn can_rover_move(&self, coordinates: &rover::Coordinates) -> Result<(), CollisionError> {
        if !self.rovers.contains(coordinates) {
            return Ok(())
        }
        Err(CollisionError)
    }

    pub fn update_rover_position(&mut self, old_coordinates: &rover::Coordinates,
                                 new_coordinates: &rover::Coordinates) -> Result<(), Box<dyn Error>> {
        self.is_move_valid(new_coordinates)?;
        if !self.rovers.remove(old_coordinates) {
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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct OutOfBounds;

impl error::Error for OutOfBounds {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for OutOfBounds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "move is out of bounds")
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
            Ok(()),
            can_move
        );
    }

    #[test]
    fn rover_cannot_move() {
        let mut plateau = Plateau::new(5, 5);

        let coordinates = rover::Coordinates::new(1, 1);

        if let Err(e) = plateau.drop_rover(coordinates) {
            panic!("should be able to drop rover")
        };

        let can_move = plateau.can_rover_move(&coordinates);

        assert!(
            can_move.is_err()
        );
    }

    #[test]
    fn move_rover() {
        let mut plateau = Plateau::new(5, 5);

        let old_coordinates = rover::Coordinates::new(1, 1);
        let new_coordinates = rover::Coordinates::new(0, 1);

        if let Err(e) = plateau.drop_rover(old_coordinates) {
            panic!("should be able to drop rover")
        };

        if let Err(e) = plateau.update_rover_position(&old_coordinates, &new_coordinates) {
            panic!("should have been able to move rover")
        }

        assert_eq!(
            vec![new_coordinates],
            plateau.list_rovers()
        );
    }

    fn move_rover_cause_collsion() {
        let mut plateau = Plateau::new(5, 5);

        let another_rover_coordinates = rover::Coordinates::new(0, 1);

        let old_coordinates = rover::Coordinates::new(1, 1);
        let new_coordinates = rover::Coordinates::new(0, 1);

        if let Err(e) = plateau.drop_rover(another_rover_coordinates) {
            panic!("should be able to drop rover")
        };

        if let Err(e) = plateau.drop_rover(old_coordinates) {
            panic!("should be able to drop rover")
        };

        if let Err(e) = plateau.update_rover_position(&old_coordinates, &new_coordinates) {
            if let None = e.source() {
                panic!("Should have returned collision error")
            }
        }

    }

    fn move_rover_not_found() {
        let mut plateau = Plateau::new(5, 5);

        let old_coordinates = rover::Coordinates::new(1, 1);
        let new_coordinates = rover::Coordinates::new(0, 1);

        if let Err(e) = plateau.update_rover_position(&old_coordinates, &new_coordinates) {
            if let None = e.source() {
                panic!("Should have returned not found error")
            }
        }

    }

    fn move_rover_out_of_bounds_up() {
        let mut plateau = Plateau::new(5, 5);

        let old_coordinates = rover::Coordinates::new(5, 5);
        let new_coordinates = rover::Coordinates::new(5, 6);

        if let Err(e) = plateau.drop_rover(old_coordinates) {
            panic!("should be able to drop rover")
        };

        if let Err(e) = plateau.update_rover_position(&old_coordinates, &new_coordinates) {
            if let None = e.source() {
                panic!("Should have returned out of bounds")
            }
        }
    }

    fn move_rover_out_of_bounds_right() {
        let mut plateau = Plateau::new(5, 5);

        let old_coordinates = rover::Coordinates::new(5, 5);
        let new_coordinates = rover::Coordinates::new(6, 5);

        if let Err(e) = plateau.drop_rover(old_coordinates) {
            panic!("should be able to drop rover")
        };

        if let Err(e) = plateau.update_rover_position(&old_coordinates, &new_coordinates) {
            if let None = e.source() {
                panic!("Should have returned out of bounds")
            }
        }
    }
}