use std::borrow::{Borrow, BorrowMut};
use std::error::Error;

use crate::modbus::Modbus;
use crate::mqtt::MqttClient;
use crate::requests::requests_stack::RequestsStack;

pub mod constants;
use self::constants::*;

pub mod loger;
use loger::Log;



pub struct App{
    modbus: Modbus,
    mqtt_client: MqttClient,
}
impl App{
    /// config modbus and mqtt services.
    pub fn config()->Result<Self, Box<dyn Error>>{

        let modbus = Modbus::config(PORT, TIMEOUT)?;

        let callbacks = MqttClient::callbacks();
        let mqtt_client = MqttClient::config(callbacks)?;

        Ok(
            App{modbus, mqtt_client}
        )
    }

    /// launch mqtt client and modbus communication.
    pub fn launch(mut self)->Self{

        self.mqtt_client_mut().run();

        let services = Modbus::services(self.modbus());
        let _modbus_services = self.modbus().run(services);

        self
    }

    /// run application
    pub fn run_forever(self)->Result<(), Box<dyn Error>>{
        loop{
            // block current thread until data in stack become avalliable:
            let mut request = RequestsStack::pull()?;

            if let Err(err) = request.insert_data(self.modbus()){
                Log::write(&format!("Error while trying to insert data into Request: {err}"));
                continue;
            }
            
            Log::write(&format!("Json pattern is ready: \n{request}"));
            
            let result = self.mqtt_client().publish(&request, DELIVERY_TIME, request.topic()).and(
                if request.bat_ic_low(){
                    self.mqtt_client().publish(&request, DELIVERY_TIME, TOPIC_RESET)
                }
                else{
                    Ok(())
                }
            );

            match result{
                Ok(_) => Log::write("Successfully delivered."),
                Err(err) => Log::write(&format!("Delivery time out. Message has not delivered. {err}"))
            }
        }
    }

    fn modbus(&self)->&Modbus{
        self.modbus.borrow()
    }
    fn mqtt_client(&self)->&MqttClient{
        self.mqtt_client.borrow()
    }
    fn mqtt_client_mut(&mut self)->&mut MqttClient{
        self.mqtt_client.borrow_mut()
    }
}








