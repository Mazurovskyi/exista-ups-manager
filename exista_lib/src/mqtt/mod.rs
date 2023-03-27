use std::borrow::{Borrow, BorrowMut};
use std::time::Duration;
use std::error::Error;

use paho_mqtt::{AsyncClient,ConnectOptions};

use crate::application::constants::*;
use crate::requests::Request;

pub mod callback;
use callback::Callbacks;




///provides an AsyncClient representation with it`s own connect options.
pub struct MqttClient{
    client: AsyncClient,
    connect_options: Option<ConnectOptions>,
    callbacks: Callbacks,
}
impl MqttClient{

    /// returns Mqtt callbacks to configure and running Mqtt client
    pub fn callbacks()->Callbacks{
        Callbacks::new()
    }
    
    pub fn config(mut callback: Callbacks)-> Result<MqttClient, Box<dyn Error>>{

        // client creation options
        let creation_options = paho_mqtt::CreateOptionsBuilder::new();

        let creation_options = creation_options.server_uri(HOST)
            .client_id(CLIENT_ID)
            .user_data(Box::new([TOPIC_BATTERY_INFO_REQ, TOPIC_DEVICE_INFO]))
            .mqtt_version(MQTT_VERSION)
            .finalize();

        // Create the new MQTT client based on creation options
        let client = AsyncClient::new(creation_options)?;

        client.set_connected_callback(callback.connected());
        client.set_connection_lost_callback(callback.connection_lost());
        client.set_message_callback(callback.message_callback());
        
        // client connection options. MQTT v3.x connection.
        let mut conn_opts = paho_mqtt::ConnectOptionsBuilder::new();
        
        let conn_opts = conn_opts
        .keep_alive_interval(Duration::from_secs(KEEP_ALIVE))
        .finalize();
        //.will_message(lwt);

        Ok(Self::from(client, conn_opts, callback))
    }

    pub fn run(&mut self){

        let on_connect_success = self.callback().on_connect_success();
        let on_connect_failure = self.callback().on_connect_failure();

        self.client().connect_with_callbacks(
            self.options(), 
            on_connect_success, 
            on_connect_failure);
    }

    pub fn publish(&self, request: &Request, timeout: Duration, topic: &str)->Result<(), paho_mqtt::Error>{

        let msg = paho_mqtt::MessageBuilder::new()
                .topic(topic)
                .payload(request.serialize())
                .qos(request.qos())
                .finalize();

        self.client().publish(msg).wait_for(timeout)
    }

    pub fn client(&self)->&AsyncClient{
        self.client.borrow()
    }
    pub fn options(&self)->ConnectOptions{
       self.connect_options.clone().unwrap()
    }
    
    fn from(client: AsyncClient, connect_options: ConnectOptions, callbacks: Callbacks)->Self{
        Self { 
            client, 
            connect_options: Some(connect_options), 
            callbacks
        }
    }
    fn callback(&mut self)->&mut Callbacks{
        self.callbacks.borrow_mut()
    }
}








