use std::process;
use std::{thread, time::Duration};

use paho_mqtt::{AsyncClient, Message};

use crate::application::{loger::Log, constants::*};

mod msg;
use msg::Handler;


// callback on incoming messages.
pub fn message_callback(_client: &AsyncClient, msg: Option<Message>){
    match msg.handle(){
        Ok(report) => Log::write(format!("mqtt message handle result: {report}").as_str()),
        Err(report) => {
            Log::write(format!("Error was heappen handling the message: {report}").as_str());
            process::exit(1);
        }
    }
}

/// closure to be called when connection is established.
pub fn set_connected(_client: &AsyncClient){
    Log::write("connected to mqtt broker")
}

/// closure to be called if the client loses the connection. Try to reconect
pub fn set_connection_lost(client: &AsyncClient){
    Log::write("mqtt broker connection lost. Trying to reconnect...");
    thread::sleep(Duration::from_millis(1000));
    client.reconnect_with_callbacks(on_connect_success, on_connect_failure);
}

/// Callback for a successful connection to the broker. Subscribe the topics
pub fn on_connect_success(client: &AsyncClient, _msgid: u16){

    client.subscribe_many(&[TOPIC_BATTERY_INFO_REQ, TOPIC_DEVICE_INFO], &[QOS, QOS]);

    Log::write(
        format!("successful connection to the broker.
        subscribed to topics: {:?}", [TOPIC_BATTERY_INFO_REQ, TOPIC_DEVICE_INFO]).as_str()
    );
}

/// Callback for a fail connection
pub fn on_connect_failure(client: &AsyncClient, _msgid: u16, rc: i32){
    Log::write(
        format!("Connection attempt failed with error code {}.
        trying to reconnect...", rc).as_str()
    );
        
    thread::sleep(Duration::from_millis(1000));
    client.reconnect_with_callbacks(on_connect_success, on_connect_failure);
}

