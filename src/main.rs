
use std::error::Error;

use crate::interface::run_interface_thread;

mod packet;
mod socket;
mod config;
mod interface;
mod querier;

fn main() -> Result<(), Box<dyn Error>> {
    let interface_name = String::from("eth0"); // Hardcoded for now

    run_interface_thread(interface_name)
}