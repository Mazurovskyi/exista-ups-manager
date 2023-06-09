
use std::borrow::{Borrow, BorrowMut};
use std::fmt::Display;

use json::JsonValue;
use chrono::Local;

use crate::application::constants::*;
use crate::application::constants::cube_serial_num::CubeSerialNumber;
use crate::requests::ModbusMsg;

mod map;
use map::Map;

use super::*;

pub struct BatteryEvent{
    json: JsonValue,
    msg: ModbusMsg,
    event_time: String,
    publish_topic: &'static str,
    qos: i32
}
impl BatteryEvent{
    pub fn new(msg: ModbusMsg)->Self{
        Self {
            json: Self::build_json(), 
            msg,
            event_time: Local::now().to_rfc3339(),
            publish_topic: TOPIC_EVENT,
            qos: 0
        }
    }

    fn json(&self)->&JsonValue{
        self.json.borrow()
    }
    fn json_mut(&mut self)->& mut JsonValue{
        self.json.borrow_mut()
    }
    fn msg(&self)->&ModbusMsg{
        self.msg.borrow()
    }
    fn event_code(&self)->Option<u16>{
        let data = self.msg().data();
        Some(((*data.get(6)? as u16) << 8) + (*data.get(7)? as u16))   // msg[6..8]
    }
    fn event_time(&self)->&str{
        self.event_time.borrow()
    }

    // decoding operations
    fn decode(&self, _event: &ModbusMsg)->Result<i32, String>{

        let event_code = self.event_code().ok_or(format!("Event message is incomplete!"))?;

        match event_code.map(){
            DONT_FORWARD => {
                Err(format!("Event should be skipped. Code: {event_code}"))
            },
            value => Ok(value)
        }
    }
}

impl JsonCreation for BatteryEvent{
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

impl MqttSending for BatteryEvent{
    fn serialize(&self)->String{
        self.json.dump()
    }
    fn topic(&self)->&str{
        self.publish_topic
    }
    fn qos(&self)->i32{
        self.qos
    }
    fn bat_ic_low(&self)->bool{
        if self.event_code() == Some(BATT_IC_LOW) {
            true
        }
        else{
            false
        }
    }
}

impl RequestObject for BatteryEvent{
    fn insert_data<'a>(&mut self, _bus: &'a Modbus)->Result<(), Box<dyn Error + 'a>>{

        let serial_number: JsonValue = CubeSerialNumber::get().into();
        let event_time: JsonValue = self.event_time().into();

        let battery_event: JsonValue = self.decode(self.msg())?.into();

        // should be "0" 
        let ac_battery_switch_counter: JsonValue = 0.into();
        let battery_missing_counter: JsonValue = 0.into();

        let battery_event = Vec::from(
            [serial_number,
            event_time,
            battery_event,
            battery_missing_counter, 
            ac_battery_switch_counter]);
        
        self.json_mut().assign(battery_event);
        Ok(())
    }
}

impl Display for BatteryEvent{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         write!(f, "BatteryEvent:,\n{}", self.json().pretty(4))
    }
}
