use termion::color;

use crate::{
    attacks::{
        get_bishop_moves, get_rook_moves, B_PAWN_CAPS, B_PAWN_DOUBLE_PUSHES, B_PAWN_PUSHES,
        KING_TABLE, KNIGHT_TABLE, W_PAWN_CAPS, W_PAWN_DOUBLE_PUSHES, W_PAWN_PUSHES,
    },
    moves::{self, Move, MoveFlags, MoveList},
    utils::{bb_to_idx, pop_lsb, print_bb},
};
#[derive(Copy, Clone)]
pub struct State {
    pub white: u64,
    pub black: u64,
    pub pawns: u64,
    pub rooks: u64,
    pub knights: u64,
    pub bishops: u64,
    pub queens: u64,
    pub kings: u64,
    pub ep: u8,
    pub ply: u8,
    pub color: Color,
    pub castle: u8, // Wk, Wq, Bk, Bq
}
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}
impl State {
    pub fn all_white(&self) -> u64 {
        let ret = self.pawns | self.knights | self.bishops | self.rooks | self.kings | self.queens;

        ret & self.white
    }
    pub fn all_black(&self) -> u64 {
        let ret = self.pawns | self.knights | self.bishops | self.rooks | self.kings | self.queens;

        ret & self.black
    }
    pub fn new() -> Self {
        {
            Self {
                white: 0xffff,
                black: 0xffff000000000000,
                pawns: 0xff00000000ff00,
                rooks: 0x8100000000000081,
                knights: 0x4200000000000042,
                bishops: 0x2400000000000024,
                queens: 0x800000000000008,
                kings: 0x1000000000000010,
                ep: 0,
                ply: 0,
                color: Color::White,
                castle: 0xf,
            }
        }
    }
    pub fn clear_square(&mut self, idx: u64) {
        let mask = !(1 << idx);

        self.pawns &= mask;
        self.knights &= mask;
        self.bishops &= mask;
        self.rooks &= mask;
        self.queens &= mask;
        //NOTE: No need to delete the king, as it is impossible to capture
    }
    pub fn clear_color(&mut self, idx: u64) {
        let mask = !(1 << idx);

        self.black &= mask;
        self.white &= mask;
    }
    pub fn pseudo_legal_moves(&self, _captures_only: bool) -> MoveList {
        let mut mlist = MoveList::new();
        let all_pieces = self.all_white() | self.all_black();
        let enemy_pieces = match self.color {
            Color::White => self.all_black(),
            Color::Black => self.all_white(),
        };
        let my_pieces = match self.color {
            Color::White => self.all_white(),
            Color::Black => self.all_black(),
        };
        let mut pawns = self.pawns & my_pieces;
        while pawns != 0 {
            fn promo(color: &Color, mlist: &mut MoveList, from_bb: u64, move_bb: u64) -> bool {
                const PROMO_MASK: u64 = 0xff000000000000ff;
                if move_bb & PROMO_MASK == 0 {
                    return false;
                }
                match color {
                    Color::White => {
                        mlist.push_move(from_bb, move_bb, MoveFlags::PromoRook);
                        mlist.push_move(from_bb, move_bb, MoveFlags::PromoQueen);
                        mlist.push_move(from_bb, move_bb, MoveFlags::PromoKnight);
                        mlist.push_move(from_bb, move_bb, MoveFlags::PromoBishop);
                    }
                    Color::Black => {
                        mlist.push_move(from_bb, move_bb, MoveFlags::PromoRook);
                        mlist.push_move(from_bb, move_bb, MoveFlags::PromoQueen);
                        mlist.push_move(from_bb, move_bb, MoveFlags::PromoKnight);
                        mlist.push_move(from_bb, move_bb, MoveFlags::PromoBishop);
                    }
                }
                true
            }
            let pawn = pop_lsb(&mut pawns);
            let from_idx = bb_to_idx(pawn);
            if self.color == Color::White {
                let push = W_PAWN_PUSHES[from_idx] & !all_pieces;
                if push != 0 {
                    if !promo(&self.color, &mut mlist, pawn, push) {
                        mlist.push_move(pawn, W_PAWN_PUSHES[from_idx], MoveFlags::PawnMove);
                        if W_PAWN_DOUBLE_PUSHES[from_idx] & !all_pieces != 0 {
                            mlist.push_move(
                                pawn,
                                W_PAWN_DOUBLE_PUSHES[from_idx],
                                MoveFlags::PawnDoublePush,
                            );
                        }
                    }
                }
                let mut caps = W_PAWN_CAPS[from_idx] & enemy_pieces;
                while caps != 0 {
                    let cap = pop_lsb(&mut caps);
                    if !promo(&self.color, &mut mlist, pawn, cap) {
                        mlist.push_move(pawn, cap, MoveFlags::PawnMove);
                    }
                }
                if self.ep != 0 {
                    let ep = W_PAWN_CAPS[from_idx] & (1 << self.ep);
                    if ep != 0 {
                        mlist.push_move(pawn, 1 << self.ep, MoveFlags::EP);
                    }
                }
            } else {
                let push = B_PAWN_PUSHES[from_idx] & !all_pieces;
                if push != 0 {
                    if !promo(&self.color, &mut mlist, pawn, push) {
                        mlist.push_move(pawn, B_PAWN_PUSHES[from_idx], MoveFlags::PawnMove);
                        if B_PAWN_DOUBLE_PUSHES[from_idx] & !all_pieces != 0 {
                            mlist.push_move(
                                pawn,
                                B_PAWN_DOUBLE_PUSHES[from_idx],
                                MoveFlags::PawnDoublePush,
                            );
                        }
                    }
                }
                let mut caps = B_PAWN_CAPS[from_idx] & enemy_pieces;
                while caps != 0 {
                    let cap = pop_lsb(&mut caps);
                    if !promo(&self.color, &mut mlist, pawn, cap) {
                        mlist.push_move(pawn, cap, MoveFlags::PawnMove);
                    }
                }
                if self.ep != 0 {
                    let ep = B_PAWN_CAPS[from_idx] & (1 << self.ep);
                    if ep != 0 {
                        mlist.push_move(pawn, 1 << self.ep, MoveFlags::EP);
                    }
                }
            }
        }
        let mut knights = self.knights & my_pieces;
        while knights != 0 {
            let p = pop_lsb(&mut knights);
            let mut moves = KNIGHT_TABLE[bb_to_idx(p)] & !my_pieces;
            while moves != 0 {
                let m = pop_lsb(&mut moves);
                mlist.push_move(p, m, MoveFlags::KnightMove);
            }
        }
        let mut bishops = self.bishops & my_pieces;
        while bishops != 0 {
            let p = pop_lsb(&mut bishops);
            let mut moves = get_bishop_moves(p, all_pieces) & !my_pieces;
            while moves != 0 {
                let m = pop_lsb(&mut moves);
                mlist.push_move(p, m, MoveFlags::BishopMove);
            }
        }
        let mut rooks = self.rooks & my_pieces;
        while rooks != 0 {
            let p = pop_lsb(&mut rooks);
            let mut moves = get_rook_moves(p, all_pieces) & !my_pieces;
            while moves != 0 {
                let m = pop_lsb(&mut moves);
                mlist.push_move(p, m, MoveFlags::RookMove);
            }
        }
        let mut queens = self.queens & my_pieces;
        while queens != 0 {
            let p = pop_lsb(&mut queens);
            let mut moves = get_rook_moves(p, all_pieces);
            moves |= get_bishop_moves(p, all_pieces);
            moves &= !my_pieces;
            while moves != 0 {
                let m = pop_lsb(&mut moves);
                mlist.push_move(p, m, MoveFlags::QueenMove);
            }
        }
        let mut kings = self.kings & my_pieces;
        while kings != 0 {
            let p = pop_lsb(&mut kings);
            let mut moves = KING_TABLE[bb_to_idx(p)] & !my_pieces;
            while moves != 0 {
                let m = pop_lsb(&mut moves);
                mlist.push_move(p, m, MoveFlags::KingMove);
            }
            if self.in_check() {
                break;
            }
            match self.color {
                Color::White => {
                    if self.castle & 0b1000 != 0 {
                        // Wq
                        let mask_wq = 0xe;
                        if all_pieces & mask_wq == 0
                            && !self.under_attack(1 << 2)
                            && !self.under_attack(1 << 3)
                        {
                            mlist.push_move(p, 1 << 2, MoveFlags::Castle);
                        }
                    }
                    if self.castle & 0b0100 != 0 {
                        //Wk
                        let mask_wk = 0x60;
                        if all_pieces & mask_wk == 0
                            && !self.under_attack(1 << 5)
                            && !self.under_attack(1 << 6)
                        {
                            mlist.push_move(p, 1 << 6, MoveFlags::Castle);
                        }
                    }
                }
                Color::Black => {
                    if self.castle & 0b0010 != 0 {
                        //Bq
                        let mask_bq = 0xe00000000000000;
                        if all_pieces & mask_bq == 0
                            && !self.under_attack(1 << 58)
                            && !self.under_attack(1 << 59)
                        {
                            mlist.push_move(p, 1 << 58, MoveFlags::Castle);
                        }
                    }
                    if self.castle & 0b0001 != 0 {
                        // Bk
                        let mask_bk = 0x6000000000000000;
                        if all_pieces & mask_bk == 0
                            && !self.under_attack(1 << 61)
                            && !self.under_attack(1 << 62)
                        {
                            mlist.push_move(p, 1 << 62, MoveFlags::Castle);
                        }
                    }
                }
            }
        }
        mlist
    }
    pub fn print_board_move(&self, m: Move) {
        fn check_bit(bb: u64, bit: u8) -> bool {
            bb & 1 << bit != 0
        }
        let from = bb_to_idx(m.from()) as u8;
        let to = bb_to_idx(m.to()) as u8;
        println!("from: {from}, to: {to}");
        for y in 0..8 {
            print!("{} ", 8 - y);
            for x in 0..8 {
                print!("{}", color::Fg(color::Rgb(0, 0, 0)));
                if (x + y) % 2 == 1 {
                    print!("{}", color::Bg(color::Rgb(181, 136, 99)));
                } else {
                    print!("{}", color::Bg(color::Rgb(0xf0, 0xd9, 0xb5)));
                }
                let bit: u8 = (7 - y) * 8 + x;
                if bit == from {
                    print!("{}", color::Bg(color::Rgb(1, 136, 1)));
                } else if bit == to {
                    print!("{}", color::Bg(color::Rgb(100, 6, 1)));
                }
                if bit == from && bit == to {
                    print!("{}", color::Bg(color::Rgb(100, 100, 100)));
                }
                let p;
                if check_bit(self.white, bit) {
                    print!("{}", color::Fg(color::Rgb(50, 30, 200)));
                } else if check_bit(self.black, bit) {
                    print!("{}", color::Fg(color::Rgb(0, 0, 0)));
                } else {
                    print!("{}", color::Fg(color::Red));
                }
                if check_bit(self.knights, bit) {
                    p = "♞";
                } else if check_bit(self.bishops, bit) {
                    p = "♝";
                } else if check_bit(self.rooks, bit) {
                    p = "♜";
                } else if check_bit(self.kings, bit) {
                    p = "♚";
                } else if check_bit(self.queens, bit) {
                    p = "♛";
                } else if check_bit(self.pawns, bit) {
                    p = "♟";
                } else {
                    p = " ";
                }
                print!("{p} ");
            }
            println!("{}", color::Bg(color::Reset));
            print!("{}", color::Fg(color::Reset));
        }
        print!("  ");
        for letter in 'a'..='h' {
            print!("{} ", letter);
        }
        println!();
    }
    pub fn print_board(&self) {
        fn check_bit(bb: u64, bit: u8) -> bool {
            bb & 1 << bit != 0
        }
        for y in 0..8 {
            print!("{} ", 8 - y);
            for x in 0..8 {
                print!("{}", color::Fg(color::Rgb(0, 0, 0)));
                if (x + y) % 2 == 1 {
                    print!("{}", color::Bg(color::Rgb(181, 136, 99)));
                } else {
                    print!("{}", color::Bg(color::Rgb(0xf0, 0xd9, 0xb5)));
                }
                let bit = (7 - y) * 8 + x;
                let p;
                if check_bit(self.white, bit) {
                    print!("{}", color::Fg(color::Rgb(50, 30, 200)));
                } else if check_bit(self.black, bit) {
                    print!("{}", color::Fg(color::Rgb(0, 0, 0)));
                } else {
                    print!("{}", color::Fg(color::Red));
                }
                if check_bit(self.knights, bit) {
                    p = "♞";
                } else if check_bit(self.bishops, bit) {
                    p = "♝";
                } else if check_bit(self.rooks, bit) {
                    p = "♜";
                } else if check_bit(self.kings, bit) {
                    p = "♚";
                } else if check_bit(self.queens, bit) {
                    p = "♛";
                } else if check_bit(self.pawns, bit) {
                    p = "♟";
                } else {
                    p = " ";
                }
                print!("{p} ");
            }
            println!("{}", color::Bg(color::Reset));
            print!("{}", color::Fg(color::Reset));
        }
        print!("  ");
        for letter in 'a'..='h' {
            print!("{} ", letter);
        }
        println!();
    }
    pub fn eprint_board(&self) {
        fn check_bit(bb: u64, bit: u8) -> bool {
            bb & 1 << bit != 0
        }
        for y in 0..8 {
            eprint!("{} ", 8 - y);
            for x in 0..8 {
                eprint!("{}", color::Fg(color::Rgb(0, 0, 0)));
                if (x + y) % 2 == 1 {
                    eprint!("{}", color::Bg(color::Rgb(181, 136, 99)));
                } else {
                    eprint!("{}", color::Bg(color::Rgb(0xf0, 0xd9, 0xb5)));
                }
                let bit = (7 - y) * 8 + x;
                let p;
                if check_bit(self.white, bit) {
                    eprint!("{}", color::Fg(color::Rgb(50, 30, 200)));
                } else if check_bit(self.black, bit) {
                    eprint!("{}", color::Fg(color::Rgb(0, 0, 0)));
                } else {
                    eprint!("{}", color::Fg(color::Red));
                }
                if check_bit(self.knights, bit) {
                    p = "♞";
                } else if check_bit(self.bishops, bit) {
                    p = "♝";
                } else if check_bit(self.rooks, bit) {
                    p = "♜";
                } else if check_bit(self.kings, bit) {
                    p = "♚";
                } else if check_bit(self.queens, bit) {
                    p = "♛";
                } else if check_bit(self.pawns, bit) {
                    p = "♟";
                } else {
                    p = " ";
                }
                eprint!("{p} ");
            }
            eprintln!("{}", color::Bg(color::Reset));
            eprint!("{}", color::Fg(color::Reset));
        }
        eprint!("  ");
        for letter in 'a'..='h' {
            eprint!("{} ", letter);
        }
        eprintln!();
    }
}
