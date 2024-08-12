use signal_hook::consts::signal::*;
use signal_hook::flag;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn signal_shutdown() -> bool {
        let term_flag = Arc::new(AtomicBool::new(false));

    // 注册关机信号
    flag::register_conditional_shutdown(SIGTERM, 1, Arc::clone(&term_flag)).unwrap();
    flag::register(SIGINT, Arc::clone(&term_flag)).unwrap();

    // 检测信号的线程
    let term_flag_clone = Arc::clone(&term_flag);
    let handle = thread::spawn(move || {
        while !term_flag_clone.load(Ordering::Relaxed) {
            println!("Running... (Press Ctrl+C or send SIGTERM to stop)");
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 等待线程结束，并检查是否收到关机信号
    handle.join().unwrap();

    // 返回 true 表示检测到关机信号
    term_flag.load(Ordering::Relaxed)
}
