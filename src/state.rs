use crate::{
    attacks::{
        get_bishop_moves, get_rook_moves, KING_TABLE, KNIGHT_TABLE, PAWN_CAPS, PAWN_DOUBLE_PUSHES,
        PAWN_PUSHES,
    },
    core_types::{Color, Piece, SquareIdx, BB},
    moves::{Move, MoveFlags, MoveList},
};
use termion::color;
#[derive(Copy, Clone)]
pub struct State {
    pub side: [BB; 2],
    pub pieces: [BB; 6],
    pub ep: SquareIdx,
    pub ply: u8,
    pub color: Color,
    pub castle: u8, // Wk, Wq, Bk, Bq
}

impl State {
    pub fn new() -> Self {
        {
            Self {
                color: Color::White,
                side: [BB(0); 2],
                pieces: [BB(0); 6],
                ep: SquareIdx::new(),
                ply: 0,
                castle: 0xff,
            }
        }
    }
    pub fn get_my_piece(&self, t: Piece) -> BB {
        self.pieces[t as usize] & self.side[self.color as usize]
    }
    pub fn get_enemy_piece(&self, t: Piece) -> BB {
        self.pieces[t as usize] & self.side[self.color.opposite() as usize]
    }
    pub fn clear_piece_bb(&mut self, sq: BB, piece: Piece) {
        self.side[self.color as usize] &= !sq;
        self.pieces[piece as usize] &= !sq;
    }
    pub fn set_piece_bb(&mut self, sq: BB, piece: Piece) {
        self.side[self.color as usize] |= sq;
        self.pieces[piece as usize] |= sq;
    }

