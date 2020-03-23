use std::io;
use std::error::Error;

use game_of_life::Grid;

fn main() -> Result<(), Box<dyn Error>>{
    let mut grid = Grid::new_rand(100, 100, 0.01);
    let mut gen = 0;
    let mut i = String::new();

    loop {
        print!("\x1B[2J{}", grid);
        grid = grid.get_next_gen();
        println!("[{}] press enter to jump to the next generation", gen);
        io::stdin().read_line(&mut i)?;
        gen += 1
    }
}
