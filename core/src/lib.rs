#![allow(dead_code)]

extern crate rusty_frp;

pub use rusty_frp::sodium;
pub mod ecs;
pub mod drivers;
pub mod math;
mod app_ctx;

use app_ctx::AppCtx;

extern {
    fn display_add_circle(x: u32, y: u32, r: u32) -> u32;
    fn display_remove(id: u32);
}

#[no_mangle]
pub extern "C" fn create_app_ctx() -> *mut AppCtx {
    Box::into_raw(Box::new(AppCtx::new()))
}

#[no_mangle]
pub extern "C" fn destroy_app_ctx(app_ctx: *mut AppCtx) {
    unsafe { Box::from_raw(app_ctx) };
}

#[no_mangle]
pub extern "C" fn test() {
    unsafe {
        display_add_circle(50, 50, 10);
    }
}
