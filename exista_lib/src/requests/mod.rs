
use std::{error::Error, ops::DerefMut};
use std::borrow::{BorrowMut, Borrow};
use std::ops::Deref;

use json::JsonValue;

use crate::{json_patterns::{Fill_old, BatteryInfo, BatteryEvent}};

use crate::modbus::{Modbus, msg::ModbusMsg};

mod ups_info;
use crate::requests::ups_info::UpsInfo;



pub struct Request(Box<dyn Fill>);
impl Request{
    pub fn ups_info()->Self{
        Self(Box::new(UpsInfo::new()))
    }
}
impl Deref for Request{
    type Target = Box<dyn Fill>;
    fn deref(&self) -> &Self::Target {
        self.0.borrow()
    }
}
impl DerefMut for Request{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.borrow_mut()
    }
}


pub trait Fill{
    fn fill_with_data<'a>(&mut self, bus: &'a Modbus)->Result<(), Box<dyn Error + 'a>>;
}


trait GetModbusData{
    fn get_modbus_data<'a>(&self, bus: &'a Modbus)->Result<Vec<ModbusMsg>, Box<dyn Error + 'a>>;
    fn parse_modbus_data(&self, raw_data: Vec<ModbusMsg>)->Vec<JsonValue>;
}


trait CreateJson{
    fn build_json()->JsonValue;
}















impl CreateJson for BatteryInfo{
    fn build_json()->JsonValue {
        object! {
            serialNumber: null,
            batteryInfo: {
                comStatus: null,
                dcStatus: null,
                batteryStatus: null,
                batteryVoltage: null,
                batteryCurrent: null,
                soc: null,
                soh: null,
                timeLeft: null
            }
        }
    }
}
impl CreateJson for BatteryEvent{
    fn build_json()->JsonValue {
        object! {
            serialNumber: null,
            eventTime: null,
            batteryEvent: null,
            batteryMissingCounter: null,
            acBatterySwitchCounter: null
        }
    }
}