//! IPv4/IGMP raw socket creation and configuration.

use std::{ffi::OsString, io::Error, os::fd::OwnedFd};

use nix::sys::socket::{
    AddressFamily, SockFlag, SockProtocol, SockType, setsockopt, socket, sockopt::{self, BindToDevice, IpMulticastTtl, Ipv4Ttl}
};

use crate::config::InterfaceConfig;

pub fn create_igmp_socket(config: &InterfaceConfig) -> Result<OwnedFd, Error> {
    // Create raw socket with IGMP protocol
    // For SOCK_RAW, we can pass None as the protocol since nix doesn't have SockProtocol::Igmp
    // The kernel will handle IGMP packets on this raw socket
    let fd = socket(AddressFamily::Inet, SockType::Raw, SockFlag::empty(), Some(SockProtocol::NetlinkUserSock))?;

    // Bind socket to specific network interface (SO_BINDTODEVICE)
    setsockopt(&fd, BindToDevice, &OsString::from(&config.name))?;

    // Set multicast interface
    setsockopt(&fd, IpMulticastTtl, &1)?;

    Ok(fd)
}