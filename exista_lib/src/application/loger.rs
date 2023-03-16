
use std::fs;
use std::{env, env::VarError};
use std::io::{self, Write};

use std::fs::File;
use once_cell::sync::Lazy;
use chrono::Local;
use crate::application::LOG_FILE_PATH;

//static LOG_FILE_PATH: Log = Log::init().unwrap_or(Log::NA);
static mut LOG_FILE: Lazy<Log> = Lazy::new(||Log::init()
    .unwrap_or_else(|err|Log::NA(err)));

pub enum Log{
    OK(File),
    NA(io::Error)
}
impl Log{
    fn init()->Result<Log, io::Error>{

        let mut file = if let Ok(path) = env::var("EXISTA_LOG"){
            File::options()
                .truncate(true)
                .write(true)
                .open(path)
                .or_else(|_|File::create(LOG_FILE_PATH))?
        }
        else{
            File::create(LOG_FILE_PATH)?
        };
            
        let title = format!("\n---new log session---\n {}", Local::now().to_rfc3339());

        if let Err(err) = file.write(title.as_bytes()){
            eprintln!("can`t write title into log file: {err}")
        }

        Ok(Self::OK(file))
    }
    pub fn write(msg: &str){
        let msg = format!("\n{msg}");
        unsafe{
            if let Err(err) = LOG_FILE.try_write(&msg){
                eprintln!("can`t write log: {err}\nmessage: {msg}")
            }
        }
    }
    fn try_write(&mut self, msg: &str)->Result<usize, String>{
        match self{
            Log::OK(file)=> file.write(msg.as_bytes())
                                .or_else(|err|Err(err.to_string())),
            Log::NA(err)=> Err(err.to_string())
        }
    }
}

