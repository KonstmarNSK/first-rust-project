use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use serde::Serialize;
use serde::Deserialize;
 
#[derive(Serialize, Deserialize)]
pub struct AppCfg{
    pub resource_reciever : ResourceRecieverType,
    pub inet_addr : String,
}

#[derive(Serialize, Deserialize)]
pub enum ResourceRecieverType{
    ActixClient,
}

// cfgReaders
pub fn get_app_config_from_json_file(file_cfg: &mut File) -> Result<AppCfg, String> {
    let mut buf_reader = BufReader::new(file_cfg);
    let mut contents = String::new();
    match buf_reader.read_to_string(&mut contents){
        Ok(_) => get_app_config_from_json_str(&contents),
        _ => Err(String::from("Ошибка при чтении файла конфигурации")),
    }
}

pub fn get_app_config_from_json_str(str_cfg: &str) -> Result<AppCfg, String> {
    match serde_json::from_str::<AppCfg>(str_cfg) {
        Ok(app_cfg) => Ok(app_cfg),
        _ => Err(String::from("Ошибка при попытке парсинга конфигурации")),
    }

}

