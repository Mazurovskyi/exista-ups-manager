
use json::JsonValue;

use std::iter::Iterator;
use std::iter::IntoIterator;
use std::vec::IntoIter;



/// trait provides method to insert data into Json-object
pub trait JsonPattern{
    //fn build(topic: &str)->Result<JsonValue, Box<dyn Error>>;
    fn fill(&mut self, values: Vec<JsonValue>);
    fn do_fill(&mut self, values: &mut IntoIter<JsonValue>);
}
impl JsonPattern for JsonValue{

    /*
    /// Takes the Json object from file, that may be filled by fill() method.
    /// Returns Error if the file path is not correct or it is not JSON-format.
    /// Returns Null if the supplied topic name exists. 
    fn build(topic: &str)->Result<JsonValue, Box<dyn Error>>{

        let path = env::var("JSON_PATTERNS")
            .unwrap_or(JSON_PATTERNS.to_owned());

        let content = fs::read_to_string(path)?;
        let mut json_pattern: JsonValue = json::parse(&content)?;
        
        Ok(json_pattern.remove(topic))
    }
    */

    fn fill(&mut self, values: Vec<JsonValue>) {
        let mut values = values.into_iter();
        self.do_fill(& mut values)
    }

    fn do_fill(&mut self, values: &mut IntoIter<JsonValue>){
        
        for (_name, el) in self.entries_mut(){
            
            if el.is_object(){
               el.do_fill(values);
            }
            else{
                let temp = values.next();
                if el.is_null(){
                    *el = temp.into()    //unwraped data or null
                }
            }
        }
    }
    
}






