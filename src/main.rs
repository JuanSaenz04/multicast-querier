
use std::io::Error;
use std::os::fd::{AsRawFd, OwnedFd};

use nix::sys::socket::{MsgFlags, SockaddrIn, sendto};

use crate::config::InterfaceConfig;
use crate::packet::igmp::Igmp_packet;
use crate::socket::igmp::create_igmp_socket;

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

    match send_igmp_packet(&fd) {
        Ok(_) => println!("Query sent successfully"),
        Err(e) => eprintln!("Failed to sent IGMP packet: {}", e)
    };
}

fn send_igmp_packet(fd: &OwnedFd) -> Result<(), Error> {
    let igmp_packet = Igmp_packet::New();

    sendto(fd.as_raw_fd(), &igmp_packet.serialize(), &SockaddrIn::new(224, 0, 0, 1, 0), MsgFlags::empty())?;

    Ok(())
}