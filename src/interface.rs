//! Per-interface thread logic and main loop.

use nix::ifaddrs::getifaddrs;

use crate::{config::InterfaceConfig, querier, socket::{igmp::create_igmp_socket, mld::create_mld_socket}};
use std::{error::Error, net::{Ipv4Addr, Ipv6Addr}, thread::{self}};

/// Main function executed by each interface thread
///
pub fn run_interface_thread(interface_name: String) -> Result<(), Box<dyn Error>> {
    let config = create_interface_config(interface_name)?;

    println!("Starting interface thread for: {}", config.name);

    // TODO: Determine local IP address for this interface
    // TODO: Create IGMP socket (if config.enable_igmp)
    // TODO: Create MLD socket (if config.enable_mld)
    // TODO: Initialize QuerierState for IPv4 and/or IPv6
    // TODO: Set socket read timeout to SOCKET_TIMEOUT

    if config.enable_igmp {
        let fd4 = create_igmp_socket(&config)?;

        let mut v4_querier = querier::v4::QuerierV4State::new(config.ipv4_addresses[0]);

        let v4_handle = thread::spawn(move || {
            v4_querier.start(&fd4)
        });
    }

    if config.enable_mld {
        let fd6 = create_mld_socket(&config)?;

        let mut v6_querier = querier::v6::QuerierV6State::new(config.ipv6_addresses[0]);

        let v6_handle = thread::spawn(move || {
            v6_querier.start(&fd6);
        });
    }

    Ok(())

    // Unreachable in normal operation
}

fn create_interface_config(interface_name: String) -> Result<InterfaceConfig, Box<dyn Error>> {
    let ifaddrs = getifaddrs()?;

    let mut ipv4_addrs = Vec::new();
    let mut ipv6_addrs = Vec::new();

    for ifaddr in ifaddrs {
        if ifaddr.interface_name == interface_name {
            if let Some(sock_addr) = ifaddr.address {
                if let Some(inet4) = sock_addr.as_sockaddr_in() {
                    ipv4_addrs.push(Ipv4Addr::from(inet4.ip()));
                }
                else if let Some(inet6) = sock_addr.as_sockaddr_in6() {
                    ipv6_addrs.push(Ipv6Addr::from(inet6.ip()));
                }
            }
        }
    }

    Ok(InterfaceConfig{
        name: interface_name,
        enable_igmp: !ipv4_addrs.is_empty(),
        enable_mld: !ipv6_addrs.is_empty(),
        ipv4_addresses: ipv4_addrs,
        ipv6_addresses: ipv6_addrs,
    })
}