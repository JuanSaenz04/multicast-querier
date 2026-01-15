//! Per-interface thread logic and main loop.

use nix::ifaddrs::getifaddrs;

use crate::{config::InterfaceConfig, querier, socket::{igmp::create_igmp_socket, mld::create_mld_socket}};
use std::{error::Error, net::{Ipv4Addr, Ipv6Addr}, sync::{atomic::AtomicBool, Arc}, thread::{self}};

/// Main function executed for each interface
pub fn run_interface_thread(interface_name: String, running: Arc<AtomicBool>) -> Result<Vec<thread::JoinHandle<()>>, Box<dyn Error>> {
    println!("Starting on interface: {}", interface_name);

    let config = create_interface_config(interface_name)?;
    let mut handles = Vec::new();

    println!("Found IPs: {:?} {:?}", config.ipv4_addresses, config.ipv6_addresses);

    if config.enable_igmp {
        let fd4 = create_igmp_socket(&config)?;

        let mut v4_querier = querier::v4::QuerierV4State::new(config.ipv4_addresses[0]);
        let running_clone = running.clone();
        let v4_handle = thread::spawn(move || {
            v4_querier.start(&fd4, running_clone)
        });
        handles.push(v4_handle);

        println!("IGMP querier started on: {}", config.name);
    }

    if config.enable_mld {
        let fd6 = create_mld_socket(&config)?;

        let mut v6_querier = querier::v6::QuerierV6State::new(config.ipv6_addresses[0]);

        let v6_handle = thread::spawn(move || {
            v6_querier.start(&fd6, running)
        });
        handles.push(v6_handle);

        println!("MLD querier started on: {}", config.name);
    }

    Ok(handles)

}

/// Creates the interface configuration struct based on the interface name
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