use std::net::{Ipv4Addr, SocketAddrV4};

use socket2::{SockAddr, Socket};

use crate::config::{InterfaceConfig, IGMP_ALL_HOSTS};
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
    let socket = match create_igmp_socket(&config) {
        Ok(sock) => sock,
        Err(e) => {
            eprintln!("Failed to create IGMP socket: {}", e);
            return;
        }
    };

    // Send a test packet
    match send_igmp_packet(&socket) {
        Ok(_) => println!("IGMP packet sent successfully!"),
        Err(e) => eprintln!("Failed to send IGMP packet: {}", e),
    }
}

fn send_igmp_packet(socket: &Socket) -> std::io::Result<usize> {
    let igmp_packet = Igmp_packet::New();

    // Convert 224.0.0.1 to SockAddr
    let dest_addr = SocketAddrV4::new(
        Ipv4Addr::new(IGMP_ALL_HOSTS[0], IGMP_ALL_HOSTS[1], IGMP_ALL_HOSTS[2], IGMP_ALL_HOSTS[3]),
        0
    );
    let sock_addr = SockAddr::from(dest_addr);

    socket.send_to(&igmp_packet.serialize(), &sock_addr)
}