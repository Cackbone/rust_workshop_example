use std::fmt;

use crate::utils::vector2::Vector2u;

#[derive(Debug, Clone)]
pub struct Cell {
    pub is_alive: bool,
    pub pos: Vector2u
}


impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_alive {
            write!(f, "●")
        } else {
            write!(f, "◌")
        }
    }
}


impl Cell {
    pub fn new(x: usize, y: usize, is_alive: bool) -> Cell {
        Cell {
            is_alive,
            pos: Vector2u::new(x, y)
        }
    }


    pub fn update(&mut self, neighbors: usize) {
        self.is_alive = if self.is_alive {
            !(neighbors < 2 || neighbors > 3)
        } else {
            neighbors == 3
        };
    }
}
