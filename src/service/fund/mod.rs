use std::{fs::File, io::Read};

use anyhow::Result;
use serde_json::Value;

const FOLDER_PATH: &str = "staticData/fund";

// 产品列表
pub fn sync_product() -> Result<Value> {
    let data_path = FOLDER_PATH.to_string() + "/strategyBoard/syncProduct.json";
    let mut file = File::open(data_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let res: Value = serde_json::from_str(&content)?;
    Ok(res)
}

// 策略列表
pub fn show_strategy() -> Result<Value> {
    let data_path = FOLDER_PATH.to_string() + "/strategyBoard/showStrategy.json";
    let mut file = File::open(data_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let res: Value = serde_json::from_str(&content)?;
    Ok(res)
}
