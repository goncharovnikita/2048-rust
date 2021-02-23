use rand::Rng;

use crate::constants::{COLS_COUNT, ROWS_COUNT, GameMovementDirection};
use crate::block::{BlockSize};

pub type GameBoardArray = [[Option<BlockSize>; COLS_COUNT as usize]; ROWS_COUNT as usize];

type Patchset = Vec<((u8, u8), (u8, u8))>;

pub struct GameBoard {
    game_board_array: GameBoardArray,
}

impl GameBoard {
    pub fn new() -> Self {
        let game_board_array: GameBoardArray = [[None; COLS_COUNT as usize]; ROWS_COUNT as usize];

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

        let range_start = match direction {
            GameMovementDirection::Up => 0,
            GameMovementDirection::Down => y,
            GameMovementDirection::Right => x,
            GameMovementDirection::Left => 0,
        };

        let range_end = match direction {
            GameMovementDirection::Up => y,
            GameMovementDirection::Down => ROWS_COUNT - 1,
            GameMovementDirection::Right => COLS_COUNT - 1,
            GameMovementDirection::Left => x,
        };

        let range = range_start..(range_end + 1);

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

        let mut merge_iterator_start = range_end.clone() as i8;

        while merge_iterator_start >= range_start as i8 {
            if merge_iterator_start == range_end as i8 {
                merge_iterator_start -= 1;

                continue;
            }

            let curr_cell = match direction {
                GameMovementDirection::Up => self.get_cell(x, merge_iterator_start as u8),
                GameMovementDirection::Right => self.get_cell(merge_iterator_start as u8, y),
                GameMovementDirection::Down => self.get_cell(x, merge_iterator_start as u8),
                GameMovementDirection::Left => self.get_cell(merge_iterator_start as u8, y),
            };

            let mut prev_cell: Option<BlockSize> = None;
            let mut prev_cell_iterator = merge_iterator_start + 1;

            while prev_cell_iterator <= range_end as i8 {
                prev_cell = match direction {
                    GameMovementDirection::Up => self.get_cell(x, prev_cell_iterator as u8),
                    GameMovementDirection::Right => self.get_cell(prev_cell_iterator as u8, y),
                    GameMovementDirection::Down => self.get_cell(x, prev_cell_iterator as u8),
                    GameMovementDirection::Left => self.get_cell(prev_cell_iterator as u8, y),
                };

                if prev_cell.is_some() {
                    break;
                }

                prev_cell_iterator += 1;
            }

            if 
                curr_cell.is_some() &&
                prev_cell.is_some() &&
                curr_cell.unwrap() == prev_cell.unwrap()
            {
                result += 1;
                merge_iterator_start -= 1;
            }

            merge_iterator_start -= 1;
        }

        result
    }

    pub fn rand_available_cell(&self) -> (u8, u8) {
        let mut rng = rand::thread_rng();

        let iterate_y: bool = rng.gen();
        let iterate_forward: bool = rng.gen();
        let first_index = rng.gen_range(0..COLS_COUNT);
        let second_index = rng.gen_range(0..ROWS_COUNT);

        let (first_start, second_start) = if iterate_forward {
            (first_index, second_index)
        } else {
            (0, 0)
        };

        let (first_end, second_end) = if iterate_forward {
            (ROWS_COUNT, COLS_COUNT)
        } else {
            (first_index, second_index)
        };

        let first_range = first_start..first_end;
        let second_range = second_start..second_end;

        for first_index in first_range.clone() {
            for second_index in second_range.clone() {
                let x = if iterate_y { first_index } else { second_index };
                let y = if iterate_y { second_index } else { first_index };

                if self.get_cell(x, y).is_none() {
                    return (x, y)
                }
            }
        }

        for y in 0..ROWS_COUNT {
            for x in 0..COLS_COUNT {
                if self.get_cell(x, y).is_none() {
                    return (x, y)
                }
            }
        }

        (first_index, second_index)
    }

    pub fn move_board(&mut self, direction: GameMovementDirection) {
        let patchset = self.gen_patchset(direction);

        self.apply_patchset(patchset);
    }

    fn gen_patchset(&self, direction: GameMovementDirection) -> Patchset {
        let iter_range = match direction {
            GameMovementDirection::Up => (0..ROWS_COUNT).collect::<Vec<u8>>(),
            GameMovementDirection::Down => (0..ROWS_COUNT).rev().collect::<Vec<u8>>(),
            GameMovementDirection::Right => (0..COLS_COUNT).rev().collect::<Vec<u8>>(),
            GameMovementDirection::Left => (0..COLS_COUNT).collect::<Vec<u8>>(),
        };

        let mut patchset: Patchset = Vec::new();

        for y in iter_range.clone() {
            for x in iter_range.clone() {
                if self.get_cell(x, y).is_some() {
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

                    if diff_x != 0 || diff_y != 0 {
                        let new_x = (x as i8 + diff_x) as u8;
                        let new_y = (y as i8 + diff_y) as u8;

                        patchset.push(((x, y), (new_x, new_y)));
                    }
                }
            }
        }

        patchset
    }

