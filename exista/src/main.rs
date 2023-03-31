use std::error::Error;
use exista_lib::application::{App, loger::Log};


fn main() -> Result<(), Box<dyn Error>>{

    Log::write("New session begins");

    let app_config = App::config().or_else(|err|{
        Log::write(&format!("Configuration error: {err}")); 
        Err(err)
    })?;

    App::launch(app_config).run_forever().or_else(|err|{
        Log::write(&format!("Executing error: {err}"));
        Err(err)
    })
}