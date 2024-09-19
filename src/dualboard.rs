use crate::{bitboard, BitBoard};

pub struct DualBoard {
    first: BitBoard,
    second: BitBoard
}

impl DualBoard {
    pub fn new() -> DualBoard {
        DualBoard {
            first: bitboard::EMPTY,
            second: bitboard::EMPTY,
        }
    }

    pub fn set(&mut self, index: usize) {
        if self.first.get(index) {
            self.second.set(index as usize, true);
        } else {
            self.first.set(index as usize, true);
        }
    }

    pub fn add(&mut self, board: BitBoard) {
        let double = self.first & board;
        if double.to_number() != 0 {
            self.second |= double;
        }
        self.first |= board;
    }
}
