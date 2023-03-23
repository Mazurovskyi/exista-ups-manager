use std::process;

use json::JsonValue;
use paho_mqtt::Message;
use crate::application::constants::*;
use crate::modbus::Modbus;
use crate::requests::requests_stack::RequestsStack;
use std::error::Error;
use crate::requests::*;
use crate::json_patterns::*;
use std::borrow::Cow;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use chrono::Local;
use std::sync::Mutex;

pub trait Handler{
    fn handle(self)->Result<String, Box<dyn Error>>;
}
impl Handler for Option<Message>{
    fn handle(self)->Result<String, Box<dyn Error>>{

        let msg = self.ok_or_else(|| "Received an empty mqtt message")?;

        match msg.topic(){
            TOPIC_BATTERY_INFO_REQ => {
                let report = format!("Received gateway/batteryInfo.req: {}", Local::now().to_rfc3339());

                RequestsStack::push(Request::battery_info())?;
                
                Ok(report)
            },
            TOPIC_DEVICE_INFO => {
                let report = format!("Received gateway/deviceInfo: {}", Local::now().to_rfc3339());

                let serial_number = msg.get_payload("serialNumber")
                                       .unwrap_or("unknown".to_owned());

                CubeSerialNumber::set(serial_number);
    
                RequestsStack::push(Request::ups_info())?;
                
                Ok(report)  
            },
            _=> Ok("Received mqtt message on unexpected topic".to_owned())
        }
    }
}

pub trait GetPayload{
    fn get_payload(self, param: &str)->Result<String, Box<dyn Error>>;
}
impl GetPayload for Message {
    fn get_payload(self, param: &str)->Result<String, Box<dyn Error>>{

        let mut json_obj = json::parse(&self.payload_str())?;
    
        let json_value = json_obj.remove(param);

        let payload = json_value.as_str().ok_or("Param is absent")?;
        
        Ok(payload.to_owned())
    }
}








