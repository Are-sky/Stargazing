use bilive_danmaku::Connector;
use futures_util::StreamExt;
use live::check_live;
use num_cpus;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

pub mod live;
pub mod live_json;
pub mod my_error;

fn main() {
    let pool = ThreadPool::new(num_cpus::get());
    let room_ids = vec![55, 25921736, 22746343];
    let mut listening_rooms = HashSet::new();
    loop {
        for room_id in &room_ids {
            let room_id = room_id.clone();
            match check_live(room_id) {
                Ok(true) => {
                    if !listening_rooms.contains(&room_id) {
                        println!("开始监听{}直播间", room_id);
                        listening_rooms.insert(room_id);
                        pool.execute(move || {
                            // 监听 room_id 直播间
                            let rt = tokio::runtime::Builder::new_current_thread()
                                .enable_all()
                                .build()
                                .unwrap();
                            rt.block_on(listen(room_id));
                        });
                    }
                }
                _ => continue,
            }
        }
        thread::sleep(Duration::from_secs(5));
    }
}

async fn listen(room_id: u64) {
    let connector = Connector::init(room_id).await.unwrap();
    let mut stream = connector.connect().await.unwrap();
    while let Some(maybe_evt) = stream.next().await {
        match maybe_evt {
            Ok(evt) => {
                println!("{}:{}", room_id, evt.to_json().unwrap());
            }
            Err(e) => {
                dbg!(e);
            }
        }
    }
    stream.abort();
}
