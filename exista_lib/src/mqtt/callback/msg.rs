use std::error::Error;

use paho_mqtt::Message;
use chrono::Local;

use crate::application::constants::{*, cube_serial_num::CubeSerialNumber};
use crate::requests::{*, requests_stack::RequestsStack};



pub trait Handler{
    /// handles incoming Mqtt message. 
    /// Returns Err() only if Request cannot be pushed into Requests handling queue
    fn handle(self)->Result<String, String>;
}
trait GetPayload{
    fn get_payload(self, payload: &str)->Result<String, Box<dyn Error>>;
}

impl Handler for Option<Message>{
    fn handle(self)->Result<String, String>{

        // empty message is not a reason to fail
        let msg = match self.ok_or_else(|| "Received an empty mqtt message"){
            Ok(msg)=>msg,
            Err(err) => return Ok(err.to_owned())
        };

        match msg.topic(){
            TOPIC_BATTERY_INFO_REQ => {
                let report = format!("Received {TOPIC_BATTERY_INFO_REQ}: {}", Local::now().to_rfc3339());
                RequestsStack::push(Request::battery_info())?;
                Ok(report)
            },
            TOPIC_DEVICE_INFO => {
                let report = format!("Received {TOPIC_DEVICE_INFO}: {}", Local::now().to_rfc3339());

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


impl GetPayload for Message {
    fn get_payload(self, payload: &str)->Result<String, Box<dyn Error>>{

        let json_obj = json::parse(&self.payload_str())?
            .remove(payload);
    
        let payload = json_obj.as_str().ok_or("Param is absent")?;
        Ok(payload.to_owned())
    }
}








