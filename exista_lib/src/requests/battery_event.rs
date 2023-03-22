
use std::borrow::{Borrow, BorrowMut};
use std::fmt::Display;

use json::JsonValue;
use chrono::Local;


use crate::requests::ModbusMsg;
use crate::application::constants::*;
use crate::application::loger::Log;

use super::*;

pub struct BatteryEvent{
    json: JsonValue,
    event: ModbusMsg,
    publish_topic: &'static str,
    qos: i32
}
impl BatteryEvent{
    pub fn new(event: ModbusMsg)->Self{
        Self {
            json: Self::build_json(), 
            event,
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
    fn event(&self)->&ModbusMsg{
        self.event.borrow()
    }

    // decoding operations
    fn decode(&self, event: &ModbusMsg)->Option<i32>{

        let msg  = event.data();

        let battery_event = ((*msg.get(6)? as u16) << 8) + (*msg.get(7)? as u16);   // msg[6..8]

        match battery_event.map(){
            DONT_FORWARD => {
                Log::write(format!("Event should be skipped. Code: {battery_event}").as_str());
                None
            },
            value => Some(value)
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
}

impl RequestObject for BatteryEvent{
    fn fill_with_data<'a>(&mut self, _bus: &'a Modbus)->Result<(), Box<dyn Error + 'a>>{

        let mut parsed_data: Vec<JsonValue> = Vec::new();

        unsafe{
            let binding = APP_INFO.lock()?;
            let serial_numver: JsonValue = binding.get_serial_number().into();
            parsed_data.push(serial_numver.into());
        }
        
        let date_time: JsonValue = Local::now().to_rfc3339().into();

        let battery_event: JsonValue = self.decode(self.event()).into();

        let ac_battery_switch_counter = 0;
        let battery_missing_counter = 0;

        parsed_data.extend_from_slice(&[date_time, battery_event, battery_missing_counter.into(), ac_battery_switch_counter.into()]);
        
        self.json_mut().fill(parsed_data);

        Ok(())
    }
}

impl Display for BatteryEvent{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         write!(f, "(BatteryEvent:,\n{})", self.json().pretty(4))
    }
}
