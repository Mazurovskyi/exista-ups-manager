use std::process;

use json::JsonValue;
use paho_mqtt::Message;
use crate::application::constants::*;
use crate::modbus::Modbus;
use std::error::Error;
use crate::requests::*;
use crate::json_patterns::*;
use std::borrow::Cow;
use std::sync::Arc;
use std::sync::mpsc::Sender;
use chrono::Local;
use std::sync::Mutex;

pub trait Handler{
    fn handle(self, stack: Arc<Mutex<Sender<(&str, [u8; 16])>>>)->Result<String, Box<dyn Error + '_>>;
}
impl Handler for Option<Message>{
    fn handle(self, tx: Arc<Mutex<Sender<(&str, [u8; 16])>>>)->Result<String, Box<dyn Error + '_>>{

        let msg = self.ok_or_else(|| "Received an empty mqtt message")?;

        match msg.topic(){
            TOPIC_BATTERY_INFO_REQ => {
                //let instanse = BatteryInfo::build()?;
                    
                //println!("instanse has created is handle");
                //println!("instance:\n {}", instanse);

                let report = format!("Received gateway/batteryInfo.req: {}", Local::now().to_rfc3339());

                match tx.lock(){
                    Ok(guard)=> guard.send((TOPIC_BATTERY_INFO_REQ, [0;16]))?,
                    Err(err) => return Err(err.to_string().into())
                };
                
                
                Ok(report)
            },
            TOPIC_DEVICE_INFO => {

                let report = format!("Received gateway/deviceInfo: {}", Local::now().to_rfc3339());

                let serial_number = msg.get_payload("serialNumber")
                                       .unwrap_or("unknown".to_owned());

                unsafe{APP_INFO.lock()?.set_serial_number(serial_number)};
    
                //let instanse = UpsInfo::build().unwrap();

                //println!("instanse has created is handle");
                // println!("instance:\n {}", instanse);

                match tx.lock(){
                    Ok(guard)=> guard.send((TOPIC_DEVICE_INFO, [0;16]))?,
                    Err(err) => return Err(err.to_string().into())
                };

                Ok(report)
            },
            _=> Ok("Received mqtt message on unexpected topic".to_owned())
        }
    }
}

trait GetPayload{
    fn get_payload(self, param: &str)->Result<String, Box<dyn Error>>;
}
impl GetPayload for Message {
    fn get_payload(self, param: &str)->Result<String, Box<dyn Error>>{

        let mut json_obj = json::parse(&self.payload_str())?;
        /*
        let mut json_obj = match self.payload_str(){
            Cow::Borrowed(payload) => json::parse(payload)?,
            Cow::Owned(payload) => {
                println!("The contents of the payload are not valid UTF-8 data!");
                json::parse(&payload)?
            }
        };
         */
        

        //println!("\nWe have recieved a JSON_OBJ:");
        //println!("{}\n", json_obj.pretty(4));
    
        let json_value = json_obj.remove(param);

        let payload = json_value.as_str().ok_or("Param is absent")?;
        
        Ok(payload.to_owned())
    }
}








