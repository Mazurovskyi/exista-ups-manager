
use std::fmt::Display;
use std::{error::Error, ops::DerefMut};
use std::borrow::{BorrowMut, Borrow};
use std::ops::Deref;
use json::JsonValue;

use crate::json_patterns::JsonPattern;

use crate::modbus::{Modbus, msg::ModbusMsg};

mod ups_info;
mod battery_info;
mod battery_event;
pub mod requests_stack;

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
impl Display for Request{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}



pub trait RequestObject : MqttSending + Display{
    /// inserts data into Request object. 
    /// Returns Err() if something wrong with Modbus connection or event should be skipped.
    fn insert_data<'a>(&mut self, bus: &'a Modbus)->Result<(), Box<dyn Error + 'a>>;
}



/// describes methods to send Request over MQTT topic
pub trait MqttSending{
    fn serialize(&self)->String;
    fn topic(&self)->&str;
    fn qos(&self)->i32;
}



/// describes how to ask and parse data over Modbus.
trait ModbusData : RequestObject{
    fn get_modbus_data<'a>(&self, bus: &'a Modbus)->Result<Vec<ModbusMsg>, Box<dyn Error + 'a>>;

    /// returns "none", if ModbusMsg cannot be decoded. 
    fn parse_modbus_data(&self, raw_data: Vec<ModbusMsg>)->Vec<JsonValue>;
}



/// describes how to create Json object for each Request
trait JsonCreation : RequestObject{
    fn build_json()->JsonValue;
}
















