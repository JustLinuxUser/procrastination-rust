use crate::{state::State, utils::bb_to_idx};

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

impl Move {
    pub fn new(from: u64, to: u64, flags: MoveFlags) -> Self {
        let mut data: u16 = 0;
        data |= bb_to_idx(from) as u16;
        data |= (bb_to_idx(to) << 6) as u16;
        data |= (flags as u16) << 12;
        Move { data }
    }
    pub fn from(&self) -> u64 {
        1 << (self.data & 0b111_111)
    }
    pub fn to(&self) -> u64 {
        1 << (self.data >> 6 & 0b111_111)
    }
    pub fn flags(&self) -> MoveFlags {
        ((self.data >> 12 & 0b1111) as u8).into()
    }

    pub fn to_text(&self) -> String {
        let mut ret = String::new();
        ret.push(((bb_to_idx(self.from()) % 8) as u8 + b'a') as char);
        ret.push(((bb_to_idx(self.from()) / 8) as u8 + b'1') as char);
        ret.push(((bb_to_idx(self.to()) % 8) as u8 + b'a') as char);
        ret.push(((bb_to_idx(self.to()) / 8) as u8 + b'1') as char);
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
    moves: [Move; 256],
    capacity: usize,
    iter: usize,
}

impl MoveList {
    pub fn new() -> Self {
        MoveList {
            moves: [Move::new(0, 0, MoveFlags::PawnMove); 256],
            capacity: 0,
            iter: 0,
        }
    }
    pub fn push(&mut self, m: Move) {
        self.moves[self.capacity] = m;
        self.capacity += 1;
    }
    pub fn push_move(&mut self, from_bb: u64, to_bb: u64, flag: MoveFlags) {
        let m = Move::new(from_bb as u64, to_bb as u64, flag);
        self.moves[self.capacity] = m;
        self.capacity += 1;
    }
}

impl Iterator for MoveList {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter < self.capacity {
            self.iter += 1;
            return Some(self.moves[self.iter - 1]);
        }
        None
    }
}

#[test]
fn test_move_integrity1() {
    let m = Move::new(1, 2, MoveFlags::Castle);
    assert_eq!(m.from(), 1);
    assert_eq!(m.to(), 2);
    assert_eq!(m.flags(), MoveFlags::Castle);
}

#[test]
fn test_move_integrity2() {
    let from_bb = 1 << 63;
    let m = Move::new(from_bb, 2, MoveFlags::QueenMove);
    assert_eq!(m.from(), from_bb);
    assert_eq!(m.to(), 2);
    assert_eq!(m.flags(), MoveFlags::QueenMove);
}
