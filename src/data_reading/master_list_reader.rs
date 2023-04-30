use serde_yaml::Value;
use std::error::Error;

use crate::shared_constants::URL;
use reqwest::blocking::get;

const master_list_url = "https://raw.githubusercontent.com/Blockchain-Super-Scaffolder/bss-master-list/main/bss-master-list.yaml";

pub fn get_master_list_data() -> Result<(), Box<dyn Error>> {
    let response = get(URL)?.text()?;
    let serde_result = serde_yaml::from_str::<Value>(&response)?;
    Ok(serde_result)
}

