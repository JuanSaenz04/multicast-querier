
use crate::config::{InterfaceConfig};
use crate::socket::igmp::create_igmp_socket;
use crate::socket::mld::create_mld_socket;

mod packet;
mod socket;
mod config;
mod interface;
mod querier;

fn main() {
    println!("Testing IGMP packet send...");

    // Create a test interface config (change "eth0" to your interface name)
    let config = InterfaceConfig {
        name: "eth0".to_string(),
        index: 0,
        enable_igmp: true,
        enable_mld: false,
    };

    // Create the IGMP socket
    let fd = match create_igmp_socket(&config) {
        Ok(sock_fd) => sock_fd,
        Err(e) => {
            eprintln!("Failed to create IGMP socket: {}", e);
            return;
        }
    };

    // Create the MLDv2 socket
    let fd6 = match create_mld_socket(&config) {
        Ok(sock_fd) => sock_fd,
        Err(e) => {
            eprintln!("Failed to create MLDv2 socket: {}", e);
            return;
        }
    };
}