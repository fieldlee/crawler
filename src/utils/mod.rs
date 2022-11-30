pub mod error;
pub mod string;
use random_number::random;
use error::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};


pub const CODE_SUCCESS: i8 = 0;
pub const CODE_FAIL: i8 = -1;

/// http接口返回模型结构，提供基础的 code，msg，data 等json数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<i8>,
    pub msg: Option<String>,
    pub data: Option<T>,
}


impl<T> RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                code: Some(CODE_SUCCESS),
                msg: None,
                data: arg.clone().ok(),
            }
        } else {
            Self {
                code: Some(CODE_FAIL),
                msg: Some(arg.clone().err().unwrap().to_string()),
                data: None,
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: Some(CODE_SUCCESS),
            msg: None,
            data: Some(arg.clone()),
        }
    }

    pub fn from_error(arg: &Error) -> Self {
        Self {
            code: Some(CODE_FAIL),
            msg: Some(arg.to_string()),
            data: None,
        }
    }

    pub fn from_error_info(code: i8, info: &str) -> Self {
        Self {
            code: Some(code),
            msg: Some(info.to_string()),
            data: None,
        }
    }

}

impl<T> ToString for RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}


pub fn random_seconds() -> u8 {
    let mut rng = random_number::rand::thread_rng();
    let n: u8 = random!(..=9, rng);
    n
}


pub fn u64_to_i32(v_64:u64) -> i32 {
    let v_32 = v_64 as i32;
    v_32
}