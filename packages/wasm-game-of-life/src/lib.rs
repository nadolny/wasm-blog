mod utils;

extern crate web_sys;
use wasm_bindgen::prelude::*;
use nalgebra::geometry::Point2;
use std::fmt;
use js_sys::Uint8Array;
use std::convert::TryInto;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> Result<u8, String> {
        let neighbors = self.get_neighbors_unsafe(row, column);
        let result = neighbors.iter().fold(0, |acc, p| {
            acc + self.cells[self.get_index(p.x, p.y)] as u8
        });
        Ok(result)
    }

    fn get_neighbors(&self, row: u32, column: u32) -> Result<Vec<Point2<u32>>, String> {
        match (row, column) {
            (x,y) if x > self.width && y > self.height 
                => return Err(format!("The point ({},{}) is not in the universe, too wide and too tall", row, column)),
            (x,_y) if x > self.width 
                => return Err(format!("The point ({},{}) is not in the universe, out of width bountary", row, column)),
            (_x,y) if y > self.height 
                => return Err(format!("The point ({},{}) is not in the universe, out of height bountary", row, column)),
            (_, _) => (),
        };

        let input: Vec<Point2<i32>> = vec![
            Point2::new(-1,-1), Point2::new(-1,0), Point2::new(-1,1),
            Point2::new( 0,-1),                    Point2::new( 0,1),
            Point2::new( 1,-1), Point2::new( 1,0), Point2::new( 1,1),
        ];

        let output: Vec<Point2<u32>> = input.iter().map(
            |point| 
                Point2::<u32>::new(
                    (point.x + self.height as i32 + row as i32) as u32 % self.height,
                    (point.y + self.width as i32 + column as i32) as u32 % self.width,
                )
            ).collect::<Vec<Point2<u32>>>();
        Ok(output)
    }

    fn get_neighbors_unsafe(&self, row: u32, column: u32) -> [Point2<u32>; 8] {
        let x_shift = |x| (x + self.height as i32 + row as i32) as u32 % self.height;
        let y_shift = |y| (y + self.width as i32 + column as i32) as u32 % self.width;
        [
            Point2::<u32>::new(x_shift(-1), y_shift(-1)),
            Point2::<u32>::new(x_shift(-1), y_shift(0)),
            Point2::<u32>::new(x_shift(-1), y_shift(1)),
            Point2::<u32>::new(x_shift(0), y_shift(-1)),
            Point2::<u32>::new(x_shift(0), y_shift(1)),
            Point2::<u32>::new(x_shift(1), y_shift(-1)),
            Point2::<u32>::new(x_shift(1), y_shift(0)),
            Point2::<u32>::new(x_shift(1), y_shift(1)),
        ]
    }

    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

#[wasm_bindgen]
impl Universe{

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors.unwrap()) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 50;
        let height = 50;
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn cells_copy(&self) -> Uint8Array {
        let data : Vec<u8> = self.cells.clone().into_iter().map(|x| x as u8).collect(); 
        let ret = Uint8Array::new_with_length(data.len().try_into().unwrap());
        ret.copy_from(data.as_slice());
        ret
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..height * self.width).map(|_i| Cell::Dead).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn set_cell_alive(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx] = Cell::Alive;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line { 
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }
}

macro_rules! log {
    ( $( $t:tt) *) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
