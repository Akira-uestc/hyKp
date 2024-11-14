mod reload;
mod save;
mod sig;

use signal_hook::consts::signal::*;
use signal_hook::flag as signal_flag;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let term_flag = Arc::new(AtomicBool::new(false));
    let term_flag_clone = Arc::clone(&term_flag);

    signal_flag::register(SIGTERM, term_flag.clone()).expect("Failed to register SIGTERM handler");
    signal_flag::register(SIGINT, term_flag.clone()).expect("Failed to register SIGINT handler");

    let handle = thread::spawn(move || {
        while !term_flag_clone.load(Ordering::Relaxed) {
            println!("Working...");
            thread::sleep(Duration::from_secs(1));
        }

        crate::save::save_layout();
    });

    handle.join().expect("Failed to join the thread");

    Ok(())
}
