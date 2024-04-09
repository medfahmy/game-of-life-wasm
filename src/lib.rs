mod utils;

use wasm_bindgen::prelude::*;
use fixedbitset::FixedBitSet;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Grid {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Grid {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        for delta_row in [self.height - 1, 0, 1] {
            for delta_col in [self.width - 1, 0, 1] {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let index = self.get_index(neighbor_row, neighbor_col);

                count += self.cells[index] as u8;
            }
        }

        count
    }
}

#[wasm_bindgen]
impl Grid {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, x) if x > 3 => false,
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    (cell, _) => cell,
                };

                next.set(self.get_index(row, col), next_cell);
            }
        }

        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const usize {
        self.cells.as_slice().as_ptr()
    }
}

// impl std::fmt::Display for Grid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for row in self.cells.as_slice().chunks(self.width as usize) {
//             for &cell in row {
//                 let symbol = if cell { '◼' } else { '◻' };
//                 write!(f, "{} ", symbol)?;
//             }

//             write!(f, "\n")?;
//         }

//         Ok(())
//     }
// }

#[wasm_bindgen]
impl Grid {
    pub fn new() -> Self {
        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            // cells.set(i, js_sys::Math::random() < 0.5);
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        Self {
            width,
            height,
            cells,
        }
    }

    // pub fn render(&self) -> String {
    //     self.to_string()
    // }
}
