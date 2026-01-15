
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::interface::run_interface_thread;

mod packet;
mod socket;
mod config;
mod interface;
mod querier;

fn main() -> Result<(), Box<dyn Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    let interface_name = String::from("eth0"); // Hardcoded for now

    let handles = run_interface_thread(interface_name, running)?;

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}