use std::borrow::Borrow;
use once_cell::sync::Lazy;
use super::*;

/// representation of cube serial number
pub struct CubeSerialNumber(Lazy<String>);

impl CubeSerialNumber {
    pub fn set(serial_num: String){
        unsafe{
            *CUBE_SERIAL_NUMBER = serial_num;
        }
    }
    pub fn get()->&'static str{
        unsafe{
            CUBE_SERIAL_NUMBER.borrow()
        }
    }
}