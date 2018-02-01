#![allow(dead_code)]

extern crate rusty_frp;

pub use rusty_frp::sodium;
use sodium::SodiumCtx;
pub mod ecs;
pub mod drivers;
pub mod math;

fn main() {
    let mut sodium_ctx = SodiumCtx::new();
    let cs = sodium_ctx.new_cell_sink(1);
    cs.send(&2);
    cs.send(&3);
}
