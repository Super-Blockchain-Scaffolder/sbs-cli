use serde_yaml::Value;
use std::error::Error;

use reqwest::blocking::get;

pub const MASTER_LIST_URL: &str = "https://raw.githubusercontent.com/Blockchain-Super-Scaffolder/bss-master-list/main/bss-master-list.yaml";

pub fn get_master_list_data() -> Result<Value, Box<dyn Error>> {
    let response = get(MASTER_LIST_URL)?.text()?;
    let serde_result = serde_yaml::from_str::<Value>(&response)?;
    Ok(serde_result)
}
