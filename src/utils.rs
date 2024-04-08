use crate::State;
use termion::color;

pub mod bitboard_utils {
    use std::u64;
    use termion::color;
    pub fn check_bit(bb: u64, bit: u8) -> bool {
        return bb & 1 << bit != 0;
    }
    pub fn print(bb: u64) {
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
}
impl State {
    pub fn print(&self) {
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
                if bitboard_utils::check_bit(self.white, bit) {
                    print!("{}", color::Fg(color::Rgb(50, 30, 200)));
                } else if bitboard_utils::check_bit(self.black, bit) {
                    print!("{}", color::Fg(color::Rgb(0, 0, 0)));
                } else {
                    print!("{}", color::Fg(color::Red));
                }
                if bitboard_utils::check_bit(self.knights, bit) {
                    p = "♞";
                } else if bitboard_utils::check_bit(self.bishops, bit) {
                    p = "♝";
                } else if bitboard_utils::check_bit(self.rooks, bit) {
                    p = "♜";
                } else if bitboard_utils::check_bit(self.kings, bit) {
                    p = "♚";
                } else if bitboard_utils::check_bit(self.queens, bit) {
                    p = "♛";
                } else if bitboard_utils::check_bit(self.pawns, bit) {
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
}
