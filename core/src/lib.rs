#![allow(dead_code)]

extern crate rusty_frp;

pub use rusty_frp::sodium;
pub mod ecs;
pub mod drivers;
pub mod math;
mod app_ctx;

use app_ctx::AppCtx;
use drivers::DisplayDriver;

extern {
    fn display_add_circle(x: f64, y: f64, r: f64) -> u32;
    fn display_move_circle(id: u32, x: f64, y: f64);
    fn display_remove(id: u32);
}

struct DisplayViaImports {}

impl DisplayDriver for DisplayViaImports {
    fn add_circle(&mut self, x: f64, y: f64, r: f64) -> u32 {
        unsafe {
            display_add_circle(x, y, r)
        }
    }
    fn move_circle(&mut self, id: u32, x: f64, y: f64) {
        unsafe {
            display_move_circle(id, x, y);
        }
    }
    fn remove(&mut self, id: u32) {
        unsafe {
            display_remove(id);
        }
    }
}

#[no_mangle]
pub extern "C" fn create_app_ctx() -> *mut AppCtx {
    Box::into_raw(Box::new(AppCtx::new(DisplayViaImports {})))
}

#[no_mangle]
pub extern "C" fn destroy_app_ctx(app_ctx: *mut AppCtx) {
    unsafe { Box::from_raw(app_ctx) };
}

#[no_mangle]
pub extern "C" fn test(app_ctx: &mut AppCtx) {
    let display_driver = &mut app_ctx.display_driver;
    let circle = display_driver.add_circle(50.0, 50.0, 10.0);
    display_driver.move_circle(circle, 100.0, 100.0);
}
