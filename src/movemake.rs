use core::panic;

use crate::{
    board::Board,
    core_types::{Color, Piece, SquareIdx, BB},
    moves::{Move, MoveFlags},
};

impl Board {
    pub fn in_check(&self) -> bool {
        let me = self.side[self.color as usize];
        let king = self.pieces[Piece::King as usize] & me;
        assert_ne!(king.0, 0);
        self.under_attack(king)
    }
    pub fn under_attack(&self, piece: BB) -> bool {
        let knight_attacks = self.get_attacks(Piece::Knight, piece);
        if !(knight_attacks & self.get_enemy_piece(Piece::Knight)).empty() {
            return true;
        }

        let bishop_attacks = self.get_attacks(Piece::Bishop, piece);
        if !(bishop_attacks
            & (self.get_enemy_piece(Piece::Bishop) | self.get_enemy_piece(Piece::Queen)))
        .empty()
        {
            return true;
        }
        let rook_attacks = self.get_attacks(Piece::Rook, piece);
        if !(rook_attacks
            & (self.get_enemy_piece(Piece::Rook) | self.get_enemy_piece(Piece::Queen)))
        .empty()
        {
            return true;
        }

        let enemy_pawns = self.get_enemy_piece(Piece::Pawn);
        let caps = self.get_attacks(Piece::Pawn, piece);
        if !(caps & enemy_pawns).empty() {
            return true;
        }

        let enemy_king = self.get_enemy_piece(Piece::King);
        let caps = self.get_attacks(Piece::King, piece);
        if !(caps & enemy_king).empty() {
            return true;
        }
        false
    }
    pub fn make_move(&mut self, m: &Move) -> bool {
        let to_bb = m.get_to();
        let from_bb = m.get_from();
        let to = SquareIdx::from(to_bb).0 as usize;
        let from = SquareIdx::from(from_bb).0 as usize;
        let my_color_idx = self.color as usize;
        let opposite_color_idx = my_color_idx ^ 1;

        //Move the color part of the piece
        self.side[my_color_idx] &= !from_bb;
        self.side[opposite_color_idx] &= !to_bb;
        self.side[my_color_idx] |= to_bb;
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
        self.castle &= CASTLE_RIGHTS[from] & CASTLE_RIGHTS[to];

        self.ep = SquareIdx(255);
        match m.flags() {
            MoveFlags::PawnMove => {
                self.move_piece(Piece::Pawn, from, to);
            }
            MoveFlags::PawnDoublePush => {
                self.move_piece(Piece::Pawn, from, to);
                let ep = SquareIdx(match self.color {
                    Color::White => (to - 8) as u8,
                    Color::Black => (to + 8) as u8,
                });
                self.ep = ep;
            }
            MoveFlags::EP => {
                self.move_piece(Piece::Pawn, from, to);
                match self.color {
                    Color::White => {
                        self.pieces[Piece::Pawn as usize] &= !BB(1 << (to - 8));
                        self.side[opposite_color_idx] &= !BB(1 << (to - 8));
                    }
                    Color::Black => {
                        self.pieces[Piece::Pawn as usize] &= !BB(1 << (to + 8));
                        self.side[opposite_color_idx] &= !BB(1 << (to + 8));
                    }
                }
            }
            MoveFlags::PromoQueen => {
                self.move_piece(Piece::Pawn, from, to);
                self.remove_piece_bit(to);
                self.pieces[Piece::Queen as usize] |= to_bb;
            }
            MoveFlags::PromoRook => {
                self.move_piece(Piece::Pawn, from, to);
                self.remove_piece_bit(to);
                self.pieces[Piece::Rook as usize] |= to_bb;
            }
            MoveFlags::PromoKnight => {
                self.move_piece(Piece::Pawn, from, to);
                self.remove_piece_bit(to);
                self.pieces[Piece::Knight as usize] |= to_bb;
            }
            MoveFlags::PromoBishop => {
                self.move_piece(Piece::Pawn, from, to);
                self.remove_piece_bit(to);
                self.pieces[Piece::Bishop as usize] |= to_bb;
            }
            MoveFlags::BishopMove => {
                self.move_piece(Piece::Bishop, from, to);
            }
            MoveFlags::KnightMove => {
                self.move_piece(Piece::Knight, from, to);
            }
            MoveFlags::RookMove => {
                self.move_piece(Piece::Rook, from, to);
            }
            MoveFlags::QueenMove => {
                self.move_piece(Piece::Queen, from, to);
            }
            MoveFlags::KingMove => {
                self.move_piece(Piece::King, from, to);
            }
            MoveFlags::Castle => {
                //from, to
                const CASTLE_ROOK_POS: [(BB, BB); 64] = {
                    let mut ret = [(BB(0), BB(0)); 64];

                    ret[2] = (BB(1 << 0), BB(1 << 3)); // Wq
                    ret[6] = (BB(1 << 7), BB(1 << 5)); // Wk

                    ret[58] = (BB(1 << 56), BB(1 << 59)); // Bq
                    ret[62] = (BB(1 << 63), BB(1 << 61)); // Bk
                    ret
                };
                let (rook_from, rook_to) = CASTLE_ROOK_POS[to];

                self.clear_piece_bb(from_bb, Piece::King);
                self.set_piece_bb(to_bb, Piece::King);

                self.clear_piece_bb(rook_from, Piece::Rook);
                self.set_piece_bb(rook_to, Piece::Rook);
            }
        }
        if self.in_check() {
            return false;
        }
        self.color = self.color.opposite();
        true
    }
    pub fn make_move_list(&mut self, moves: &str) {
        if moves.is_empty() {
            return;
        }
        let req_moves = moves.split_whitespace().collect::<Vec<_>>();
        for req_m in req_moves {
            let moves = self.gen_pseudo_legal();
            let mut found = false;
            for m in moves {
                if m.as_text() == req_m {
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
