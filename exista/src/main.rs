use std::error::Error;
use exista_lib::application::{App, loger::Log};

fn main() -> Result<(), Box<dyn Error>>{

    let app_config = match App::config(){
        Ok(val) => val,
        Err(err) => return log_and_exit("Config error: ", err)
    };

    if let Err(err) = App::run(app_config){
        return log_and_exit("Execute error: ", err)
    }

    Ok(())
}

fn log_and_exit(err_msg: &str, err: Box<dyn Error>) -> Result<(), Box<dyn Error>>{
    let err_msg = format!("{err_msg}{err}");
    Log::write(&err_msg);
    Err(err)
}