    pub fn pseudo_legal_moves(&self, _captures_only: bool) -> MoveList {
        let mut mlist = MoveList::new();
        let all_pieces = self.side[0] | self.side[1];

        let enemy_color = self.color.opposite();

        let my_color_idx = self.color as usize;
        let enemy_color_idx = enemy_color as usize;

        let my_pieces = self.side[my_color_idx];
        let enemy_pieces = self.side[enemy_color_idx];

        let pawns = self.get_my_piece(Piece::Pawn);
        for pawn in pawns {
            fn promo(mlist: &mut MoveList, from: BB, to: BB) -> bool {
                const PROMO_MASK: BB = BB(0xff000000000000ff);
                if (to & PROMO_MASK).empty() {
                    return false;
                }
                mlist.push_move(from, to, MoveFlags::PromoRook);
                mlist.push_move(from, to, MoveFlags::PromoQueen);
                mlist.push_move(from, to, MoveFlags::PromoKnight);
                mlist.push_move(from, to, MoveFlags::PromoBishop);
                true
            }
            let from_idx = SquareIdx::from(pawn).0 as usize;
            let pushes = BB(PAWN_PUSHES[my_color_idx][from_idx]) & !all_pieces;
            for push in pushes {
                if !promo(&mut mlist, pawn, push) {
                    mlist.push_move(pawn, push, MoveFlags::PawnMove);
                    let double_push = PAWN_DOUBLE_PUSHES[my_color_idx][from_idx];
                    if !(BB(double_push) & !all_pieces).empty() {
                        mlist.push_move(pawn, BB(double_push), MoveFlags::PawnDoublePush);
                    }
                }
            }
            let caps = BB(PAWN_CAPS[my_color_idx][from_idx]) & enemy_pieces;
            for cap in caps {
                if !promo(&mut mlist, pawn, cap) {
                    mlist.push_move(pawn, cap, MoveFlags::PawnMove);
                }
            }
            if self.ep.valid() {
                let ep = BB(PAWN_CAPS[my_color_idx][from_idx]) & BB::from(self.ep);
                if !ep.empty() {
                    mlist.push_move(pawn, BB::from(self.ep), MoveFlags::EP);
                }
            }
        }
        let knights = self.get_my_piece(Piece::Knight);
        for p in knights {
            let p_idx = SquareIdx::from(p).0 as usize;
            let moves = BB(KNIGHT_TABLE[p_idx]) & !my_pieces;
            for m in moves {
                mlist.push_move(p, m, MoveFlags::KnightMove);
            }
        }
        let bishops = self.get_my_piece(Piece::Bishop);
        for p in bishops {
            let moves = get_bishop_moves(p, all_pieces) & !my_pieces;
            for m in moves {
                mlist.push_move(p, m, MoveFlags::BishopMove);
            }
        }
        let rooks = self.get_my_piece(Piece::Rook);
        for p in rooks {
            let moves = get_rook_moves(p, all_pieces) & !my_pieces;
            for m in moves {
                mlist.push_move(p, m, MoveFlags::RookMove);
            }
        }
        let queens = self.get_my_piece(Piece::Queen);
        for p in queens {
            let mut moves = get_rook_moves(p, all_pieces) & !my_pieces;
            moves |= get_bishop_moves(p, all_pieces) & !my_pieces;
            for m in moves {
                mlist.push_move(p, m, MoveFlags::QueenMove);
            }
        }
        let kings = self.get_my_piece(Piece::King);
        for p in kings {
            let moves: BB = BB(KING_TABLE[SquareIdx::from(p).0 as usize]) & !my_pieces;
            for m in moves {
                mlist.push_move(p, m, MoveFlags::KingMove);
            }
            if self.in_check() {
                break;
            }
            match self.color {
                Color::White => {
                    if self.castle & 0b1000 != 0 {
                        // Wq
                        let mask_wq = 0xe.into();
                        if (all_pieces & mask_wq).empty()
                            && !self.under_attack(BB(1 << 2))
                            && !self.under_attack(BB(1 << 3))
                        {
                            mlist.push_move(p, BB(1 << 2), MoveFlags::Castle);
                        }
                    }
                    if self.castle & 0b0100 != 0 {
                        //Wk
                        let mask_wk = 0x60;
                        if (all_pieces & mask_wk.into()).empty()
                            && !self.under_attack(BB(1 << 5))
                            && !self.under_attack(BB(1 << 6))
                        {
                            mlist.push_move(p, BB(1 << 6), MoveFlags::Castle);
                        }
                    }
                }
                Color::Black => {
                    if self.castle & 0b0010 != 0 {
                        //Bq
                        let mask_bq = 0xe00000000000000;
                        if (all_pieces & mask_bq.into()).empty()
                            && !self.under_attack(BB(1 << 58))
                            && !self.under_attack(BB(1 << 59))
                        {
                            mlist.push_move(p, BB(1 << 58), MoveFlags::Castle);
                        }
                    }
                    if self.castle & 0b0001 != 0 {
                        // Bk
                        let mask_bk = 0x6000000000000000;
                        if (all_pieces & mask_bk.into()).empty()
                            && !self.under_attack(BB(1 << 61))
                            && !self.under_attack(BB(1 << 62))
                        {
                            mlist.push_move(p, BB(1 << 62), MoveFlags::Castle);
                        }
                    }
                }
            }
        }
        mlist
    }

    #[allow(dead_code)]
    pub fn eprint_board_move(&self, m: Move) {
        fn check_bit(bb: u64, bit: u8) -> bool {
            bb & 1 << bit != 0
        }
        let from = m.to_idx().0;
        let to = m.to_idx().0;
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
    pub fn remove_piece_bit(&mut self, bit: usize) {
        for i in 0..6 {
            self.pieces[i] &= !BB(1 << bit);
        }
    }
    pub fn move_piece(&mut self, p: Piece, from_idx: usize, to_idx: usize) {
        // remove from
        self.pieces[p as usize] &= !BB(1 << from_idx);

        // remove to
        self.remove_piece_bit(to_idx);

        // put to
        self.pieces[p as usize] |= BB(1 << to_idx);
    }
}
