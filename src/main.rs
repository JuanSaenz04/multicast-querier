
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
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <interface1> [interface2] ...", args[0]);
        std::process::exit(1);
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    let mut all_handles = Vec::new();

    for interface_name in args.into_iter().skip(1) {
        match run_interface_thread(interface_name, running.clone()) {
            Ok(handles) => all_handles.extend(handles),
            Err(e) => eprintln!("Failed to start on interface: {}", e),
        }
    }

    for handle in all_handles {
        handle.join().unwrap();
    }

    Ok(())
}