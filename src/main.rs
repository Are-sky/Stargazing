use live::check_live;
use models::{listen, live, logo};
use num_cpus;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

pub mod models;

static MAX_THREADS: usize = 50;

fn main() {
    logo::print_logo();
    let mut pool = ThreadPool::new(num_cpus::get());
    //TODO 监听room_id的读取
    let room_ids = vec![25851629];
    let mut listening_rooms = HashSet::new();
    loop {
        for room_id in &room_ids {
            let room_id = room_id.clone();
            match check_live(room_id) {
                Ok(true) => {
                    if !listening_rooms.contains(&room_id) {
                        println!("开始监听{}直播间", room_id);
                        listening_rooms.insert(room_id);
                        //判断线程池是否满
                        if pool.queued_count() + pool.active_count() >= (pool.max_count() / 2) {
                            //动态扩容
                            pool.set_num_threads(min(pool.max_count() * 2, MAX_THREADS));
                        } else if pool.queued_count() + pool.active_count()
                            < (pool.max_count() / 2 - 1)
                        {
                            //动态缩小
                            pool.set_num_threads(max(pool.max_count() / 2, num_cpus::get()));
                        }
                        // 监听 room_id 直播间
                        pool.execute(move || {
                            let rt = tokio::runtime::Builder::new_current_thread()
                                .enable_all()
                                .build()
                                .unwrap();
                            rt.block_on(listen::start(room_id));
                        });
                        // TODO录制直播间
                    }
                }
                _ => continue,
            }
        }
        thread::sleep(Duration::from_secs(5));
    }
}
