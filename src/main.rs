
use std::io::Error;
use std::net::{Ipv6Addr, SocketAddrV6};
use std::os::fd::{AsRawFd, OwnedFd};

use nix::sys::socket::{MsgFlags, SockaddrIn, SockaddrIn6, sendto};
use nix::unistd::close;

use crate::config::{InterfaceConfig, MLD_ALL_NODES};
use crate::packet::igmp::IgmpPacket;
use crate::packet::mld::MldQueryPacket;
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

    match send_igmp_packet(&fd) {
        Ok(_) => println!("IGMP query sent successfully"),
        Err(e) => eprintln!("Failed to sent IGMP packet: {}", e)
    };

    // Create the MLDv2 socket
    let fd6 = match create_mld_socket(&config) {
        Ok(sock_fd) => sock_fd,
        Err(e) => {
            eprintln!("Failed to create MLDv2 socket: {}", e);
            return;
        }
    };

    match send_mld_packet(&fd6) {
        Ok(_) => println!("MLDv2 query sent successfully"),
        Err(e) => eprintln!("Failed to sent MLDv2 packet: {}", e)
    };
}

fn send_mld_packet(fd: &OwnedFd) -> Result<(), Error> {
    let mut mld_packet = MldQueryPacket::new();

    let src: [u8; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]; // Unspecified source
    let dst: [u8; 16] = MLD_ALL_NODES;

    // Calculate and set checksum
    mld_packet.calculate_checksum(&src, &dst);

    // Create IPv6 destination address (ff02::1, port 0)
    let ipv6_addr = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 1);
    let socket_addr = SocketAddrV6::new(ipv6_addr, 0, 0, 0);
    let dest_addr = SockaddrIn6::from(socket_addr);

    sendto(fd.as_raw_fd(), &mld_packet.serialize(), &dest_addr, MsgFlags::empty())?;

    Ok(())
}