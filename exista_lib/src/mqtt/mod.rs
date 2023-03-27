use std::borrow::Borrow;
use std::time::Duration;
use std::error::Error;

use paho_mqtt::{AsyncClient,ConnectOptions};

use crate::application::constants::*;
use crate::requests::Request;

pub mod callbacks;
use callbacks as callback;



///provides an AsyncClient representation with it`s own connect options.
pub struct MqttClient{
    client: AsyncClient,
    connect_options: Option<ConnectOptions>
}
impl MqttClient{
    
    pub fn config()-> Result<MqttClient, Box<dyn Error>>{

        // client creation options
        let creation_options = paho_mqtt::CreateOptionsBuilder::new();

        let creation_options = creation_options.server_uri(HOST)
            .client_id(CLIENT_ID)
            .user_data(Box::new([TOPIC_BATTERY_INFO_REQ, TOPIC_DEVICE_INFO]))
            .mqtt_version(MQTT_VERSION)
            .finalize();

        // Create the new MQTT client based on creation options
        let client = AsyncClient::new(creation_options)?;

        client.set_connected_callback(callback::set_connected);
        client.set_connection_lost_callback(callback::set_connection_lost);
        client.set_message_callback(callback::message_callback);
        
        // client connection options. MQTT v3.x connection.
        let mut conn_opts = paho_mqtt::ConnectOptionsBuilder::new();
        
        let conn_opts = conn_opts
        .keep_alive_interval(Duration::from_secs(KEEP_ALIVE))
        .finalize();
        //.will_message(lwt);

        Ok(Self::from(client, conn_opts))
    }

    pub fn run(&self){
        self.client().connect_with_callbacks(
            self.options(), 
            callback::on_connect_success, 
            callback::on_connect_failure);
    }

    pub fn publish(&self, request: Request, timeout: Duration)->Result<(), paho_mqtt::Error>{

        let msg = paho_mqtt::MessageBuilder::new()
                .topic(request.topic())
                .payload(request.serialize())
                .qos(request.qos())
                .finalize();

        let token = self.client().publish(msg);

        token.wait_for(timeout)
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
}








