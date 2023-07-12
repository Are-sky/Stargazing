use bilive_danmaku::{event::EventData, Connector};
use colored::*;
use futures_util::StreamExt;

pub async fn start(room_id: u64) {
    let connector = Connector::init(room_id).await.unwrap();
    let mut stream = connector.connect().await.unwrap();
    while let Some(maybe_evt) = stream.next().await {
        match maybe_evt {
            Ok(evt) => {
                let event = evt.clone();
                // TODO 弹幕数据监听 保存至数据库
                match evt.data {
                    EventData::GuardBuyEvent(_) => {
                        println!(
                            "{}",
                            format!("{}:{}", room_id, event.to_json().unwrap()).red()
                        );
                    }
                    EventData::DanmakuEvent(_) => {
                        println!(
                            "{}",
                            format!("{}:{}", room_id, event.to_json().unwrap()).blue()
                        );
                    }
                    EventData::StopLiveEvent(_) => {
                        println!(
                            "{}",
                            format!("{}:{}", room_id, event.to_json().unwrap()).yellow()
                        );
                    }
                    _ => continue,
                    //TODO 完善bilive_danmaku 以支持下播来进行任务停止
                }
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }
    println!("{}直播结束", room_id);
    stream.abort();
}
