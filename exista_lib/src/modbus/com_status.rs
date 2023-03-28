use std::sync::{Arc, RwLock};
use crate::application::constants::*;


///UPS com status
pub struct ComStatus(Arc<RwLock<Connection>>);

impl ComStatus{
    pub fn set_connect(&mut self){
        *self.0.write().unwrap() = Connection::connect();
    }
    pub fn set_disconnect(&mut self){
        *self.0.write().unwrap() = Connection::disconect();
    }
    pub fn clone(&self)->Self{
        ComStatus(Arc::clone(&self.0))
    }
    pub fn get_status(&self)->u8{
        match *self.0.read().unwrap(){
            Connection::Connect(val)=>val,
            Connection::Disconnect(val)=>val
        }
    }
}

impl Default for ComStatus{
    fn default() -> Self {
        Self(Arc::new(RwLock::new(Connection::disconect())))
    }
}

#[derive(PartialEq)]
enum Connection{
    Connect(u8),
    Disconnect(u8)
}
impl Connection{
    fn connect()->Self{
        Self::Connect(CONNECT)
    }
    fn disconect()->Self{
        Self::Disconnect(DISCONNECT)
    }
}
