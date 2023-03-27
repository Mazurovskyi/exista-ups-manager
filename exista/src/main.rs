use std::error::Error;
use exista_lib::application::{App, loger::Log};

fn main() -> Result<(), Box<dyn Error>>{

    let app_config = match App::config(){
        Ok(val) => val,
        Err(err) => return log_err("Configuration error: ", err)
    };

    if let Err(err) = App::launch(app_config).run_forever(){
        return log_err("Executing error: ", err.into())
    }

    Ok(())
}

fn log_err(msg: &str, err: Box<dyn Error>) -> Result<(), Box<dyn Error>>{
    let err_msg = format!("{msg}{err}");
    Log::write(&err_msg);
    Err(err)
}