use std::collections::HashSet;
use Vec;
use crate::rover;

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

    pub fn drop_rover(&mut self, coordinates: rover::Coordinates) {
        self.rovers.insert(coordinates);
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
}