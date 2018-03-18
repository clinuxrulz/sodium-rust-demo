#![allow(dead_code)]

extern crate rusty_frp;

pub use rusty_frp::sodium;
pub mod ecs;
pub mod drivers;
pub mod math;
mod app_ctx;

use app_ctx::AppCtx;
use drivers::DiagDriver;
use drivers::DisplayDriver;
use math::Vec2;

use std::ffi::CString;
use std::os::raw::c_char;

extern {

    fn log_(text: *const c_char, len: u32);

    fn display_add_circle(x: f64, y: f64, r: f64) -> u32;
    fn display_move_circle(id: u32, x: f64, y: f64);
    fn display_remove(id: u32);
}

fn log<S: ToString>(text: S) {
    let text = text.to_string();
    let len = text.len() as u32;
    let text2 = CString::new(text).unwrap();
    unsafe {
        log_(text2.as_ptr(), len);
    }
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

struct DiagViaImports {}

impl DiagDriver for DiagViaImports {
    fn log_string(&mut self, msg: String) {
        log(msg);
    }
}

#[no_mangle]
pub extern "C" fn create_app_ctx() -> *mut AppCtx {
    log("App Created");
    Box::into_raw(Box::new(AppCtx::new(DiagViaImports {}, DisplayViaImports {})))
}

#[no_mangle]
pub extern "C" fn destroy_app_ctx(app_ctx: *mut AppCtx) {
    unsafe { Box::from_raw(app_ctx) };
}

#[no_mangle]
pub extern "C" fn test(app_ctx: &mut AppCtx) {
    //let mut display_driver = app_ctx.display_driver.borrow_mut();
    //let circle = display_driver.add_circle(50.0, 50.0, 10.0);
    //display_driver.move_circle(circle, 100.0, 100.0);
}

// Input Driver Exports

#[no_mangle]
pub extern "C" fn input_tick(app_ctx: &mut AppCtx, ms: f64) {
    app_ctx.input_tick(ms);
}

#[no_mangle]
pub extern "C" fn input_key_pressed(app_ctx: &mut AppCtx, key_code: u32) {
    app_ctx.input_key_pressed(key_code);
}

#[no_mangle]
pub extern "C" fn input_key_released(app_ctx: &mut AppCtx, key_code: u32) {
    app_ctx.input_key_released(key_code);
}

#[no_mangle]
pub extern "C" fn input_mouse_moved(app_ctx: &mut AppCtx, x: f64, y: f64) {
    app_ctx.input_mouse_moved(Vec2::of(x, y));
}

#[no_mangle]
pub extern "C" fn input_mouse_exited(app_ctx: &mut AppCtx) {
    app_ctx.input_mouse_exited();
}

#[no_mangle]
pub extern "C" fn input_mouse_pressed(app_ctx: &mut AppCtx, buttons: u32) {
    app_ctx.input_mouse_pressed(buttons);
}

#[no_mangle]
pub extern "C" fn input_mouse_released(app_ctx: &mut AppCtx, buttons: u32) {
    app_ctx.input_mouse_released(buttons);
}
