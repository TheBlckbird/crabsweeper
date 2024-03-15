use crate::Board;

pub struct Field {
    pub state: FieldState,
    pub is_mine: bool,
    board: Board,
    index: usize,
}

pub enum FieldState {
    Closed,
    Opened,
    Flagged,
}

impl Default for FieldState {
    fn default() -> Self {
        Self::Closed
    }
}

impl Field {
    pub fn new(is_mine: bool, board: Board, index: usize) -> Self {
        Self {
            state: FieldState::default(),
            is_mine,
            board,
            index,
        }
    }

    pub fn pos_to_index(pos: (u8, u8), board_size: u8) -> isize {
        (pos.0 + (pos.1 * board_size)) as isize
    }

    pub fn get_pos(&self, board_size: u8) -> (u8, u8) {
        let x = self.index as u8 % board_size;
        (x, self.index as u8 / board_size)
    }

    pub fn open(&mut self) -> bool {
        true
    }

    fn get_neighbour_indixes(&self, board_size: u8) -> Vec<usize> {
        let mut temp_indexes = Vec::with_capacity(8);

        let pos = self.get_pos(board_size);

        temp_indexes.push(Self::pos_to_index((pos.0 - 1, pos.1 - 1), board_size)); // -1, -1
        temp_indexes.push(Self::pos_to_index((pos.0, pos.1 - 1), board_size)); // 0, -1
        temp_indexes.push(Self::pos_to_index((pos.0 + 1, pos.1 - 1), board_size)); // 1, -1
        temp_indexes.push(Self::pos_to_index((pos.0 - 1, pos.1), board_size)); // -1, 0
        temp_indexes.push(Self::pos_to_index((pos.0 + 1, pos.1), board_size)); // 1, 0
        temp_indexes.push(Self::pos_to_index((pos.0 - 1, pos.1 + 1), board_size)); // -1, 1
        temp_indexes.push(Self::pos_to_index((pos.0, pos.1 + 1), board_size)); // 0, 1
        temp_indexes.push(Self::pos_to_index((pos.0 + 1, pos.1 + 1), board_size)); // 1, 1

        let mut indexes = Vec::with_capacity(3);

        for item in temp_indexes.iter() {
            if !(*item < 0 || *item as usize >= self.board.borrow().len()) {
                indexes.push(*item as usize);
            }
        }

        indexes
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    #[test]
    fn test_pos_index_works() {
        assert_eq!(Field::pos_to_index((3, 6), 7), 45);
        assert_eq!(Field::pos_to_index((6, 3), 7), 27);
        assert_eq!(Field::pos_to_index((2, 0), 5), 2);
        assert_eq!(Field::pos_to_index((0, 2), 3), 6);
        assert_eq!(Field::pos_to_index((0, 0), 5), 0);

        let board = Rc::new(RefCell::new(Vec::new()));

        let field = Field::new(false, board.clone(), 3);
        assert_eq!(field.get_pos(5), (3, 0));

        let field = Field::new(false, board.clone(), 10);
        assert_eq!(field.get_pos(5), (0, 2));

        let field = Field::new(false, board.clone(), 12);
        assert_eq!(field.get_pos(5), (2, 2));

        let field = Field::new(false, board.clone(), 0);
        assert_eq!(field.get_pos(5), (0, 0));
    }
}
