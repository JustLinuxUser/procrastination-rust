mod attacks;
mod fen;
mod hist;
mod movemake;
mod moves;
mod state;
mod uci;
mod utils;

use attacks::init_magics;
use uci::uci_loop;

fn main() {
    init_magics(false);
    uci_loop();
}
