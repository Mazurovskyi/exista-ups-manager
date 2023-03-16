#![feature(file_create_new)]

#[macro_use]
extern crate json;

use log::{info, warn, error, LevelFilter};

mod modbus;
mod mqtt;
mod requests;
pub mod json_patterns;
pub mod application;













