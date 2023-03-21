
use std::borrow::{Borrow, BorrowMut};

use json::JsonValue;

use crate::requests::ModbusMsg;
use crate::application::constants::*;
use super::*;


pub struct UpsInfo{
    json: JsonValue,
    requests_list: Vec<ModbusMsg>
}
impl UpsInfo{
    pub fn new()->Self{
        Self { 
            json: Self::build_json(), 
            requests_list: Self::build_request_list()
        }
    }


    fn json_mut(&mut self)->& mut JsonValue{
        self.json.borrow_mut()
    }
    fn requests_list(&self)-> &Vec<ModbusMsg>{
        self.requests_list.borrow()
    }
    fn build_request_list()->Vec<ModbusMsg>{
        [READ_MAX_AUTHONOMY_TIME, READ_FW_VERSION]
            .iter()
            .map(|msg|ModbusMsg::from(&msg[..], msg.len()))
            .collect()
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


impl CreateJson for UpsInfo{
    fn build_json()->JsonValue {
        object! {
            moduleName: null,
            firmwareVersion: null,
            upsSerialNumber: null
        }
    }
}


impl Fill for UpsInfo{
    fn fill_with_data<'a>(&mut self, bus: &'a Modbus)->Result<(), Box<dyn Error + 'a>>{

        let raw_data = self.get_modbus_data(bus)?;
        let mut parsed_data = self.parse_modbus_data(raw_data);

        parsed_data.push(UPS_SERIAL_NUMBER.into());

        self.json_mut().fill(parsed_data);

        Ok(())
    }
}


impl GetModbusData for UpsInfo{

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





