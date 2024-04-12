use crate::state::{Color, State};

pub fn load_fen(b: &mut State, fen: &str) -> Result<(), ()> {
    //r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -
    b.white = 0;
    b.black = 0;
    b.knights = 0;
    b.bishops = 0;
    b.kings = 0;
    b.rooks = 0;
    b.pawns = 0;
    b.queens = 0;
    let fen_parts = fen.trim().split_whitespace().collect::<Vec<_>>();
    if fen_parts.len() < 4 {
        load_fen(
            b,
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -",
        )
        .expect("The default fen is always possible to load");
        return Err(());
    }
    let piece_position = fen_parts[0].as_bytes();
    let mut i = 0;
    for c in piece_position {
        let p = 56 - (i / 8) * 8 + i % 8;
        match *c as char {
            '1'..='8' => {
                let n = c - b'0';
                i += n - 1;
            }
            'p' | 'P' => b.pawns |= 1 << p,
            'r' | 'R' => b.rooks |= 1 << p,
            'b' | 'B' => b.bishops |= 1 << p,
            'n' | 'N' => b.knights |= 1 << p,
            'k' | 'K' => b.kings |= 1 << p,
            'q' | 'Q' => b.queens |= 1 << p,
            _ => continue,
        }
        if c.is_ascii_lowercase() {
            b.black |= 1 << p;
        } else if c.is_ascii_uppercase() {
            b.white |= 1 << p;
        }
        i += 1;
    }

    match fen_parts[1].chars().next().unwrap() {
        'w' => b.color = Color::White,
        'b' => b.color = Color::Black,
        _ => (),
    }

    b.castle = 0;
    if fen_parts[2].contains('K') {
        b.castle |= 0b1;
    }
    if fen_parts[2].contains('Q') {
        b.castle |= 0b10;
    }
    if fen_parts[2].contains('k') {
        b.castle |= 0b100;
    }
    if fen_parts[2].contains('q') {
        b.castle |= 0b1000;
    }
    if fen_parts[3] == "-" {
        b.ep = 0;
    } else {
        let s_ep = fen_parts[3].chars().collect::<Vec<_>>();
        let ep = (s_ep[1] as u8 - b'1') * 8 + (s_ep[0] as u8 - b'a');
        b.ep = ep;
    }
    Ok(())
}
