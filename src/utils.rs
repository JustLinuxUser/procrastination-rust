use crate::moves::Move;
use crate::{
    board::Board,
    core_types::{Color, Piece},
};
use termion::color;

pub fn check_bit(bb: u64, bit: u8) -> bool {
    bb & 1 << bit != 0
}

#[allow(dead_code)]
pub fn print_bb(bb: u64) {
    for y in 0..8 {
        print!("{} ", 8 - y);
        for x in 0..8 {
            print!("{}", color::Fg(color::Rgb(0, 0, 0)));
            if (x + y) % 2 == 1 {
                print!("{}", color::Bg(color::Rgb(181, 136, 99)));
            } else {
                print!("{}", color::Bg(color::Rgb(0xf0, 0xd9, 0xb5)));
            }
            if check_bit(bb, (7 - y) * 8 + x) {
                print!("{}🔴", color::Fg(color::Rgb(30, 10, 100)));
            } else {
                print!("  ");
            }
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

impl Board {
    #[allow(dead_code)]
    pub fn eprint_board_move(&self, m: Move) {
        fn check_bit(bb: u64, bit: u8) -> bool {
            bb & 1 << bit != 0
        }
        let from = m.get_to_idx().0;
        let to = m.get_to_idx().0;
        eprintln!("from: {from}, to: {to}");
        for y in 0..8 {
            eprint!("{} ", 8 - y);
            for x in 0..8 {
                eprint!("{}", color::Fg(color::Rgb(0, 0, 0)));
                if (x + y) % 2 == 1 {
                    eprint!("{}", color::Bg(color::Rgb(181, 136, 99)));
                } else {
                    eprint!("{}", color::Bg(color::Rgb(0xf0, 0xd9, 0xb5)));
                }
                let bit: u8 = (7 - y) * 8 + x;
                if bit == from {
                    eprint!("{}", color::Bg(color::Rgb(1, 136, 1)));
                } else if bit == to {
                    eprint!("{}", color::Bg(color::Rgb(100, 6, 1)));
                }
                if bit == from && bit == to {
                    eprint!("{}", color::Bg(color::Rgb(100, 100, 100)));
                }
                let p;
                if check_bit(self.side[Color::White as usize].0, bit) {
                    eprint!("{}", color::Fg(color::Rgb(50, 30, 200)));
                } else if check_bit(self.side[Color::Black as usize].0, bit) {
                    eprint!("{}", color::Fg(color::Rgb(0, 0, 0)));
                } else {
                    eprint!("{}", color::Fg(color::Red));
                }
                if check_bit(self.pieces[Piece::Knight as usize].0, bit) {
                    p = "♞";
                } else if check_bit(self.pieces[Piece::Bishop as usize].0, bit) {
                    p = "♝";
                } else if check_bit(self.pieces[Piece::Rook as usize].0, bit) {
                    p = "♜";
                } else if check_bit(self.pieces[Piece::King as usize].0, bit) {
                    p = "♚";
                } else if check_bit(self.pieces[Piece::Queen as usize].0, bit) {
                    p = "♛";
                } else if check_bit(self.pieces[Piece::Pawn as usize].0, bit) {
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
                let bit: u8 = (7 - y) * 8 + x;
                let p;
                if check_bit(self.side[Color::White as usize].0, bit) {
                    eprint!("{}", color::Fg(color::Rgb(50, 30, 200)));
                } else if check_bit(self.side[Color::Black as usize].0, bit) {
                    eprint!("{}", color::Fg(color::Rgb(0, 0, 0)));
                } else {
                    eprint!("{}", color::Fg(color::Red));
                }
                if check_bit(self.pieces[Piece::Knight as usize].0, bit) {
                    p = "♞";
                } else if check_bit(self.pieces[Piece::Bishop as usize].0, bit) {
                    p = "♝";
                } else if check_bit(self.pieces[Piece::Rook as usize].0, bit) {
                    p = "♜";
                } else if check_bit(self.pieces[Piece::King as usize].0, bit) {
                    p = "♚";
                } else if check_bit(self.pieces[Piece::Queen as usize].0, bit) {
                    p = "♛";
                } else if check_bit(self.pieces[Piece::Pawn as usize].0, bit) {
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
