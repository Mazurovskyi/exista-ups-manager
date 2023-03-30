use std::{thread, time::Duration};

use super::{Modbus, ModbusMsg};
use crate::application::{loger::Log, constants::*};
use crate::requests::{Request, requests_stack::RequestsStack};



/// representation of Modbus service: heartbeat or listener
pub struct Service(Box<dyn FnOnce() + Send + 'static>);
impl Service{
    pub fn new_list(bus: &Modbus)->Vec<Self>{
        let heartbeat = Service::new(heartbeat(bus.clone()));
        let listener = Service::new(listener(bus.clone()));
        Vec::from([heartbeat, listener])
    }

    fn new(closure: impl FnOnce() + Send + 'static)->Self{
        Self(Box::new(closure))
    }
}
impl FnOnce<()> for Service{
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
                   
                    Log::write(&format!("Received an event: {:?}", msg.data()));
                        
                    RequestsStack::push(Request::battery_event(msg))
                        .unwrap_or_else(|err|{
                            panic!("Executing error: can`t write event into stack! {err}");
                        });
                }
                else{
                    println!("received trash: {:?}", feedback);
                }
            }
        }
    }
}