    fn apply_patchset(&mut self, patchset: Patchset) {
        for patch in patchset.into_iter() {
            let ((x, y), (new_x, new_y)) = patch;

            if let Some(block) = self.get_cell(x, y) {
                if let Some(block_to_merge) = self.get_cell(new_x, new_y) {
                    self.set_cell(new_x, new_y, Some(block_to_merge.next().unwrap()));
                } else {
                    self.set_cell(new_x, new_y, Some(block));
                }

                self.set_cell(x, y, None);
            }
        }
    }
}

#[cfg(test)]
mod game_board_tests {
    use super::*;
    use crate::block::*;
    use crate::constants::*;

    #[test]
    fn steps_plain() {
        let mut game_board = GameBoard::new();

        game_board.set_cell(0, 0, Some(BlockSize::_2));

        println!("{}", game_board.pretty_string());

        assert!(game_board.get_cell(0, 0).is_some());
        assert!(game_board.get_cell(0, 3).is_none());

        game_board.move_board(GameMovementDirection::Down);

        println!("{}", game_board.pretty_string());

        assert!(game_board.get_cell(0, 0).is_none());
        assert!(game_board.get_cell(0, 3).is_some());
    }

    #[test]
    fn steps_multi_blocks() {
        let mut game_board = GameBoard::new();

        game_board.set_cell(0, 0, Some(BlockSize::_2));
        game_board.set_cell(0, 1, Some(BlockSize::_4));

        println!("{}", game_board.pretty_string());

        assert!(game_board.get_cell(0, 0).is_some());
        assert!(game_board.get_cell(0, 1).is_some());
        assert!(game_board.get_cell(0, 3).is_none());
        assert!(game_board.get_cell(0, 2).is_none());

        game_board.move_board(GameMovementDirection::Down);

        println!("{}", game_board.pretty_string());

        assert!(game_board.get_cell(0, 0).is_none());
        assert!(game_board.get_cell(0, 1).is_none());
        assert!(game_board.get_cell(0, 3).is_some());
        assert!(game_board.get_cell(0, 2).is_some());

        assert_eq!(game_board.get_cell(0, 3).unwrap(), BlockSize::_4);
        assert_eq!(game_board.get_cell(0, 2).unwrap(), BlockSize::_2);
    }

    #[test]
    fn blocks_merging() {
        let mut game_board = GameBoard::new();

        game_board.set_cell(0, 0, Some(BlockSize::_2));
        game_board.set_cell(0, 1, Some(BlockSize::_2));

        println!("{}", game_board.pretty_string());

        assert!(game_board.get_cell(0, 0).is_some());
        assert!(game_board.get_cell(0, 1).is_some());
        assert!(game_board.get_cell(0, 3).is_none());
        assert!(game_board.get_cell(0, 2).is_none());

        game_board.move_board(GameMovementDirection::Down);

        println!("{}", game_board.pretty_string());

        assert!(game_board.get_cell(0, 0).is_none());
        assert!(game_board.get_cell(0, 1).is_none());
        assert!(game_board.get_cell(0, 2).is_none());
        assert!(game_board.get_cell(0, 3).is_some());

        assert_eq!(game_board.get_cell(0, 3).unwrap(), BlockSize::_4);
    }

    #[test]
    fn complex_blocks_merging() {
        let mut game_board = GameBoard::new();

        game_board.set_cell(0, 3, Some(BlockSize::_2));
        game_board.set_cell(1, 3, Some(BlockSize::_2));
        game_board.set_cell(2, 3, Some(BlockSize::_4));

        println!("{}", game_board.pretty_string());

        assert!(game_board.get_cell(0, 3).is_some());
        assert!(game_board.get_cell(1, 3).is_some());
        assert!(game_board.get_cell(2, 3).is_some());

        game_board.move_board(GameMovementDirection::Left);

        println!("{}", game_board.pretty_string());

        assert!(game_board.get_cell(0, 3).is_some());
        assert!(game_board.get_cell(1, 3).is_some());
        assert!(game_board.get_cell(2, 3).is_none());

        assert_eq!(game_board.get_cell(0, 3).unwrap(), BlockSize::_4);
        assert_eq!(game_board.get_cell(1, 3).unwrap(), BlockSize::_4);
    }

    #[test]
    fn rand_cells() {
        let mut game_board = GameBoard::new();

        for _x in 0..ROWS_COUNT {
            for _y in 0..COLS_COUNT {
                println!("testing: ({}, {})", _x, _y);

                let (x, y) = game_board.rand_available_cell();
                assert!(game_board.get_cell(x, y).is_none());

                game_board.set_cell(x, y, Some(BlockSize::_2));
            }
        }
    }
}
