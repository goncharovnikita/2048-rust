use crate::constants::{COLS_COUNT, ROWS_COUNT, GameMovementDirection};
use crate::block::{BlockSize};

pub type GameBoardArray = [[Option<BlockSize>; COLS_COUNT as usize]; ROWS_COUNT as usize];

pub struct GameBoard {
    game_board_array: GameBoardArray,
}

impl GameBoard {
    pub fn new(game_board_array: GameBoardArray) -> Self {
        GameBoard {
            game_board_array: game_board_array,
        }
    }

    pub fn get_cell(&self, x: u8, y: u8) -> Option<BlockSize> {
        self.game_board_array[y as usize][x as usize]
    }

    pub fn set_cell(&mut self, x: u8, y: u8, block: Option<BlockSize>) {
        self.game_board_array[y as usize][x as usize] = block;
    }

    pub fn pretty_string(&self) -> String {
        let mut result = String::from("\n");

        for arr in self.game_board_array.iter() {
            result.push_str("|");

            for cell in arr.iter() {
                if let Some(block_size) = cell {
                    result += &format!(" {} ", block_size.to_string())[..];
                } else {
                    result += " - "
                }

                result += "|";
            }

            result += "\n";
        }

        String::from(result)
    }

    pub fn steps(&self, x: u8, y: u8, direction: GameMovementDirection) -> u8 {
        let mut result = 0;

        let range = match direction {
            GameMovementDirection::Up => 0..y,
            GameMovementDirection::Down => y..ROWS_COUNT,
            GameMovementDirection::Right => x..COLS_COUNT,
            GameMovementDirection::Left => 0..x,
        };

        for cell_index in range {
            let cell = match direction {
                GameMovementDirection::Up => self.get_cell(x, cell_index),
                GameMovementDirection::Right => self.get_cell(cell_index, y),
                GameMovementDirection::Down => self.get_cell(x, cell_index),
                GameMovementDirection::Left => self.get_cell(cell_index, y),
            };

            if cell.is_none() {
                result += 1;
            }
        }

        println!("print result: {:?} for direction: {:?}", result, direction);

        result
    }

    pub fn move_board(&mut self, direction: GameMovementDirection) {
        let iter_range = match direction {
            GameMovementDirection::Up => (0..ROWS_COUNT).collect::<Vec<u8>>(),
            GameMovementDirection::Down => (0..ROWS_COUNT).rev().collect::<Vec<u8>>(),
            GameMovementDirection::Right => (0..COLS_COUNT).rev().collect::<Vec<u8>>(),
            GameMovementDirection::Left => (0..COLS_COUNT).collect::<Vec<u8>>(),
        };

        for y in iter_range.clone() {
            for x in iter_range.clone() {
                println!("iterating ({}, {})", x, y);

                if let Some(block) = self.get_cell(x, y) {
                    println!("got block at: ({}, {})", x, y);

                    let diff_x = match direction {
                        GameMovementDirection::Right => self.steps(x, y, direction) as i8,
                        GameMovementDirection::Left => -(self.steps(x, y, direction) as i8),
                        _ => 0,
                    };

                    let diff_y = match direction {
                        GameMovementDirection::Up => -(self.steps(x, y, direction) as i8),
                        GameMovementDirection::Down => self.steps(x, y, direction) as i8,
                        _ => 0,
                    };

                    println!("block diff: ({}, {})", diff_x, diff_y);

                    if diff_x != 0 || diff_y != 0 {
                        self.set_cell((x as i8 + diff_x) as u8, (y as i8 + diff_y) as u8, Some(block));
                        self.set_cell(x, y, None);
                    }
                }
            }
        }
    }
}