pub mod movegen;
mod utils;

use std::u64;

use movegen::{
    find_magic, gen_bishop_potential_moves, gen_rook_potential_moves, init_magics, KNIGHT_TABLE,
};
use utils::bitboard_utils as BB;

#[derive(Copy, Clone)]
struct State {
    white: u64,
    black: u64,
    pawns: u64,
    rooks: u64,
    knights: u64,
    bishops: u64,
    queens: u64,
    kings: u64,
    ep: i8,
    ply: u8,
}
impl State {
    pub fn all_white(&self) -> u64 {
        let ret = self.pawns | self.knights | self.bishops | self.rooks | self.kings | self.queens;
        let ret = ret & self.white;
        ret
    }
    pub fn all_black(&self) -> u64 {
        let ret = self.pawns | self.knights | self.bishops | self.rooks | self.kings | self.queens;
        let ret = ret & self.black;
        ret
    }
    pub fn new() -> Self {
        return {
            Self {
                white: 0xffff,
                black: 0xffff000000000000,
                pawns: 0xff00000000ff00,
                rooks: 0x8100000000000081,
                knights: 0x4200000000000042,
                bishops: 0x2400000000000024,
                queens: 0x800000000000008,
                kings: 0x1000000000000010,
                ep: -1,
                ply: 0,
            }
        };
    }
}

fn main() {
    let bb = 0b1110000100110000000000000000000000000000000000000000000010000001;
    BB::print(bb);
    //find_magic();
    //gen_rook_potential_moves();
    init_magics(false);
    let a: u64 = 1;
    let board = State::new();
}
