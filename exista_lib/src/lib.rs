#![feature(file_create_new)]
#![feature(is_some_and)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

#[macro_use]
extern crate json;

mod modbus;
mod mqtt;
mod requests;
pub mod json_patterns;
pub mod application;



use chrono::Local;
pub fn time()->String{
    Local::now().to_rfc3339()
}












