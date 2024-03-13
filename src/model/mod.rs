use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResult<T> {
    pub code: i32,
    pub msg: String,
    pub result: T,
}
