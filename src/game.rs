use core::fmt;
use rand::Rng;
use std::ops::Index;

mod world_parts;

pub struct Row<'a> {
    cells: &'a [Cell],
}

impl<'a> Row<'a> {
    pub fn new(cells: &'a [Cell]) -> Row<'a> {
        Self { cells }
    }
}

impl<'a> fmt::Display for Row<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in self.cells {
            let symbol = if cell.is_alive() { '#' } else { ' ' };
            write!(f, "{}", symbol)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    fn is_alive(&self) -> bool {
        match self {
            Cell::Alive => true,
            Cell::Dead => false,
        }
    }

    fn set_state(&self, n: u8) -> Self {
        match (self, n) {
            (Self::Alive, 3) => Self::Alive,
            (Self::Alive, 2) => Self::Alive,
            (Self::Dead, 3) => Self::Alive,
            (_, _) => Self::Dead,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    grid: Vec<Cell>,
    pub width: u32,
    pub height: u32,
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        // The new function will create a new grid and set the initial state of
        // all cells to dead. After which it will use an iterator to set random
        // cells to Alive.
        let mut grid = vec![Cell::Dead; (width * height) as usize];
        let mut rng = rand::thread_rng();

        for cell in grid.iter_mut() {
            if rng.gen::<bool>() {
                *cell = Cell::Alive;
            }
        }

        Self {
            grid,
            width,
            height,
        }
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn get_row(&self, row: u32) -> &[Cell] {
        let start = (row * self.width) as usize;
        let end = ((row + 1) * self.width) as usize;
        &self.grid[start..end]
    }

    pub fn evolve(&mut self) {
        // The function creates a clone of the old grid and then sets the state of each
        // new cell based on the circumstances of the old grid. Finally it sets the grid
        // field of World to the new grid.
        let mut new_grid = self.grid.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let num_neighbours = self.get_num_alive_neighbours(row, col);
                new_grid[idx] = self.grid[idx].set_state(num_neighbours);
            }
        }

        self.grid = new_grid;
    }

    fn get_num_alive_neighbours(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1] {
            for delta_col in [self.width - 1, 0, 1] {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let n_row = (delta_row + row) % self.height;
                let n_col = (delta_col + col) % self.width;
                let idx = self.get_index(n_row, n_col);
                if self.grid[idx] == Cell::Alive {
                    count += 1
                }
            }
        }

        count
    }
}

impl Index<usize> for World {
    type Output = Cell;

    fn index(&self, i: usize) -> &Cell {
        &self.grid[i]
    }
}

#[cfg(test)]
mod test {
    use crate::game::Cell::*;
    use crate::game::World;

    #[test]
    fn test_world_get_row() {
        let world = World {
            grid: vec![Dead, Dead, Dead, Alive, Alive, Alive],
            width: 3,
            height: 2,
        };

        let row = world.get_row(0);
        assert_eq!(vec![Dead, Dead, Dead], row);

        let row = world.get_row(1);
        assert_eq!(vec![Alive, Alive, Alive], row);
    }

    #[test]
    fn test_cell_is_alive() {
        let a = Alive;
        assert_eq!(a.is_alive(), true);

        let b = Dead;
        assert_eq!(b.is_alive(), false);
    }

    #[test]
    fn test_get_num_alive_neighbours() {
        let world = World {
            grid: vec![Dead, Dead, Alive, Alive, Dead, Dead, Alive, Dead, Dead],
            width: 3,
            height: 3,
        };

        let result = world.get_num_alive_neighbours(1, 1);
        assert_eq!(result, 3);

        let world = World {
            grid: vec![Dead, Alive, Dead, Dead, Alive, Dead, Dead, Alive, Dead],
            width: 3,
            height: 3,
        };

        let result = world.get_num_alive_neighbours(1, 1);
        assert_eq!(result, 2);
        let result = world.get_num_alive_neighbours(1, 0);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_evolve() {
        let mut world = World {
            grid: vec![
                Dead, Dead, Dead, Dead, Dead, Dead, Dead, Alive, Dead, Dead, Dead, Dead, Alive,
                Dead, Dead, Dead, Dead, Alive, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
            ],
            width: 5,
            height: 5,
        };

        world.evolve();
        let assertion = vec![
            Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Alive, Alive, Alive,
            Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead, Dead,
        ];
        assert_eq!(world.grid, assertion);
    }
}
