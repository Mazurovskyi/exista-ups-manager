use std::{sync::{Mutex, RwLock}, borrow::Borrow, time::Duration};
use once_cell::sync::Lazy;


//use std::error::Error;
//use std_semaphore::Semaphore;

use std::sync::Arc;
pub struct AppInfo{
    serial_number: String,
    ups_serial_number: String,
}
impl AppInfo{
    fn default()->Self{
        AppInfo{
            serial_number: String::default(),
            ups_serial_number: "NA".to_string(),
        }
    }
    pub fn set_serial_number(&mut self, serial_number: String){
        self.serial_number = serial_number
    }
    pub fn get_serial_number(&self)->&str{
        
        self.serial_number.borrow()
        
    }
}


/*
use std::sync::Arc;
pub struct Stack{
    status: Arc<Mutex<Semaphore>>,
    data: Arc<Mutex<Vec<u8>>>
}
impl Stack{

    pub fn new()->Self{
        Stack{
            status: Arc::new(Mutex::new(Semaphore::new(0))),
            data: Arc::new(Mutex::new(Vec::new()))
        }
    }

    ///clones current instanse
    pub fn clone(&self)->Self{
        Stack{
            status: Arc::clone(&self.status),
            data: Arc::clone(&self.data)
        }
    }

    /// push the new Insertion instanse into stack and increments the internal count of the semaphore.
    pub fn push(&mut self, instanse: u8){
        self.data.lock().unwrap().push(instanse);
        println!("semaphore release +1");
        self.status.lock().unwrap().release();
    }

    /// blocking the current thread until the internal count of the semaphore is at least 1.
    pub fn pull(&mut self)->Option<u8>{
        self.status.lock().unwrap().acquire();
        println!("semaphore gert access -1");
        self.data.lock().unwrap().pop()
    }
}
*/



//pub static mut STACK: Lazy<Stack> = Lazy::new(||Stack::new());
pub static mut APP_INFO: Lazy<Mutex<AppInfo>> = Lazy::new(||Mutex::new(AppInfo::default()));




pub const LOG_FILE_PATH: &str = "/home/schindler/rust_exista/exista_log.txt";
pub const JSON_PATTERNS: &str = "/home/schindler/rust_exista/patterns.json";

// MQTT constants
pub const TOPIC_BATTERY_INFO_REQ: &str = "gateway/batteryInfo.req";
pub const TOPIC_BATTERY_INFO_REP: &str = "gateway/batteryInfo.rep";
pub const TOPIC_DEVICE_INFO: &str = "gateway/deviceInfo";   // recive
pub const TOPIC_UPS_INFO: &str = "gateway/upsInfo";         // reply
pub const TOPIC_EVENT: &str = "gateway/event/battery";
pub const SUBSCRIBE_TOPICS: [&str;2] = [TOPIC_BATTERY_INFO_REQ, TOPIC_DEVICE_INFO];
pub const DELIVERY_TIME: Duration = Duration::from_secs(1);

pub const QOS:   &[i32]=     &[0, 0];  
pub const MQTT_VERSION: u32 = 0;
pub const CLIENT_ID: &str = "exista_ups_manager";
pub const HOST: &str = "127.0.0.1:1883";
pub const KEEP_ALIVE: u64 = 60;

//Modbus constants
pub const PORT: &str = "/dev/ttyUPS";

// UPS delay consists about 5 ms. Setting timeout to 10 ms allows to receive replies without timeout errors.
pub const TIMEOUT: u64 = 10;    



///UPS com status
pub struct ComStatus(Arc<RwLock<Connection>>);
pub enum Connection{
    Connect,
    Disconnect
}
impl ComStatus{
    pub fn set_connect(&mut self){
        *self.0.write().unwrap() = Connection::Connect;
    }
    pub fn set_disconnect(&mut self){
        *self.0.write().unwrap() = Connection::Disconnect;
    }
    pub fn clone(&self)->Self{
        ComStatus(Arc::clone(&self.0))
    }
    pub fn is_connect(&self)->bool{
        if let Connection::Connect = *self.0.read().unwrap(){
            return true
        }
        false
    }
    pub fn get_status(&self)->u8{
        if self.is_connect(){
            CONNECT
        }
        else{
            DISCONNECT
        }
    }
}

impl Default for ComStatus{
    fn default() -> Self {
        Self(Arc::new(RwLock::new(Connection::Disconnect)))
    }
}

// status representation
const CONNECT: u8 = 1;
const DISCONNECT: u8 = 2;









//----SERIAL_NUM----
//const SERIAL_NUMBER: &str = "xxx"; //unknown

//----BATTERY_INFO----

pub const READ_DC_STATUS: [u16; 4] =      [0x11, 0x03, 0x17, 0x01];
pub const READ_BATTERY_STATUS: [u16; 4] = [0x11, 0x03, 0x00, 0x01];
pub const READ_VOLTAGE: [u16; 4] =        [0x11, 0x03, 0x04, 0x01];
pub const READ_CURRENT_VALUE: [u16; 4] =  [0x11, 0x03, 0x12, 0x01];
pub const READ_SOC: [u16; 4] =            [0x11, 0x03, 0x1C, 0x01];
pub const READ_SOH: [u16; 4] =            [0x11, 0x03, 0x1D, 0x01];
pub const READ_BACKUP_TIME: [u16; 4] =    [0x11, 0x03, 0x1B, 0x01]; //same as REMAIN_TIME

//----BATTERY_EVENT----

