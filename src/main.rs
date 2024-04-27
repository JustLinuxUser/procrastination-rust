mod attacks;
mod core_types;
mod fen;
mod movemake;
mod moves;
mod state;
mod uci;
mod utils;

use attacks::init_magics;
use uci::uci_loop;

fn main() {
    init_magics(false);
    eprintln!("finished init magics");
    uci_loop();
}
