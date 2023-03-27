use std::borrow::Borrow;
use std::ops::Deref;
use std::{thread, process, time::Duration};

use chrono::Local;

use super::{Modbus, ModbusMsg};
use crate::application::{loger::Log, constants::*};
use crate::requests::{Request, requests_stack::RequestsStack};

/// returns heartbeat and permanent listening services
pub fn services(bus: &Modbus)->Vec<ModbusService>{
    let heartbeat = ModbusService::new(heartbeat(bus.clone()));
    let listener = ModbusService::new(listener(bus.clone()));
    Vec::from([heartbeat, listener])
}

pub struct ModbusService(Box<dyn FnOnce() + Send + 'static>);
impl ModbusService{
    fn new(closure: impl FnOnce() + Send + 'static)->Self{
        Self(Box::new(closure))
    }
}
impl FnOnce<()> for ModbusService{
    type Output = ();
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        self.0()
    }
}

/// heartbeat service
fn heartbeat(mut bus: Modbus)->impl FnOnce() + Send + 'static{

    let heartbeat_msg = ModbusMsg::from(&HEARTBEAT[..], HEARTBEAT.len());

    move || {
        loop{
            Log::write("sending heartbeat...");

            if bus.send(&heartbeat_msg).is_ok(){
                Log::write("heartbeat reply received. com status: connect.");
                bus.set_connect()
            }
            else{
                Log::write("no heartbeat reply. com status: disconect.");
                bus.set_disconnect()
            }

            thread::sleep(Duration::from_secs(HEARTBEAT_FREQ))
        }
    }
}

/// permanent listening port service. It reacts for incoming events
fn listener(bus: Modbus)->impl FnOnce() + Send + 'static{

    let mut feedback = [0;16];

    move || {
        loop{
            if let Ok(msg) = bus.read_once(&mut feedback){

                if msg.is_event(){
                    Log::write(
                        format!("received event: {:?}, time: {}", msg.data(), Local::now().to_rfc3339()).as_str());
                        
                    RequestsStack::push(Request::battery_event(msg))
                        .unwrap_or_else(|err|{
                            Log::write(format!("can`t write event into stack! {err}").as_str());
                            process::exit(1);
                        });
                }
                else{
                    Log::write(format!("received trash: {feedback:?}").as_str());
                }
            }
        }
    }
}