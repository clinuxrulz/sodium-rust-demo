extern crate rusty_frp;

pub use rusty_frp::sodium;
use sodium::SodiumCtx;
pub mod math;
pub mod ecs;

fn main() {
    let mut sodium_ctx = SodiumCtx::new();
    let cs = sodium_ctx.new_cell_sink(1);
    cs.send(&2);
    cs.send(&3);
}
