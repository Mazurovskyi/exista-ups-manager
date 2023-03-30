use std::env;
use std::io::{self, Write};

use std::fs::File;
use once_cell::sync::Lazy;
use chrono::Local;
use crate::application::LOG_FILE_PATH;


static mut LOG_FILE: Lazy<Log> = Lazy::new(||Log::init());

pub struct Log(Option<File>);
impl Log{
    pub fn write(msg: &str){

        let time = Local::now().to_rfc3339();
        let msg = format!("{time}: {msg}\n");

        unsafe{
            let res = LOG_FILE.0.as_mut()
                .and_then(|file|file.write(msg.as_bytes()).ok())
                .is_none();
            
            if res {
                println!("Cannot write message into log file!")
            }
        }
    }

    fn init()->Self{
        Self(Self::get_file().ok())
    }

    fn get_file()->Result<File, io::Error>{
        if let Ok(path) = env::var("EXISTA_LOG"){
            File::options()
                .truncate(true)
                .write(true)
                .open(path)
                .or_else(|_|File::create(LOG_FILE_PATH))
        }
        else{
            File::create(LOG_FILE_PATH)
        }
    }
}


