use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Mul, Not};
use std::fmt::Display;

use crate::{Board, PieceType};

#[derive(Copy, Clone)]
pub struct BitBoard (u64);

pub const EMPTY : BitBoard = BitBoard(0);

impl BitBoard {
    pub fn set(&mut self, index: usize, value: bool) {
        if value {
            self.0 |= 1u64 << index;
        } else {
            self.0 &= !(1u64 << index);
        }
    }

    pub fn set_coord(&mut self, x: usize, y: usize, value: bool) {
        self.set(y * 8 + x, value);
    }

    pub fn predicate_and_set<F>(&mut self, x: u8, y: u8, predicate: F) -> bool 
        where F: Fn(u8, u8) -> bool
    {
        if predicate(x, y) {
            self.set(((y * 8) + x) as usize, true);
            return true;
        }

        false
    }

    pub fn is_empty_on_board_and_set(&mut self, board: &Board, x: u8, y: u8) -> bool {
        Board::is_inbounds(x, y) && self.predicate_and_set(x, y, |x, y| board.is_empty(x, y))
    }

    pub fn compare_and_set(&mut self, compare: &Self, compare_value: bool, x: u8, y: u8) -> bool {
        Board::is_inbounds(x, y) && self.predicate_and_set(x, y, |x, y| compare.get(y * 8 + x) == compare_value)
    }

    pub fn get(&self, index: u8) -> bool {
        (self.0 & (1u64 << index)) != 0
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = write!(f, "  ABCDEFGH\n");
        for (i, byte) in self.0.to_be_bytes().iter().enumerate() {
            res = write!(f, "{}|{:08b}\n", i, byte);
        }

        res 
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

// impl BitAnd for &BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn bitand(self, other: &BitBoard) -> BitBoard {
//         BitBoard(self.0 & other.0)
//     }
// }
//
// impl BitAnd<&BitBoard> for BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn bitand(self, other: &BitBoard) -> BitBoard {
//         BitBoard(self.0 & other.0)
//     }
// }
//
// impl BitAnd<BitBoard> for &BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn bitand(self, other: BitBoard) -> BitBoard {
//         BitBoard(self.0 & other.0)
//     }
// }

// Impl BitOr
impl BitOr for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

// impl BitOr for &BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn bitor(self, other: &BitBoard) -> BitBoard {
//         BitBoard(self.0 | other.0)
//     }
// }
//
// impl BitOr<&BitBoard> for BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn bitor(self, other: &BitBoard) -> BitBoard {
//         BitBoard(self.0 | other.0)
//     }
// }
//
// impl BitOr<BitBoard> for &BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn bitor(self, other: BitBoard) -> BitBoard {
//         BitBoard(self.0 | other.0)
//     }
// }

// Impl BitXor

impl BitXor for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn bitxor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ other.0)
    }
}

// impl BitXor for &BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn bitxor(self, other: &BitBoard) -> BitBoard {
//         BitBoard(self.0 ^ other.0)
//     }
// }
//
// impl BitXor<&BitBoard> for BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn bitxor(self, other: &BitBoard) -> BitBoard {
//         BitBoard(self.0 ^ other.0)
//     }
// }
//
// impl BitXor<BitBoard> for &BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn bitxor(self, other: BitBoard) -> BitBoard {
//         BitBoard(self.0 ^ other.0)
//     }
// }

// Impl BitAndAssign

impl BitAndAssign for BitBoard {
    #[inline]
    fn bitand_assign(&mut self, other: BitBoard) {
        self.0 &= other.0;
    }
}

// impl BitAndAssign<&BitBoard> for BitBoard {
//     #[inline]
//     fn bitand_assign(&mut self, other: &BitBoard) {
//         self.0 &= other.0;
//     }
// }

// Impl BitOrAssign
impl BitOrAssign for BitBoard {
    #[inline]
    fn bitor_assign(&mut self, other: BitBoard) {
        self.0 |= other.0;
    }
}

// impl BitOrAssign<&BitBoard> for BitBoard {
//     #[inline]
//     fn bitor_assign(&mut self, other: &BitBoard) {
//         self.0 |= other.0;
//     }
// }

// Impl BitXor Assign
impl BitXorAssign for BitBoard {
    #[inline]
    fn bitxor_assign(&mut self, other: BitBoard) {
        self.0 ^= other.0;
    }
}

// impl BitXorAssign<&BitBoard> for BitBoard {
//     #[inline]
//     fn bitxor_assign(&mut self, other: &BitBoard) {
//         self.0 ^= other.0;
//     }
// }

// Impl Mul
impl Mul for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn mul(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0.wrapping_mul(other.0))
    }
}

// impl Mul for &BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn mul(self, other: &BitBoard) -> BitBoard {
//         BitBoard(self.0.wrapping_mul(other.0))
//     }
// }
//
// impl Mul<&BitBoard> for BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn mul(self, other: &BitBoard) -> BitBoard {
//         BitBoard(self.0.wrapping_mul(other.0))
//     }
// }
//
// impl Mul<BitBoard> for &BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn mul(self, other: BitBoard) -> BitBoard {
//         BitBoard(self.0.wrapping_mul(other.0))
//     }
// }

// Impl Not
impl Not for BitBoard {
    type Output = BitBoard;

    #[inline]
    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}

// impl Not for &BitBoard {
//     type Output = BitBoard;
//
//     #[inline]
//     fn not(self) -> BitBoard {
//         BitBoard(!self.0)
//     }
// }
