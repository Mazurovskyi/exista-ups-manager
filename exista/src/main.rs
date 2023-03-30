use std::error::Error;
use exista_lib::application::App;

fn main() -> Result<(), Box<dyn Error>>{

    let app_config = App::config().or_else(|err|{
        println!("Configuration error: {err}"); 
        Err(err)
    })?;

    App::launch(app_config).run_forever().or_else(|err|{
        println!("Executing error: {err}");
        Err(err)
    })
}