
use std::borrow::{Borrow, BorrowMut};
use std::fmt::Display;

use json::JsonValue;

use crate::requests::ModbusMsg;
use crate::application::constants::*;
use super::*;


pub struct UpsInfo{
    json: JsonValue,
    modbus_requests: Vec<ModbusMsg>,
    ups_serial_number: String,
    publish_topic: &'static str,
    qos: i32
}
impl UpsInfo{
    pub fn new()->Self{
        Self { 
            json: Self::build_json(), 
            modbus_requests: Self::build_request_list(),
            ups_serial_number: String::from(UPS_SERIAL_NUMBER),
            publish_topic: TOPIC_UPS_INFO,
            qos: 0
        }
    }

    fn json(&self)->&JsonValue{
        self.json.borrow()
    }
    fn json_mut(&mut self)->& mut JsonValue{
        self.json.borrow_mut()
    }
    fn requests_list(&self)-> &Vec<ModbusMsg>{
        self.modbus_requests.borrow()
    }
    fn build_request_list()->Vec<ModbusMsg>{
        [READ_MAX_AUTHONOMY_TIME, READ_FW_VERSION]
            .iter()
            .map(|msg|ModbusMsg::from(&msg[..], msg.len()))
            .collect()
    }
    fn ups_serial_number(&self)->&str{
        self.ups_serial_number.borrow()
    }
    

    // decoding operations 
    fn get_module_name(&self, msg: ModbusMsg)->Option<&str>{
        match *msg.data().get(4)?{
            0=> Some(HOURS_1),
            1=> Some(HOURS_4),
            _=> Some(HOURS_NA)
        }
    }
    fn get_fw_version(&self, msg: ModbusMsg)->Option<String>{
        let msg = msg.data();

        let registers_value = ((*msg.get(3)? as u32) << 8) + (*msg.get(4)? as u32);

        let main_vers = (registers_value - 0xA003) / 255;
        let sub_vers =  (registers_value - 0xA003) % 255;

        Some(format!("{main_vers}.{sub_vers}"))
    }
}


impl JsonCreation for UpsInfo{
    fn build_json()->JsonValue {
        object! {
            moduleName: null,
            firmwareVersion: null,
            upsSerialNumber: null
        }
    }
}

impl MqttSending for UpsInfo{
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

impl RequestObject for UpsInfo{
    fn insert_data<'a>(&mut self, bus: &'a Modbus)->Result<(), Box<dyn Error + 'a>>{

        let raw_data = self.get_modbus_data(bus)?;
        let mut parsed_data = self.parse_modbus_data(raw_data);

        parsed_data.push(self.ups_serial_number().into());

        self.json_mut().assign(parsed_data);

        Ok(())
    }
}


impl ModbusData for UpsInfo{

    fn get_modbus_data<'a>(&self, bus: &'a Modbus)->Result<Vec<ModbusMsg>, Box<dyn Error + 'a>>{
        
        let mut modbus_replies = Vec::new();
        
        for msg in self.requests_list(){
            modbus_replies.push(bus.send(msg)?)
        };
        Ok(modbus_replies)
    }
    
    fn parse_modbus_data(&self, raw_data: Vec<ModbusMsg>)->Vec<JsonValue>{

        let mut iter = raw_data.into_iter();

        let module_name: JsonValue = self.get_module_name(iter.next().unwrap()).into();

        let fw_version: JsonValue =  self.get_fw_version(iter.next().unwrap()).into();

        Vec::from([module_name, fw_version])
    }
}


impl Display for UpsInfo{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         write!(f, "(UPSInfo:,\n{})", self.json().pretty(4))
    }
}


