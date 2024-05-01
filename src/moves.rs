use std::vec;

use crate::core_types::{SquareIdx, BB};

/// Layout  MoveFlags   To     From
///           1111    111111  111111
#[derive(Copy, Clone)]
pub struct Move {
    // 6 bits for from
    // 6 bits for to
    // 4 bits left for a flag,
    pub data: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveFlags {
    PromoQueen,
    PromoRook,
    PromoKnight,
    PromoBishop,
    EP,
    Castle,
    PawnMove,
    PawnDoublePush,
    BishopMove,
    KnightMove,
    RookMove,
    KingMove,
    QueenMove,
}
impl From<u8> for MoveFlags {
    fn from(value: u8) -> Self {
        match value {
            0 => MoveFlags::PromoQueen,
            1 => MoveFlags::PromoRook,
            2 => MoveFlags::PromoKnight,
            3 => MoveFlags::PromoBishop,
            4 => MoveFlags::EP,
            5 => MoveFlags::Castle,
            6 => MoveFlags::PawnMove,
            7 => MoveFlags::PawnDoublePush,
            8 => MoveFlags::BishopMove,
            9 => MoveFlags::KnightMove,
            10 => MoveFlags::RookMove,
            11 => MoveFlags::KingMove,
            12 => MoveFlags::QueenMove,
            _ => panic!("Unexpected flag"),
        }
    }
}

impl Default for Move {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl Move {
    pub fn new(from: SquareIdx, to: SquareIdx, flags: MoveFlags) -> Self {
        let mut data: u16 = 0;
        // <flags(4)> <to(6)> <from(6)>
        data |= from.0 as u16;
        data |= (to.0 as u16) << 6;
        data |= (flags as u16) << 12;
        Move { data }
    }
    pub fn get_from(&self) -> BB {
        BB(1 << (self.data & 0b111_111))
    }
    pub fn get_to(&self) -> BB {
        BB(1 << (self.data >> 6 & 0b111_111))
    }
    pub fn get_from_idx(&self) -> SquareIdx {
        SquareIdx((self.data & 0b111_111) as u8)
    }
    pub fn get_to_idx(&self) -> SquareIdx {
        SquareIdx((self.data >> 6 & 0b111_111) as u8)
    }
    pub fn flags(&self) -> MoveFlags {
        ((self.data >> 12 & 0b1111) as u8).into()
    }

    pub fn as_text(&self) -> String {
        let mut ret = String::new();
        ret.push(((self.get_from_idx().0 % 8) + b'a') as char);
        ret.push(((self.get_from_idx().0 / 8) + b'1') as char);
        ret.push(((self.get_to_idx().0 % 8) + b'a') as char);
        ret.push(((self.get_to_idx().0 / 8) + b'1') as char);
        match self.flags() {
            MoveFlags::PromoQueen => ret.push('q'),
            MoveFlags::PromoRook => ret.push('r'),
            MoveFlags::PromoKnight => ret.push('n'),
            MoveFlags::PromoBishop => ret.push('b'),
            _ => (),
        }
        ret
    }
}

pub struct MoveList {
    pub moves: Vec<Move>,
}

impl MoveList {
    pub fn new() -> Self {
        MoveList {
            moves: Vec::with_capacity(30),
        }
    }
    pub fn push(&mut self, m: Move) {
        self.moves.push(m);
    }
    pub fn push_move(&mut self, from: BB, to: BB, flag: MoveFlags) {
        let m = Move::new(SquareIdx::from(from), SquareIdx::from(to), flag);
        self.push(m)
    }
}

impl IntoIterator for MoveList {
    type Item = Move;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves.into_iter()
    }
}

#[test]
fn test_move_integrity1() {
    let m = Move::new(SquareIdx(1), SquareIdx(32), MoveFlags::Castle);
    assert_eq!(m.get_from_idx().0, 1);
    assert_eq!(m.get_to_idx().0, 32);
    assert_eq!(m.flags(), MoveFlags::Castle);
}
#[test]
fn test_move_integrity2() {
    //a7a5
    let m = Move::new(SquareIdx(7 * 8), SquareIdx(8 * 5), MoveFlags::Castle);
    assert_eq!(m.get_from_idx().0, 7 * 8);
    assert_eq!(m.get_to_idx().0, 5 * 8);
    assert_eq!(m.flags(), MoveFlags::Castle);
}
