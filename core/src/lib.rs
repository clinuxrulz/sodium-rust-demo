#![allow(dead_code)]

extern crate rusty_frp;

pub use rusty_frp::sodium;
pub mod ecs;
pub mod drivers;
pub mod math;
mod app_ctx;

use app_ctx::AppCtx;

#[no_mangle]
pub extern "C" fn create_app_ctx() -> *mut AppCtx {
    Box::into_raw(Box::new(AppCtx::new()))
}

#[no_mangle]
pub extern "C" fn destroy_app_ctx(app_ctx: *mut AppCtx) {
    unsafe { Box::from_raw(app_ctx) };
}
