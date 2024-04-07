use std::{fs::File, io::Read};

use anyhow::Result;
use serde_json::Value;

const FOLDER_PATH: &str = "staticData/fund/strategyBoard";

// 产品列表
pub fn sync_product() -> Result<Value> {
    let data_path = FOLDER_PATH.to_string() + "/syncProduct.json";
    let mut file = File::open(data_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let res: Value = serde_json::from_str(&content)?;
    Ok(res)
}

// 策略列表
pub fn show_strategy() -> Result<Value> {
    let data_path = FOLDER_PATH.to_string() + "/showStrategy.json";
    let mut file = File::open(data_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let res: Value = serde_json::from_str(&content)?;
    Ok(res)
}

// 策略类型
pub fn load_category() -> Result<Value> {
    let data_path = FOLDER_PATH.to_string() + "/category.json";
    let mut file = File::open(data_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let res: Value = serde_json::from_str(&content)?;
    Ok(res)
}

// 策略版本
pub fn load_version() -> Result<Value> {
    let data_path = FOLDER_PATH.to_string() + "/version.json";
    let mut file = File::open(data_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let res: Value = serde_json::from_str(&content)?;
    Ok(res)
}

// 批量开仓
pub fn parse_file() -> Result<Value> {
    let data_path = FOLDER_PATH.to_string() + "/parseFile.json";
    let mut file = File::open(data_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let res: Value = serde_json::from_str(&content)?;
    Ok(res)
}

// 保存状态
pub fn save_basic_info() -> Result<Value> {
    let data_path = FOLDER_PATH.to_string() + "/parseFile.json";
    let mut file = File::open(data_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let res: Value = serde_json::from_str(&content)?;
    Ok(res)
}
