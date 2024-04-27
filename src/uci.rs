use std::{env, io::stdin, time::Instant};

use crate::{fen::load_fen, state::State};

pub fn uci_loop() {
    let mut command = String::new();

    stdin().read_line(&mut command).unwrap();
    command = command.trim().to_owned();

    let _command_split: Vec<_> = command.split_whitespace().collect();
    if command == "uci" {
        println!("uciok")
    } else if command == "isready" {
        println!("readyok")
    } else if command == "test" {
        perftree(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ",
            6,
            "",
        );
    } else if command == "perftree" {
        //./your-perft.sh "$depth" "$fen" "$moves"
        let args = env::args().collect::<Vec<_>>();
        if args.len() != 3 && args.len() != 4 {
            panic!("When running in perftree mode, provide the correct number of arguments!!");
        }
        let depth: u8 = args[1].parse().unwrap();
        let fen = &args[2];
        eprintln!("fen: {fen}");
        let moves: &str = if args.len() == 4 { &args[3] } else { "" };
        eprintln!("moves: {moves}");
        perftree(fen, depth, moves);
    }
}

fn perft(board: &mut State, depth: u8) -> u64 {
    let mut count = 0;
    if depth == 0 {
        return 1;
    }

    let moves = board.pseudo_legal_moves(false);
    for m in moves {
        let mut new_board = *board;
        if new_board.make_move(&m) {
            if depth == 1 {
                count += 1;
            } else {
                count += perft(&mut new_board, depth - 1);
            }
        }
    }
    count
}
fn perftree(fen: &str, depth: u8, moves: &str) -> u64 {
    let mut board = State::new();
    let time_start = Instant::now();
    load_fen(&mut board, fen).unwrap();
    board.eprint_board();
    board.make_move_list(moves);
    if depth == 0 {
        return 1;
    }

    let mut total_count = 0;

    let moves = board.pseudo_legal_moves(false);
    eprintln!("Number of moves: {}", moves.moves.len());
    for m in moves {
        let mut new_board = board;
        if new_board.make_move(&m) {
            let count = perft(&mut new_board, depth - 1);
            eprintln!("{} {count}, {:?}", m.as_text(), m.flags());
            total_count += count;
        }
    }
    println!("\n{total_count}");
    let time_end = Instant::now();
    let diff = time_end - time_start;
    if diff.as_millis() != 0 {
        eprintln!("NPS: {}", total_count as u128 / diff.as_millis() * 1000);
    } else {
        eprintln!("NPS: inf");
    }
    eprintln!("{:#b}", board.castle);
    total_count
}
