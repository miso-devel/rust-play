use std::{
    sync::{Arc, Mutex},
    thread,
};

#[allow(dead_code)]
fn main() {
    // スレッドの複数立ち上げ
    // let mut handlers = Vec::new();
    // for x in 0..=10 {
    //     handlers.push(thread::spawn(move || {
    //         println!("Hello-world:{}", x);
    //     }))
    // }
    // for handler in handlers {
    //     let _ = handler.join();
    // }

    // 各スレッドでのメモリ共有
    let mut multiple_hundler = Vec::new();
    let share_data = Arc::new(Mutex::new(vec![1; 10]));
    for x in 0..10 {
        let cloned_share_data = share_data.clone();
        multiple_hundler.push(thread::spawn(move || {
            cloned_share_data.lock().unwrap()[x] += x;
        }))
    }

    for handler in multiple_hundler {
        let _ = handler.join();
    }
    println!("{:?}", share_data)
}
