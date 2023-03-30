use std::borrow::{BorrowMut, Borrow};
use std::error::Error;
use std::time::Duration;
use std::io::{self, Write, Read, ErrorKind};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};

extern crate serial;
use serial:: {SystemPort, prelude::SerialPort};

mod com_status;
pub mod msg;
pub mod services;

use com_status::ComStatus;
use msg::ModbusMsg;
use services::Service;




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

    /// creates a new Modbus service list: heartbeat and listener
    pub fn services(&self)->Vec<Service>{
        Service::new_list(self)
    }

    /// creates and configure the Modbus port.
    pub fn config(port: &str, timeout: u64)->Result<Self, Box<dyn Error>>{
        let bus = Modbus{
            port: ModbusPort::open(port)?.config()?.timeout(timeout)?,
            status: ComStatus::default()
        };

        dbg!("Modbus configured.");
        Ok(bus)
    }

    /// running modbus services.
    pub fn run(&self, services: Vec<Service>)->Vec<JoinHandle<()>>{
        dbg!("Running modbus...");
        services.into_iter().map(thread::spawn).collect()
    }

    /// athomary operation to send data into modbus and return a reply immediately.
    pub fn send(&self, msg: &ModbusMsg)->Result<ModbusMsg, Box<dyn Error + '_>>{

        dbg!("sending modbus message: {:?}", msg.data());

        let mut port_guard = self.port().lock()?;

        Self::sending(&mut port_guard, msg.data())?;
        let returned_msg = Self::reading(&mut port_guard)?;

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

    fn sending(port_guard: &mut MutexGuard<SystemPort>, data: &[u8])->Result<(), io::Error>{
        loop{
            match port_guard.write(data){
                Ok(_) => return Ok(()),
                Err(err) if err.kind() == ErrorKind::Interrupted => continue,
                Err(err) => return Err(err)
            }
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

