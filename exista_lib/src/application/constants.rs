use std::time::Duration;
use once_cell::sync::Lazy;
pub mod cube_serial_num;



pub const UPS_SERIAL_NUMBER: &str = "NA";
static mut CUBE_SERIAL_NUMBER: Lazy<String> = Lazy::new(||String::from("unknown"));



//----PATHS----
pub const LOG_FILE_PATH: &str = "/home/schindler/rust_exista/exista_log.txt";
pub const JSON_PATTERNS: &str = "/home/schindler/rust_exista/patterns.json";



//-----------------------MQTT_CONSTANTS-----------------------
pub const HOST: &str = "127.0.0.1:1883";
pub const CLIENT_ID: &str = "exista_ups_manager";
pub const MQTT_VERSION: u32 = 0; 
pub const KEEP_ALIVE: u64 = 60;
pub const QOS: i32 = 0; 
pub const DELIVERY_TIME: Duration = Duration::from_secs(1);

pub const TOPIC_BATTERY_INFO_REQ: &str = "gateway/batteryInfo.req";
pub const TOPIC_BATTERY_INFO_REP: &str = "gateway/batteryInfo.rep";
pub const TOPIC_DEVICE_INFO: &str = "gateway/deviceInfo";   // recive
pub const TOPIC_UPS_INFO: &str = "gateway/upsInfo";         // reply
pub const TOPIC_EVENT: &str = "gateway/event/battery";



//-----------------------MODBUS_CONSTANTS-----------------------
pub const PORT: &str = "/dev/ttyUPS";
pub const HEARTBEAT_FREQ: u64 = 60;

// Setting timeout to 10 ms allows to receive replies without timeout errors. 
// Because UPS delay consists about 5 ms. 
pub const TIMEOUT: u64 = 10;    

//com_status code
pub const CONNECT: u8 = 1;
pub const DISCONNECT: u8 = 2;

//----UPS_MODULE_NAME----
pub const HOURS_1: &str = "UPS1H";
pub const HOURS_4: &str = "UPS4H";
pub const HOURS_NA: &str = "unknown";



//-----------------------REQUESTS-----------------------
//----BATTERY_INFO----
pub const READ_DC_STATUS: [u16; 4] =      [0x11, 0x03, 0x17, 0x01];
pub const READ_BATTERY_STATUS: [u16; 4] = [0x11, 0x03, 0x00, 0x01];
pub const READ_VOLTAGE: [u16; 4] =        [0x11, 0x03, 0x04, 0x01];
pub const READ_CURRENT_VALUE: [u16; 4] =  [0x11, 0x03, 0x12, 0x01];
pub const READ_SOC: [u16; 4] =            [0x11, 0x03, 0x1C, 0x01];
pub const READ_SOH: [u16; 4] =            [0x11, 0x03, 0x1D, 0x01];
pub const READ_BACKUP_TIME: [u16; 4] =    [0x11, 0x03, 0x1B, 0x01]; //same as REMAIN_TIME

//----UPS_INFO----
pub const READ_MAX_AUTHONOMY_TIME: [u16; 4] = [0x11, 0x03, 0x20, 0x01];   //-> 0: "UPS1H", 1: "UPS4H" else: "unknown"
pub const READ_FW_VERSION: [u16; 4] =         [0x01, 0x03, 0x00, 0x01];

//----HEARTBEAT----
pub const HEARTBEAT: [u16; 4] =       [0x01, 0x06, 0x50, 0x00];

//----UNUSED----
const _READ_REMAIN_TIME: [u16; 4] =    [0x11, 0x03, 0x1A, 0x01];
const _READ_CHARGING_STATUS: [u16; 4]= [0x11, 0x03, 0x19, 0x01];
const _GET_SIGN: [u16; 4] =            [0x11, 0x03, 0x11, 0x01];
const _GET_TEMPERATURE: [u16; 4] =     [0x01, 0x03, 0x55, 0x01];
const _CUBE_POWER_RESET: [u16; 4] =    [0x01, 0x06, 0x1F, 0xAA55];



//----EVENTS----
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

