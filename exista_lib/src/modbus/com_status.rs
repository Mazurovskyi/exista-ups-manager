use std::sync::{Arc, RwLock};
use crate::application::constants::*;


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