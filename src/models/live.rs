use super::{live_json, my_error};
use live_json::LiveJson;
use my_error::MyError;
use std::time::Duration;

pub async fn check_live_async(room_id: u64, duration: Duration) -> Result<bool, MyError> {
    let url = format!(
        "https://api.live.bilibili.com/room/v1/Room/get_info?room_id={}",
        room_id
    );
    let response = match reqwest::Client::new().get(url).send().await {
        Ok(response) => response,
        Err(e) => {
            return Err(MyError::Api(e));
        }
    };
    if response.status().is_success() {
        // 获取响应的文本内容
        let text = response.text().await;

        let msg: LiveJson = serde_json::from_str(&(text.unwrap())).unwrap();
        // println!("{:?}", msg);
        match msg.code {
            0 => match msg.data.live_status {
                1 => {
                    // 等待一段时间再继续循环
                    tokio::time::sleep(duration).await;
                    return Ok(true);
                }
                _ => return Ok(false),
            },

            _ => {
                return Err(MyError::NotFound {
                    reason: format!("房间号为{}的主播不存在", room_id),
                })
            }
        }
    }
    return Ok(false);
}

pub fn check_live(room_id: u64) -> Result<bool, MyError> {
    let url = format!(
        "https://api.live.bilibili.com/room/v1/Room/get_info?room_id={}",
        room_id
    );
    let response = match reqwest::blocking::Client::new().get(url).send() {
        Ok(response) => response,
        Err(e) => {
            return Err(MyError::Api(e));
        }
    };
    if response.status().is_success() {
        // 获取响应的文本内容
        let text = response.text().unwrap();
        // println!("{:?}", text);
        let msg: LiveJson = serde_json::from_str(&text).unwrap();
        // println!("{:?}", msg);
        match msg.code {
            0 => match msg.data.live_status {
                1 => return Ok(true),
                _ => return Ok(false),
            },

            _ => {
                return Err(MyError::NotFound {
                    reason: format!("房间号为{}的主播不存在", room_id),
                })
            }
        }
    }
    return Ok(false);
}
