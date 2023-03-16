extern crate serial;
use std::borrow::{BorrowMut, Borrow};
use std::error::Error;
use std::io::{Write, Read};

use serial:: {SystemPort, prelude::SerialPort};
use std::time::Duration;

use std::sync::{Arc, Mutex, MutexGuard, RwLock};
use std::io::ErrorKind;
pub mod msg;
use msg::ModbusMsg;
use crate::application::loger::Log;

use std::thread::{self, JoinHandle};
use chrono::Local;
use crate::application::constants::*;

use std::sync::mpsc::Sender;
pub struct Modbus{
    port: Arc<Mutex<SystemPort>>,
    status: Arc<RwLock<u8>>
}
impl Modbus{
    /// creates and configure the Modbus port.
    pub fn config(port: &str, timeout: u64)->Result<Self, Box<dyn Error>>{

        let mut port = serial::open(port)?;
    
        //serial port settings
        port.reconfigure(&|port_config|{
            port_config.set_baud_rate(serial::Baud115200)?;
            port_config.set_char_size(serial::Bits8);
            port_config.set_parity(serial::ParityNone);
            port_config.set_stop_bits(serial::Stop1);
            port_config.set_flow_control(serial::FlowNone);
    
            Ok(())
        })?;
    
        //timeout listening the port
        port.set_timeout(Duration::from_millis(timeout))?;  

        let bus = Modbus{
            port: Arc::new(Mutex::new(port)),
            status: Arc::new(RwLock::new(DISCONNECT))
        };

       Ok(bus)
    }

    /// running modbus services: listening the port and running modbus timer.
    pub fn run(&self, tx: Arc<Mutex<Sender<(&'static str, [u8; 16])>>>)->(JoinHandle<()>, JoinHandle<()>){

        let connection_checker = self.clone();
        let listener = self.clone();

        let checker = connection_checker.run_heartbeat();
        
        let listener = listener.run_listening(tx);

        (checker, listener)
    }

    /// athomary operation to send data into modbus and return a reply immediately.
    pub fn send(&self, data: &[u16])->Result<ModbusMsg, Box<dyn Error + '_>>{

        Log::write(format!("sending modbus message: {data:?}").as_str());
        let msg = ModbusMsg::from(data, data.len());

        if let Ok(mut port_guard) = self.port.lock(){

            Self::sending(port_guard.borrow_mut(), msg.data())?;
            let returned_msg = Self::reading(&mut port_guard)?;
            
            thread::sleep(Duration::from_millis(2));
            drop(port_guard);

            Ok(returned_msg)
        }
        else{
            Err("Error trying lock the serial port.".into())
        }
        
        
    }

    /// clones an exist modbus struct owned the serial port. 
    pub fn clone(&self)->Self{
        Modbus{
            port: Arc::clone(&self.port),
            status: Arc::clone(&self.status)
        }
    }
    pub fn get_status(&self)->u8{
        *self.status.read().unwrap()
    }
    pub fn set_status(&mut self, status: u8){
        *self.status.write().unwrap() = status
    }


    // private API
    fn run_heartbeat(mut self)->JoinHandle<()>{

        let heartbeat_forever = move || {
            
            loop{

                Log::write("sending heartbeat...");

                let com_status = match self.send(&HEARTBEAT){
                    Ok(_) => {
                        Log::write("heartbeat reply received. com status: connect.");
                        //self.set_status(CONNECT)
                        CONNECT
                    }
                    Err(err) => {Log::write(
                        format!("catn`t handle a modbus message: {err}\n
                        no heartbeat reply. com status: disconect.").as_str());
                        DISCONNECT
                        //self.set_status(DISCONNECT)
                    }               
                };

                self.set_status(com_status);

                thread::sleep(Duration::from_secs(HEARTBEAT_FREQ))
            }
        };

        thread::spawn(heartbeat_forever)
    }

    fn run_listening(self, tx: Arc<Mutex<Sender<(&'static str, [u8; 16])>>>)->JoinHandle<()>{

        let mut feedback = [0;16];

        let listening_forever = move || {
            loop{
                if let Ok(_) = self.listening_once(&mut feedback){

                    if feedback[..2] == [0x00, 0x64]{
                        Log::write(
                            format!("received event: {feedback:?}, 
                            time: {}", Local::now().to_rfc3339()).as_str());
                            
                        //stack.lock().unwrap().push(TOPIC_EVENT);
                        tx.lock().unwrap().send((TOPIC_EVENT, feedback)).unwrap();
                    }
                    else{
                        Log::write(format!("received trash: {feedback:?}").as_str());
                    }
                }
            }
        };

        thread::spawn(listening_forever)
    }

    fn sending(port_guard: &mut MutexGuard<SystemPort>, data: &[u8])->Result<(), Box<dyn Error>>{

        let mut count = 10;

        while count > 0{
            if let Ok(_) = port_guard.write(data){
                return Ok(())
            }
            count -= 1;
        }
        Err("catn`t send, no connection.".into())
    }
    
    fn reading(mut port_guard: &mut MutexGuard<SystemPort>)->Result<ModbusMsg, Box<dyn Error>>{

        let mut feedback = [0;8];

        let mut count = 10;  // count of attempt read the buffer

        //while count > 0{
        while count > 0{
            //println!("reading athomary...");
            if let Ok(bytes_count) = port_guard.read(&mut feedback){
                return Ok(ModbusMsg::from(&feedback[..], bytes_count))
            }
            count -=1;
        }
            
        Err("no reply.".into())
        //}

    }

    fn listening_once(&self, feedback: &mut [u8])->Result<(), Box<dyn Error+ '_>>{

        let bytes_count = self.port.lock()?.borrow_mut().read(feedback)?;

        Ok(())
    }

}


