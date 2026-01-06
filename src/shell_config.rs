use serde::{Deserialize, de::Error};
// di sini config config utuk `m-shell` dengan file Toml 

#[derive(Deserialize)]
pub struct Config{ 
    path_log : String,
    path_config : String,
}


impl Config {
    //  FUNTION TUNGGAL  
    // *bgdar : untuk sekarang cuman bisa export begini aja dulu 
    pub fn get_path_log( &self) -> String {
        if !self.path_log.is_empty() {
            self.path_log.clone();
        }
        String::from("tidak di temukan path log file")
    }
    pub fn get_path_config( &self) -> String {
        if !self.path_config.is_empty() {
            self.path_config.clone();
        }
        String::from("tidak di temukan path config file")
    }



    
}


