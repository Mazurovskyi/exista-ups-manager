
use crate::modbus::Modbus;
use crate::mqtt::MqttClient;
use crate::requests::requests_stack::RequestsStack;

pub mod constants;
pub mod loger;
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;

use crate::application::loger::Log;
use chrono::Local;
use self::constants::*;


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
    pub fn run_forever(self)->Result<(), String>{
        loop{
            // block current thread until data in stack become avalliable:
            let mut request = RequestsStack::pull()?;

            if let Err(err) = request.insert_data(self.modbus()){
                Log::write(format!("Error while trying to insert data into Request: {err}").as_str());
                continue;
            }
            
            let time = Local::now().to_rfc3339();
            Log::write(format!("\nJson pattern is ready: {time}\n{}", request).as_str());
            

            let result = self.mqtt_client().publish(&request, DELIVERY_TIME, request.topic()).and(
                if request.bat_ic_low(){
                    self.mqtt_client().publish(&request, DELIVERY_TIME, TOPIC_RESET)
                }
                else{
                    Ok(())
                }
            );

            match result{
                Ok(_) => Log::write(format!("Successfully delivered! {}", Local::now().to_rfc3339()).as_str()),
                Err(err) => Log::write(format!("Delivery time out. Message has not delivered. {err}").as_str())
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







/*

pub struct Channel<T: Send>{
    transmitters: Vec<Arc<Mutex<Sender<T>>>>,
    receiver: Receiver<T>
}

impl <T: Send> Channel<T>{

    ///Creates a new Channel with one Receiver and "tx_count" transmitters
    pub fn new(count: u8)->Self{

        let (tx,rx) = mpsc::channel::<T>();

        let tx = Arc::new(Mutex::new(tx));

        let tx_vect = vec![Arc::clone(&tx),];

        let mut channel = Channel {
            transmitters: tx_vect, 
            receiver: rx 
        };

        for _i in 0..count{
            channel.transmitters.push(Arc::clone(&tx));
        }

        channel
    }

    pub fn get_transmitter(&mut self)->Result<Arc<Mutex<Sender<T>>>, Box<dyn Error>>{
        self.transmitters.pop().ok_or(format!("All transmittes have been extracted!").into())
    }

    pub fn recv(&self)->Result<T, RecvError>{
        self.receiver.recv()
    }
}

*/












