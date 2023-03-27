
use crate::modbus::Modbus;
use crate::mqtt::MqttClient;
use crate::requests::requests_stack::RequestsStack;

pub mod constants;
pub mod loger;
use std::borrow::Borrow;
use std::error::Error;

use crate::application::loger::Log;
use chrono::Local;
use self::constants::*;


pub struct App{
    modbus: Modbus,
    mqtt_client: MqttClient,
}
impl App{
    fn modbus(&self)->&Modbus{
        self.modbus.borrow()
    }
    fn mqtt_client(&self)->&MqttClient{
        self.mqtt_client.borrow()
    }

    /// config modbus and mqtt services.
    pub fn config()->Result<Self, Box<dyn Error>>{

        //let heartbeat = 
        //let listner = 

        let modbus = Modbus::config(PORT, TIMEOUT)?;
        Log::write("Serial port configured.");

        let mqtt_client = MqttClient::config()?;
        Log::write("Mqtt client configured.");

        Ok(
            App{
                modbus, 
                mqtt_client
            }
        )
    }

    /// run mqtt client and modbus communication.
    pub fn run(app_config: Self)->Result<(), Box<dyn Error>>{
        
        Log::write("running mqtt client...");
        app_config.mqtt_client().run();

        Log::write("running modbus...");
        let _modbus_serv = app_config.modbus().run();

        app_config.run_forever()
    }

    fn run_forever(self)->Result<(), Box<dyn Error>>{
        
        loop{

            // block current thread until data in stack become avalliable:
            let mut request = RequestsStack::pull()?;

            if let Err(err) = request.fill_with_data(self.modbus()){
                Log::write(format!("Error while trying to fill Json pattern with data: {err}").as_str());
                continue;
            }
            
            let time = Local::now().to_rfc3339();
            Log::write(format!("\nJson pattern is ready: {time}\n{}", request).as_str());
            
            if let Err(err) = self.mqtt_client().publish(request, DELIVERY_TIME){
                Log::write(format!("Delivery time out. Message has not delivered. {err}").as_str());
            }
            else {
                Log::write(format!("Successfully delivered! {}", Local::now().to_rfc3339()).as_str());
            }
        }
    }
}





















































use std::sync::{Arc, Mutex, mpsc};
use mpsc::{Sender, Receiver};
use std::sync::mpsc::RecvError;



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