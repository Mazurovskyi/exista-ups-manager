use crate::json_patterns::{BatteryInfo, UpsInfo, BatteryEvent};
use crate::{modbus::Modbus, json_patterns::Insertion};
use crate::mqtt::MqttClient;
use paho_mqtt::{self, AsyncClient};
pub mod constants;
pub mod loger;
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::process;
use crate::application::loger::Log;
use chrono::Local;
use self::constants::*;
use loger::*;

pub struct App{
    modbus: Option<Modbus>,
    mqtt_client: Option<MqttClient>,
    channel: Channel<(&'static str, [u8; 16])>
}
impl App{
    fn modbus(&self)->&Modbus{
        self.modbus.as_ref().unwrap()
    }
    fn mqtt_client(&self)->&MqttClient{
        self.mqtt_client.as_ref().unwrap()
    }
    fn channels(&mut self)->&mut Channel<(&'static str, [u8; 16])>{
        self.channel.borrow_mut()
    }

    /// config modbus and mqtt services.
    pub fn config()->Result<Self, Box<dyn Error>>{

        let mut request_channel: Channel<(&str, [u8; 16])> = Channel::new(3);

        //let mut stack = Stack::new();

        let modbus = Modbus::config(PORT, TIMEOUT)?;
        Log::write("Serial port configured.");

        let request_tx = request_channel.get_transmitter().unwrap();
        let mqtt_client = MqttClient::config(request_tx)?;

        Log::write("Mqtt client configured.");

        Ok(
            App{
                modbus: Some(modbus), 
                mqtt_client: Some(mqtt_client), 
                channel: request_channel
            }
        )
    }

    /// run mqtt client and modbus communication.
    pub fn run(mut app_config: Self)->Result<(), Box<dyn Error>>{
        
        Log::write("running mqtt client...");
        app_config.mqtt_client().run();

        Log::write("running modbus...");
        let event_tx = app_config.channels().get_transmitter()?;
        let (_heartbeat, _listener)= app_config.modbus().run(event_tx);

        Log::write("config is done.\n\n");
        app_config.run_forever()?;

        Ok(())
    }

    fn run_forever(mut self)->Result<(), Box<dyn Error>>{
        while let Ok(pattern) = self.channels().recv(){

            let mut pattern: Box<dyn Insertion> = match pattern{
                (TOPIC_BATTERY_INFO_REQ,_) => Box::new(BatteryInfo::build()?),
                (TOPIC_DEVICE_INFO, _) =>     Box::new(UpsInfo::build()?),          //panic here while unwrap()
                (TOPIC_EVENT, msg) =>         Box::new(BatteryEvent::build(msg)?),
                _ => return Err("unreachable topic".into())
            };

            if let Err(err) = pattern.fill(self.modbus()){
                Log::write(format!("Error while trying to fill pattern with data: {err}").as_str());
                continue;
            }
            
            let time = Local::now().to_rfc3339();
            Log::write(format!("\nJson pattern is ready: {time}\n{pattern}").as_str());
            
            if let Err(err) = pattern.publish(self.mqtt_client(), DELIVERY_TIME){
                Log::write(format!("Delivery time out. Message has not delivered. {err}").as_str());
            }
            else {
                Log::write(format!("Successfully delivered! {}", Local::now().to_rfc3339()).as_str());
            }
        }

        Err("channel shut down because all of corresponding senders had disconnected, 
        or it disconnected while this call is blocking".into())

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