mod attacks;
mod board;
mod core_types;
mod eval;
mod fen;
mod movemake;
mod moves;
mod search;
mod uci;
mod utils;

use attacks::init_magics;
use uci::Game;

fn main() {
    init_magics(false);
    eprintln!("finished init magics");
    let mut g = Game { b: None };
    g.uci_loop();
}
