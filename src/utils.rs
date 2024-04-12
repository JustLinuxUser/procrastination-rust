use termion::color;

use crate::state;

pub fn check_bit(bb: u64, bit: u8) -> bool {
    bb & 1 << bit != 0
}

pub const fn bb_to_idx(bb: u64) -> usize {
    bb.trailing_zeros() as usize
}

pub fn pop_lsb(bb: &mut u64) -> u64 {
    let bit = 1 << bb.trailing_zeros();
    *bb &= !bit;
    bit
}

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
