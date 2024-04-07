use std::{fs::File, io::Read};

use anyhow::Result;
use serde_json::Value;

const FOLDER_PATH: &str = "staticData/fund/login";

// 登录
pub fn login() -> Result<Value> {
    let data_path = FOLDER_PATH.to_string() + "/login.json";
    let mut file = File::open(data_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let res: Value = serde_json::from_str(&content)?;
    Ok(res)
}
