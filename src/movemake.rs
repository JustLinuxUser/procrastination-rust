use core::panic;

use crate::{
    attacks::{get_bishop_moves, get_rook_moves, B_PAWN_CAPS, KNIGHT_TABLE, W_PAWN_CAPS},
    moves::{Move, MoveFlags},
    state::{Color, State},
    utils::bb_to_idx,
};

impl State {
    pub fn in_check(&self) -> bool {
        let me;
        match self.color {
            Color::White => {
                me = self.white;
            }
            Color::Black => {
                me = self.black;
            }
        };
        let king = self.kings & me;
        return self.under_attack(king);
    }
    pub fn under_attack(&self, piece_bb: u64) -> bool {
        let enemy;
        let me;
        let all;
        match self.color {
            Color::White => {
                me = self.white;
                enemy = self.black
            }
            Color::Black => {
                me = self.black;
                enemy = self.white
            }
        };
        all = me | enemy;
        let piece_idx = bb_to_idx(piece_bb);

        let enemy_knights = self.knights & enemy;
        if KNIGHT_TABLE[piece_idx] & enemy_knights != 0 {
            return true;
        }

        let enemy_pawns = self.pawns & enemy;
        let caps = match self.color {
            Color::White => W_PAWN_CAPS[piece_idx],
            Color::Black => B_PAWN_CAPS[piece_idx],
        };
        if caps & enemy_pawns != 0 {
            return true;
        }

        let enemy_queens = self.queens & enemy;

        let enemy_bishops = self.bishops & enemy;
        let b_moves = get_bishop_moves(piece_bb, all);
        if b_moves & (enemy_bishops | enemy_queens) != 0 {
            return true;
        }

        let enemy_rooks = self.rooks & enemy;
        let b_moves = get_rook_moves(piece_bb, all);
        if b_moves & (enemy_rooks | enemy_queens) != 0 {
            return true;
        }

        false
    }
    pub fn make_move(&mut self, m: &Move) -> bool {
        let to_bb = m.to();
        let from_bb = m.from();
        let from = bb_to_idx(from_bb) as u64;
        let to = bb_to_idx(to_bb) as u64;

        //Move the color part of the piece
        match self.color {
            Color::White => {
                self.white &= !from_bb;
                self.black &= !to_bb;
                self.white |= to_bb;
            }
            Color::Black => {
                self.black &= !from_bb;
                self.white &= !to_bb;
                self.black |= to_bb;
            }
        }
        const CASTLE_RIGHTS: [u8; 64] = {
            let mut ret = [0xff; 64];
            ret[0] = 0b0111; //wq, rook
            ret[7] = 0b1011; //wk, rook
            ret[4] = 0b0011; //wq, king

            ret[56] = 0b1101; //bq, rook
            ret[63] = 0b1110; //bk, rook
            ret[60] = 0b1100; //bq, king
            ret
        };
        self.castle &= CASTLE_RIGHTS[from as usize] & CASTLE_RIGHTS[to as usize];

        self.pawns &= !from_bb;
        self.ep = 0;
        match m.flags() {
            MoveFlags::PawnMove => {
                self.pawns &= !from_bb;
                self.clear_square(to);
                self.pawns |= to_bb;
            }
            MoveFlags::PawnDoublePush => {
                self.pawns &= !from_bb;
                self.clear_square(to);
                self.pawns |= to_bb;
                let ep = match self.color {
                    Color::White => to - 8,
                    Color::Black => to + 8,
                };
                self.ep = ep as u8;
            }
            MoveFlags::EP => {
                self.pawns &= !from_bb;
                self.pawns |= to_bb;
                match self.color {
                    Color::White => {
                        self.pawns &= !(1 << (to - 8));
                        self.black &= !(1 << (to - 8));
                    }
                    Color::Black => {
                        self.pawns &= !(1 << (to + 8));
                        self.black &= !(1 << (to + 8));
                    }
                }
            }
            MoveFlags::PromoQueen => {
                self.pawns &= !from_bb;
                self.clear_square(to);
                self.queens |= to_bb;
            }
            MoveFlags::PromoRook => {
                self.pawns &= !from_bb;
                self.clear_square(to);
                self.rooks |= to_bb;
            }
            MoveFlags::PromoKnight => {
                self.pawns &= !from_bb;
                self.clear_square(to);
                self.knights |= to_bb;
            }
            MoveFlags::PromoBishop => {
                self.pawns &= !from_bb;
                self.clear_square(to);
                self.bishops |= to_bb;
            }
            MoveFlags::BishopMove => {
                self.bishops &= !from_bb;
                self.clear_square(to);
                self.bishops |= to_bb;
            }
            MoveFlags::KnightMove => {
                self.knights &= !from_bb;
                self.clear_square(to);
                self.knights |= to_bb;
            }
            MoveFlags::RookMove => {
                self.rooks &= !from_bb;
                self.clear_square(to);
                self.rooks |= to_bb;
            }
            MoveFlags::QueenMove => {
                self.queens &= !from_bb;
                self.clear_square(to);
                self.queens |= to_bb;
            }
            MoveFlags::KingMove => {
                self.kings &= !from_bb;
                self.clear_square(to);
                self.kings |= to_bb;
            }
            MoveFlags::Castle => {
                //from, to
                const CASTLE_ROOK_POS: [(u64, u64); 64] = {
                    let mut ret = [(0, 0); 64];

                    ret[2] = (1 << 0, 1 << 3); // Wq
                    ret[6] = (1 << 7, 1 << 5); // Wk

                    ret[58] = (1 << 56, 1 << 59); // Bq
                    ret[62] = (1 << 63, 1 << 61); // Bk
                    ret
                };
                let (rook_from, rook_to) = CASTLE_ROOK_POS[to as usize];

                // NOTE: No need to clear the square in case of a castle move
                self.kings &= !from_bb;
                self.kings |= to_bb;

                if self.color == Color::White {
                    self.white &= !rook_from;
                    self.white |= rook_to;
                } else {
                    self.black &= !rook_from;
                    self.black |= rook_to;
                }
                self.rooks &= !rook_from;
                self.rooks |= rook_to;
            }
        }
        if self.in_check() {
            return false;
        }
        match self.color {
            Color::White => self.color = Color::Black,
            Color::Black => self.color = Color::White,
        }
        return true;
    }
    pub fn make_move_list(&mut self, moves: &str) {
        if moves == "" {
            return;
        }
        let req_moves = moves.trim().split_whitespace().collect::<Vec<_>>();
        for req_m in req_moves {
            let moves = self.pseudo_legal_moves(false);
            let mut found = false;
            for m in moves {
                if m.to_text() == req_m {
                    if self.make_move(&m) {
                        found = true;
                    }
                    break;
                }
            }
            if !found {
                panic!("Move {req_m} not found!");
            }
        }
    }
}
