use std::{
    collections::HashMap,
    env,
    io::stdin,
    time::{Duration, Instant},
};

use crate::{
    board::Board,
    core_types::Color::{Black, White},
    fen::load_fen,
    search::Search,
};
pub struct Game {
    pub b: Option<Board>,
}
impl Game {
    pub fn uci_loop(&mut self) {
        loop {
            let mut command = String::new();

            stdin().read_line(&mut command).unwrap();
            command = command.trim().to_owned();

            let mut cmd = HashMap::<String, String>::new();
            let mut munch = "".to_owned();
            let mut fen_munching = false;
            let mut move_munching = false;
            let mut go_munching = false;
            let mut munching = false;

            for word in command.split_whitespace() {
                eprintln!("word {word}");
                if fen_munching && word == "moves" {
                    fen_munching = false;
                    munching = false;
                    cmd.insert("fen".to_owned(), munch.trim().to_owned());
                    munch.clear();
                }
                if munching {
                    munch += word;
                    munch += " ";
                    continue;
                }
                if word == "uci" {
                    println!("id name the Rust Procrastination");
                    println!("id author Andrii Dokhniak");
                    println!("uciok");
                } else if word == "isready" {
                    println!("readyok");
                } else if command == "test" {
                    let mut b = Board::new();
                    load_fen(&mut b, "1k6/2pp4/1p2p3/4q3/1P2B3/2PK1R2/8/8 w - - 0 46").unwrap();
                    let res = Search::search(Duration::from_secs(2), b);
                    println!("bestmove {}", res.as_text());
                } else if word == "position" {
                    cmd.insert("position".to_owned(), "true".to_owned());
                } else if word == "startpos" {
                    cmd.insert("startpos".to_owned(), "true".to_owned());
                } else if word == "fen" {
                    fen_munching = true;
                    munching = true;
                } else if word == "moves" {
                    move_munching = true;
                    munching = true;
                } else if word == "go" {
                    go_munching = true;
                    munching = true;
                } else if word == "quit" {
                    return;
                }
            }
            if move_munching {
                cmd.insert("moves".to_owned(), munch.trim().to_owned());
            }
            if fen_munching {
                cmd.insert("fen".to_owned(), munch.trim().to_owned());
            }
            if go_munching {
                cmd.insert("go".to_owned(), "true".to_owned());
                eprintln!("munch: {munch}");
                let mut i = munch.split_whitespace();
                while let (Some(k), Some(v)) = (i.next(), i.next()) {
                    cmd.insert(k.to_owned(), v.to_owned());
                }
            }
            if cmd.get("position").is_some() {
                let mut b = Board::new();
                load_fen(
                    &mut b,
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                )
                .unwrap();
                if let Some(fen) = cmd.get("fen") {
                    load_fen(&mut b, fen).unwrap();
                }

                if let Some(moves) = cmd.get("moves") {
                    b.make_move_list(moves);
                }
                self.b = Some(b);
            }
            if cmd.get("go").is_some() {
                let color = self.b.unwrap().color;
                let time: i32 = match color {
                    White => cmd.get("wtime").unwrap().parse().unwrap(),
                    Black => cmd.get("btime").unwrap().parse().unwrap(),
                };
                let time = time.abs(); // in case the time is negative
                let time = time.abs();
                let inc: i32 = match color {
                    White => cmd.get("winc").unwrap_or(&"0".to_owned()).parse().unwrap(),
                    Black => cmd.get("binc").unwrap_or(&"0".to_owned()).parse().unwrap(),
                };
                assert!(inc >= 0);
                let tc = time / 20 + inc / 2;
                let s = Search::search(Duration::from_millis(tc as u64), self.b.unwrap());
                println!("bestmove {}", s.as_text());
            }
        }
    }
}
fn perft(board: &mut Board, depth: u8) -> u64 {
    let mut count = 0;
    if depth == 0 {
        return 1;
    }

    let moves = board.gen_pseudo_legal();
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
fn perftree() -> u64 {
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
    let mut board = Board::new();
    let time_start = Instant::now();
    load_fen(&mut board, fen).unwrap();

    board.make_move_list(moves);
    board.eprint_board();
    if depth == 0 {
        return 1;
    }

    let mut total_count = 0;

    let moves = board.gen_pseudo_legal();
    eprintln!("Number of moves: {}", moves.moves.len());
    for m in moves {
        let mut new_board = board;
        if new_board.make_move(&m) {
            let count = perft(&mut new_board, depth - 1);
            println!("{} {count}", m.as_text());
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
