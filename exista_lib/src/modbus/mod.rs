extern crate serial;
use std::borrow::{BorrowMut, Borrow};
use std::error::Error;
use std::io::{Write, Read};
use std::ops::{Deref, DerefMut};

use serial:: {SystemPort, prelude::SerialPort};
use std::time::Duration;

use std::sync::{Arc, Mutex, MutexGuard};
use std::io::ErrorKind;
pub mod msg;
use msg::ModbusMsg;
use crate::application::loger::Log;
use crate::requests::Request;
use crate::requests::requests_stack::RequestsStack;

use std::thread::{self, JoinHandle};
use chrono::Local;
use crate::application::constants::*;

use std::sync::mpsc::Sender;

use std::io;



pub struct ModbusPort(Arc<Mutex<SystemPort>>);

impl ModbusPort{
    fn open(port: &str)->Result<Self, io::Error>{
        let port = serial::open(port)?;
        Ok(ModbusPort(Arc::new(Mutex::new(port))))   
    }
    fn config(self)->Result<Self, io::Error>{
    
        self.lock().unwrap()
            .reconfigure(&|port_config|{

                port_config.set_baud_rate(serial::Baud115200)?;
                port_config.set_char_size(serial::Bits8);
                port_config.set_parity(serial::ParityNone);
                port_config.set_stop_bits(serial::Stop1);
                port_config.set_flow_control(serial::FlowNone);
    
                Ok(())
            }
        )?;
    
        Ok(self)
    }
    fn timeout(self, timeout: u64)->Result<Self, serial::Error>{
        self.lock().unwrap().set_timeout(Duration::from_millis(timeout))?;
        Ok(self)
    }
    fn clone(&self)->Self{
        ModbusPort(Arc::clone(&self.0))
    }
}

impl Deref for ModbusPort{

    type Target = Arc<Mutex<SystemPort>>;

    fn deref(&self) -> &Self::Target {
        self.0.borrow()
    }
}



pub struct Modbus{
    port: ModbusPort,
    status: ComStatus
}
impl Modbus{
    /// creates and configure the Modbus port.
    pub fn config(port: &str, timeout: u64)->Result<Self, Box<dyn Error>>{

        let bus = Modbus{
            port: ModbusPort::open(port)?.config()?.timeout(timeout)?,
            status: ComStatus::default()
        };

       Ok(bus)
    }

    /// running modbus services: listening the port and running modbus timer.
    pub fn run(&self)->(JoinHandle<()>, JoinHandle<()>){

        let heartbeat = self.clone().create_heartbet();
        let listener = self.clone().create_listener();

        let heartbeat = thread::spawn(heartbeat);
        let listener = thread::spawn(listener);

        (heartbeat, listener)
    }

    /// athomary operation to send data into modbus and return a reply immediately.
    pub fn send(&self, msg: &ModbusMsg)->Result<ModbusMsg, Box<dyn Error + '_>>{

        Log::write(format!("sending modbus message: {:?}", msg.data()).as_str());

        let mut port_guard = self.port().lock()?;

        Self::sending(&mut port_guard, msg.data())?;
        let returned_msg = Self::reading(&mut port_guard)?;

        // drop(port_guard)

        // Modbus RTU requires min 2 ms delay.
        thread::sleep(Duration::from_millis(2));

        Ok(returned_msg)
    }

    /// try to read the port once time
    pub fn read_once(&self, feedback: &mut [u8])->Result<ModbusMsg, Box<dyn Error+ '_>>{
        let bytes_count = self.port().lock()?.read(feedback)?;
        Ok(ModbusMsg::from(feedback, bytes_count))
    }

    pub fn clone(&self)->Self{
        Modbus{
            port: self.port.clone(),
            status: self.status.clone()
        }
    }

    pub fn port(&self)->&ModbusPort{
        self.port.borrow()
    }



    // private API

    fn create_heartbet(mut self)->impl FnOnce() + Send + 'static{

        let heartbeat_msg = ModbusMsg::from(&HEARTBEAT[..], HEARTBEAT.len());

        move || {
            loop{

                Log::write("sending heartbeat...");

                if self.send(&heartbeat_msg).is_ok(){
                    Log::write("heartbeat reply received. com status: connect.");
                    self.set_connect()
                }
                else{
                    Log::write("no heartbeat reply. com status: disconect.");
                    self.set_disconnect()
                }

                thread::sleep(Duration::from_secs(HEARTBEAT_FREQ))
            }
        }
    }

    fn create_listener(self)->impl FnOnce() + Send + 'static{

        let mut feedback = [0;16];

        move || {
            loop{
                if let Ok(msg) = self.read_once(&mut feedback){

                    if msg.is_event(){
                        Log::write(
                            format!("received event: {:?}, time: {}", msg.data(), Local::now().to_rfc3339()).as_str());
                            
                        RequestsStack::push(Request::battery_event(msg));
                    }
                    else{
                        Log::write(format!("received trash: {feedback:?}").as_str());
                    }
                }
            }
        }
    }

    fn sending(port_guard: &mut MutexGuard<SystemPort>, data: &[u8])->Result<(), io::Error>{

        loop{
            if port_guard.write(data).is_err_and(|err|err.kind() == ErrorKind::Interrupted){
                continue;
            }
            return Ok(())
        }
    }
    
    fn reading(port_guard: &mut MutexGuard<SystemPort>)->Result<ModbusMsg, io::Error>{

        let mut feedback = [0; 8];
 
        loop{
            match port_guard.read(&mut feedback){
                Ok(bytes_count)=> return Ok(ModbusMsg::from(&feedback[..], bytes_count)),
                Err(err) if err.kind() == ErrorKind::Interrupted => continue,
                Err(err) => return Err(err)
            }
        }
    }
}


impl Deref for Modbus{
    type Target = ComStatus;
    fn deref(&self) -> &Self::Target {
        self.status.borrow()
    }
}

impl DerefMut for Modbus{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.status.borrow_mut()
    }
}