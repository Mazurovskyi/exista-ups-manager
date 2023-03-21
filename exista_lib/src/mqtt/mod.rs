
use std::borrow::Borrow;

use {paho_mqtt, paho_mqtt::Message};

use std::borrow::BorrowMut;
use std::fmt::Display;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, mpsc::Sender};
use std::{env, process, sync::RwLock, thread, time::Duration};
use std::error::Error;

use paho_mqtt::{AsyncClient, connect_options, ConnectOptions};

use crate::application::constants::*;
use crate::application::loger::Log;
use crate::json_patterns::Insertion;
use crate::modbus::Modbus;
use crate::mqtt::msg::Handler;
mod msg;



///provides an AsyncClient representation with it`s own connect options.
pub struct MqttClient{
    client: AsyncClient,
    connect_options: Option<ConnectOptions>
}
impl MqttClient{
    
    pub fn config(tx: Arc<Mutex<Sender<(&'static str, [u8; 16])>>>)-> Result<MqttClient, Box<dyn Error>>{

        // client creation options
        let creation_options = paho_mqtt::CreateOptionsBuilder::new();

        let creation_options = creation_options.server_uri(HOST)
            .client_id(CLIENT_ID)
            .user_data(Box::new(SUBSCRIBE_TOPICS))
            .mqtt_version(MQTT_VERSION)
            .finalize();


        // Create the new MQTT client based on creation options
        let client = AsyncClient::new(creation_options)?;

        // closure to be called when connection is established.
        client.set_connected_callback(|_cli: &AsyncClient| {
            Log::write("connected to mqtt broker")
        });

        // closure to be called if the client loses the connection. Try to reconect
        client.set_connection_lost_callback(|client: &AsyncClient| {
            Log::write("mqtt broker connection lost. Trying to reconnect...");
            thread::sleep(Duration::from_millis(1000));
            client.reconnect_with_callbacks(Self::on_connect_success, Self::on_connect_failure);
        });

        
        // callback on incoming messages.
        client.set_message_callback(move |_client, msg: Option<Message>| {
            let result = msg.handle(tx.clone())
                            .unwrap_or_else(|err| err.to_string());

            Log::write(format!("mqtt message handle result: {result}").as_str());
        });
        

        // client connection options. MQTT v3.x connection.
        let mut conn_opts = paho_mqtt::ConnectOptionsBuilder::new();
        
        let conn_opts = conn_opts
        .keep_alive_interval(Duration::from_secs(KEEP_ALIVE))
        .finalize();
        //.will_message(lwt);

        Ok(Self::from(client, conn_opts))
    }

    /// connect client to the mqtt broker.
    pub fn run(&self){
        self.client().connect_with_callbacks(
            self.options(), 
            Self::on_connect_success, 
            Self::on_connect_failure);
    }

    pub fn client(&self)->&AsyncClient{
        self.client.borrow()
    }
    pub fn options(&self)->ConnectOptions{
       self.connect_options.clone().unwrap()
    }
    fn from(client: AsyncClient, connect_options: ConnectOptions)->Self{
        MqttClient { client, connect_options: Some(connect_options)}
    }

    pub fn publish(&self, pattern: & dyn Insertion, timeout: Duration, topic: &str)->Result<(), paho_mqtt::Error>{

        let msg = paho_mqtt::MessageBuilder::new()
                .topic(topic)
                .payload(pattern.serialize())
                .qos(0)
                .finalize();

        let token = self.client().publish(msg);

        token.wait_for(timeout)
    }

    // Callback for a successful connection to the broker. Subscribe the topics
    fn on_connect_success(client: &AsyncClient, _msgid: u16){
        client.subscribe_many(&SUBSCRIBE_TOPICS, QOS);

        Log::write(
            format!("successful connection to the broker.
            subscribed to topics: {:?}", SUBSCRIBE_TOPICS).as_str());
    }

    // Callback for a fail connection
    fn on_connect_failure(client: &AsyncClient, _msgid: u16, rc: i32){
        Log::write(
            format!("Connection attempt failed with error code {}.
            trying to reconnect...", rc).as_str());
            
        thread::sleep(Duration::from_millis(1000));
        client.reconnect_with_callbacks(Self::on_connect_success, Self::on_connect_failure);
    }

}