//----UPS_INFO----
pub const READ_MAX_AUTHONOMY_TIME: [u16; 4] = [0x11, 0x03, 0x20, 0x01];   //-> 0: "UPS1H", 1: "UPS4H" else: "unknown"
pub const READ_FW_VERSION: [u16; 4] =     [0x01, 0x03, 0x00, 0x01];
pub const UPS_SERIAL_NUMBER: &str = "NA";
pub const HOURS_1: &str = "UPS1H";
pub const HOURS_4: &str = "UPS4H";
pub const HOURS_NA: &str = "unknown";

//----else----
const _READ_REMAIN_TIME: [u16; 4] =    [0x11, 0x03, 0x1A, 0x01];
const _READ_CHARGING_STATUS: [u16; 4]= [0x11, 0x03, 0x19, 0x01];
const _GET_SIGN: [u16; 4] =            [0x11, 0x03, 0x11, 0x01];
const _GET_TEMPERATURE: [u16; 4] =     [0x01, 0x03, 0x55, 0x01];

const _CUBE_POWER_RESET: [u16; 4] =    [0x01, 0x06, 0x1F, 0xAA55];
pub const HEARTBEAT: [u16; 4] =       [0x01, 0x06, 0x50, 0x00];
pub const HEARTBEAT_FREQ: u64 = 60;



pub const BATTERY_INFO_REQUEST: &[[u16;4]] = &[READ_DC_STATUS, READ_BATTERY_STATUS, READ_VOLTAGE,
READ_CURRENT_VALUE, READ_SOC, READ_SOH, READ_BACKUP_TIME];























pub const BATT_IC_OK: u16 = 0x1400;  //Battery normal
pub const BATT_IC_SUPPLY_BYDC: u16 = 0x1500;  //supply from DC
pub const BATT_IC_SUPPLY_BYBATT: u16 = 0x1600;  //supply from battery
pub const BATT_IC_CHARGING: u16 = 0x1700;  //charging
pub const BATT_IC_LOW: u16 = 0x1800;  //battery capacity low
pub const BATT_IC_DISCHARGED: u16 = 0x1900; // battery discharged
pub const BATT_IC_MISSING: u16 = 0x1A01;  //battery missing dummy message
pub const BATT_IC_DEFECT: u16 = 0x1B00;  //battery is defect
pub const BATT_IC_AGED: u16 = 0x1C00;  //battery is aged
pub const BATT_IC_SHAKE: u16 = 0x1D00; // unstable conn. dummy message
pub const BATT_IC_CHARGED: u16 = 0x1E00;  // battery is charged
pub const BATT_IC_REBOOT: u16 = 0x1F05;  //reboot due to insufficient capacity dummy message
pub const BATT_IC_CHARGING_NO_CHANGE: u16 = 0x2000;  // charging no change
pub const BATT_IC_CHARG_OVER_CURRENT: u16 = 0x2100;  // over current fault
pub const BATT_IC_OVER_VLOT: u16= 0x2200;  // over voltage fault
pub const BATT_IC_OVER_TEMPERATURE: u16 = 0x2300;  // over temperature fault
pub const BATT_IC_LOW_TEMPERATURE: u16 = 0x2400;  // low temperature fault
pub const BATT_IC_EQUAGL_CHARG_TOOLONG: u16 = 0x2500;  // aver. charging too long
pub const BATT_IC_DEFECT_DISCHARG_OVER_CURR: u16 = 0x2600;  // over current discharging fail

pub const DONT_FORWARD: i32 = 10;

pub trait Map{
    fn map(self)->i32;
}
impl Map for u16{
    fn map(self)->i32{
        match self{
            BATT_IC_OK => 7,                    // BATT_IC_OK
            BATT_IC_SUPPLY_BYDC => 2,           // BATT_IC_SUPPLY_BYDC
            BATT_IC_SUPPLY_BYBATT => 1,         // BATT_IC_SUPPLY_BYBATT
            BATT_IC_CHARGING => 9,              // BATT_IC_CHARGING
            BATT_IC_LOW => 6,                   // BATT_IC_LOW 
            BATT_IC_DISCHARGED => 8,            // BATT_IC_DISCHARGED
            BATT_IC_MISSING => 3,               // BATT_IC_MISSING
            BATT_IC_DEFECT => 5,                // BATT_IC_DEFECT
            BATT_IC_AGED => 4,                  // BATT_IC_AGED
            BATT_IC_SHAKE => 5,                 // BATT_IC_DEFECT
            BATT_IC_CHARGED => 9,               // BATT_IC_CHARGING
            BATT_IC_REBOOT => DONT_FORWARD,     // DONT_FORWARD!
            BATT_IC_CHARGING_NO_CHANGE => 9,    // BATT_IC_CHARGING
            BATT_IC_CHARG_OVER_CURRENT => 5,    // BATT_IC_DEFECT
            BATT_IC_OVER_VLOT => 5,             // BATT_IC_DEFECT
            BATT_IC_OVER_TEMPERATURE => DONT_FORWARD,   // DONT_FORWARD!
            BATT_IC_LOW_TEMPERATURE => DONT_FORWARD,    // DONT_FORWARD!
            BATT_IC_EQUAGL_CHARG_TOOLONG => 4,          // BATT_IC_AGED  
            BATT_IC_DEFECT_DISCHARG_OVER_CURR => 8,     // BATT_IC_DISCHARGED
            _=> DONT_FORWARD                            // DONT_FORWARD!
        }
    }
}