/*
 pub struct BatteryInfo{
    json_pattern: JsonValue,
    json_data: Option<Vec<JsonValue>>,
    topic: &'static str
}
impl BatteryInfo{
    pub fn build()->Result<Self, Box<dyn Error>>{
        let json_pattern = JsonValue::build(TOPIC_BATTERY_INFO_REP)?;
        //println!("BatteryInfo=================\n{}", json_pattern.pretty(4));
        Ok(Self{
            json_pattern: json_pattern,
            json_data: Some(Vec::new()),
            topic: TOPIC_BATTERY_INFO_REP
        })
    }
    fn insert(&mut self, index: usize, element: JsonValue){
        self.json_data.as_mut().unwrap().insert(index, element)
    }
    fn push(&mut self, element: JsonValue){
        self.json_data.as_mut().unwrap().push(element)
    }
    fn decode(msg: ModbusMsg, i: usize)->JsonValue{
        if i == 4 || i == 5 {
            ModbusMsg::registers_value_percent(msg.data())
        }
        else{
            ModbusMsg::registers_value(msg.data())
        } 
    }
}







pub struct UpsInfo{
    json_pattern: JsonValue,
    json_data: Option<Vec<JsonValue>>,
    topic: &'static str
}
impl UpsInfo{
    pub fn build()->Result<Self, Box<dyn Error>>{
        let json_pattern = JsonValue::build(TOPIC_UPS_INFO)?;
        //println!("UpsInfo=============\n{}", json_pattern.pretty(4));
        Ok(Self{
            json_pattern: json_pattern,
            json_data: Some(Vec::new()),
            topic: TOPIC_UPS_INFO
        })
    }
    fn insert(&mut self, index: usize, element: JsonValue){
        self.json_data.as_mut().unwrap().insert(index, element)
    }
    fn push(&mut self, element: JsonValue){
        self.json_data.as_mut().unwrap().push(element)
    }
    fn decode_fw_version(msg: ModbusMsg)->String{
        let msg = msg.data();
        let registers_value = ((msg[3] as u32) << 8) + (msg[4]as u32);

        //println!("Register`s value of FW version: {registers_value}");

        let main_vers = (registers_value - 0xA003) / 255;
        let sub_vers =  (registers_value - 0xA003) % 255;

        format!("{main_vers}.{sub_vers}")
    }
    fn decode_module_name(msg: ModbusMsg)->String{
        match msg.data()[4]{
            0=> HOURS_1.to_owned(),
            1=> HOURS_4.to_owned(),
            _=> HOURS_NA.to_owned()
        }
    }

}






pub struct BatteryEvent{
    json_pattern: JsonValue,
    json_data: Option<Vec<JsonValue>>,
    topic: &'static str,
    msg: [u8; 16]
}
impl BatteryEvent{
    pub fn build(msg: [u8; 16])->Result<Self, Box<dyn Error>>{
        let json_pattern = JsonValue::build(TOPIC_EVENT)?;
    
        Ok(Self{
            json_pattern,
            json_data: Some(Vec::new()),
            topic: TOPIC_EVENT,
            msg
        })
    }
    fn insert(&mut self, index: usize, element: JsonValue){
        self.json_data.as_mut().unwrap().insert(index, element)
    }
    fn push(&mut self, element: JsonValue){
        self.json_data.as_mut().unwrap().push(element)
    }
    fn decode(&mut self, msg: [u8; 16])->Result<i32, String>{

        let battery_event = ((msg[6] as u16) << 8) + (msg[7] as u16);   // msg[6..8]

        match battery_event.map(){
            DONT_FORWARD => Err(format!("Event should be skipped. Code: {battery_event}")),
            value => Ok(value)
        }
    }
}








pub trait Insertion : Display{
    fn fill<'a>(&'a mut self, bus: &'a Modbus)->Result<(), Box<dyn Error + '_>>;
    fn fill_with_data(&mut self);
    fn serialize(&self)->String;
    fn publish(&self, client: &MqttClient, timeout: Duration)->Result<(), paho_mqtt::Error>;
}

impl Insertion for BatteryInfo{

    fn fill<'a>(&'a mut self, bus: &'a Modbus)->Result<(), Box<dyn Error + '_>>{

        unsafe{
            let binding = APP_INFO.lock()?;
            let serial_numver = binding.get_serial_number();
            self.insert(0, serial_numver.into());
        }

        let com_status = bus.get_status();
        self.insert(1, com_status.into());
        

        for (i, request) in BATTERY_INFO_REQUEST.iter().enumerate(){
            let msg = ModbusMsg::from(&request[..], request.len());
            let feedback = bus.send(&msg)?;
            let value = Self::decode(feedback, i);
            self.push(value)
        }

        //println!("Instanse data is ready: {:?}", self.json_data);

        self.fill_with_data();

        Ok(())
    }
    fn fill_with_data(&mut self){
        self.json_pattern.fill(self.json_data.take().unwrap().into_iter())
    }
    fn serialize(&self)->String{
        self.json_pattern.dump()
    }
    fn publish(&self, client: &MqttClient, timeout: Duration)->Result<(), paho_mqtt::Error>{
        client.publish(self, timeout, self.topic)
    }
    
}

impl Insertion for UpsInfo{
    fn fill<'a>(&'a mut self, bus: &'a Modbus)->Result<(), Box<dyn Error + '_>> {
        
        
        let msg = ModbusMsg::from(&READ_MAX_AUTHONOMY_TIME[..], READ_MAX_AUTHONOMY_TIME.len());
        let module_name = bus.send(&msg)?;
        let module_name = Self::decode_module_name(module_name);



        let msg = ModbusMsg::from(&READ_FW_VERSION[..], READ_FW_VERSION.len());
        let fw_version = bus.send(&msg)?;
        let fw_version = Self::decode_fw_version(fw_version);

        let ups_serial_num = UPS_SERIAL_NUMBER.to_owned();

        [module_name, fw_version, ups_serial_num].into_iter()
            .for_each(|el|self.push(el.into()));

        //println!("Instanse data is ready: {:?}", self.json_data);

        self.fill_with_data();

        Ok(())

    }

    fn fill_with_data(&mut self){
        self.json_pattern.fill(self.json_data.take().unwrap().into_iter())
    }

    fn serialize(&self)->String{
        self.json_pattern.dump()
    }
    fn publish(&self, client: &MqttClient, timeout: Duration)->Result<(), paho_mqtt::Error>{
        client.publish(self, timeout, self.topic)
    }
}

impl Insertion for BatteryEvent{
    fn fill<'a>(&'a mut self, _bus: &'a Modbus)->Result<(), Box<dyn Error + '_>> {
        
        unsafe{
            let binding = APP_INFO.lock()?;
            let serial_numver = binding.get_serial_number();
            self.insert(0, serial_numver.into());
        }
        
        let date_time = Local::now().to_rfc3339();
        self.push(date_time.into());

        let battery_event = self.decode(self.msg)?;
        self.push(battery_event.into());

        // acBatterySwitchCounter and batteryMissingCounter == 0.
        
        self.fill_with_data();

        Ok(())
    }
    
    fn fill_with_data(&mut self){
        self.json_pattern.fill(self.json_data.take().unwrap().into_iter())
    }

    fn serialize(&self)->String{
        self.json_pattern.dump()
    }
    fn publish(&self, client: &MqttClient, timeout: Duration)->Result<(), paho_mqtt::Error>{
        client.publish(self, timeout, self.topic)
    }
}
*/
















