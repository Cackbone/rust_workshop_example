use std::fmt;

use crate::Cell;
use crate::utils::rand::random_bool;

#[derive(Debug, Clone)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize
}


impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line = format!(
            "■{}■\n", "▬".repeat(self.width)
        );
        let cells_str: String = self.cells.iter().map(|line| {
            let line_cells: String = line.iter().map(|cell| {
                cell.to_string()
            }).collect();
            format!("|{}|\n", line_cells)

        }).collect();

        write!(f, "{}{}{}", line, cells_str, line)
    }
}


impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let cells: Vec<Vec<Cell>> = (0..height).map(|y| {
            (0..width)
                .map(|x| Cell::new(x, y, true))
                .collect()
        }).collect();

        Grid {
            cells,
            width,
            height
        }
    }


    pub fn new_rand(width: usize, height: usize, p: f64) -> Grid {
        let mut grid = Grid::new(width, height);

        for line in grid.cells.iter_mut() {
            for cell in line.iter_mut() {
                cell.is_alive = random_bool(p);
            }
        }

        grid
    }

    pub fn get_next_gen(&self) -> Grid {
        let mut grid = Grid::new(self.width, self.height);

        for line in grid.cells.iter_mut() {
            for cell in line.iter_mut() {
                let neighbors = self.count_neighbors(cell);
                cell.update(neighbors);
            }
        }

        grid
    }

    fn count_neighbors(&self, cell: &Cell) -> usize {
        let pos_x = cell.pos.x as i32;
        let pos_y = cell.pos.y as i32;
        let width = self.width as i32;
        let height = self.height as i32;
        let mut counter = 0;

        for x in (pos_x - 1)..(pos_x + 2) {
            for y in (pos_y - 1)..(pos_y + 2) {
                if x >= 0 && x < width
                    && y >= 0 && y < height
                    && self.cells[y as usize][x as usize].is_alive {
                        counter += 1;
                    }
            }
        }

        counter
    }

}
