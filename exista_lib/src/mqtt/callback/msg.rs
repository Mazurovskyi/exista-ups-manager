use std::error::Error;

use paho_mqtt::Message;
use chrono::Local;

use crate::application::constants::{*, cube_serial_num::CubeSerialNumber};
use crate::requests::{*, requests_stack::RequestsStack};



pub trait Handler{
    /// handles incoming Mqtt message. 
    /// Returns Err() only if Request cannot be pushed into Requests handling queue
    fn handle(&self)->Result<String, String>;
    fn get_payload(&self, payload: &str)->Result<String, Box<dyn Error>>;
}

impl Handler for Message{
    fn handle(&self)->Result<String, String>{

        let time = Local::now().to_rfc3339();
        
        match self.topic(){
            TOPIC_BATTERY_INFO_REQ => {
                RequestsStack::push(Request::battery_info())?;
                Ok(format!("Received {TOPIC_BATTERY_INFO_REQ} at {time}."))
            },
            TOPIC_DEVICE_INFO => {
                let serial_number = self.get_payload("serialNumber")
                                       .unwrap_or("unknown".to_owned());

                CubeSerialNumber::set(serial_number);
                RequestsStack::push(Request::ups_info())?;
                
                Ok(format!("Received {TOPIC_DEVICE_INFO}: {time}"))
            },
            _=> Ok("Received mqtt message on unexpected topic".to_owned())
        }
    }

    fn get_payload(&self, payload: &str)->Result<String, Box<dyn Error>>{

        let json_obj = json::parse(&self.payload_str())?
            .remove(payload);
    
        let payload = json_obj.as_str().ok_or("Param is absent")?;
        Ok(payload.to_owned())
    }
}








