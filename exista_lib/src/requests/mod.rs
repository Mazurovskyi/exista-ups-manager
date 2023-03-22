
use std::{error::Error, ops::DerefMut};
use std::borrow::{BorrowMut, Borrow};
use std::ops::Deref;

use json::JsonValue;

use crate::json_patterns::JsonPattern;

use crate::modbus::{Modbus, msg::ModbusMsg};

mod ups_info;
mod battery_info;
mod battery_event;
use crate::requests::{ups_info::UpsInfo, battery_info::BatteryInfo, battery_event::BatteryEvent};



pub struct Request(Box<dyn RequestObject>);
impl Request{
    pub fn ups_info()->Self{
        Self(Box::new(UpsInfo::new()))
    }
    pub fn battery_info()->Self{
        Self(Box::new(BatteryInfo::new()))
    }
    pub fn battery_event(event: ModbusMsg)->Self{
        Self(Box::new(BatteryEvent::new(event)))
    }
}

impl Deref for Request{
    type Target = Box<dyn RequestObject>;
    fn deref(&self) -> &Self::Target {
        self.0.borrow()
    }
}

impl DerefMut for Request{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.borrow_mut()
    }
}



pub trait RequestObject : MqttSending{
    fn fill_with_data<'a>(&mut self, bus: &'a Modbus)->Result<(), Box<dyn Error + 'a>>;
}

pub trait MqttSending{
    fn serialize(&self)->String;
    fn topic(&self)->&str;
    fn qos(&self)->i32;
}

trait ModbusData : RequestObject{
    fn get_modbus_data<'a>(&self, bus: &'a Modbus)->Result<Vec<ModbusMsg>, Box<dyn Error + 'a>>;
    fn parse_modbus_data(&self, raw_data: Vec<ModbusMsg>)->Vec<JsonValue>;
}

trait JsonCreation : RequestObject{
    fn build_json()->JsonValue;
}
















