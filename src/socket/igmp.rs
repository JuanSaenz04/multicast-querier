//! IPv4/IGMP raw socket creation and configuration.

use std::{ffi::OsString, io::Error, os::fd::{AsRawFd, OwnedFd}};

use nix::sys::{socket::{
    AddressFamily, SockFlag, SockProtocol, SockType, setsockopt, socket, sockopt::{BindToDevice, IpMulticastTtl, ReceiveTimeout, IpMulticastLoop}
}, time::TimeVal};

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

    // Enable multicast loopback so we can hear our own queries
    setsockopt(&fd, IpMulticastLoop, &true)?;

    // Set recieve timeout to 1 second
    setsockopt(&fd, ReceiveTimeout, &TimeVal::new(1, 0))?;

    // Set IPv4 Router Alert Option
    let router_alert: &[u8] = &[
        0x94, // Type: Router Alert (148)
        0x04, // Length: 4 bytes
        0x00, 0x00 // Value: Router shall examine packet
    ];

    unsafe {
        use libc::{setsockopt, IPPROTO_IP, IP_OPTIONS, c_void, socklen_t};

        let ret = setsockopt(
            fd.as_raw_fd(),
            IPPROTO_IP,
            IP_OPTIONS,
            router_alert.as_ptr() as *const c_void,
            router_alert.len() as socklen_t,
        );

        if ret < 0 {
            return Err(Error::last_os_error());
        }
    }

    Ok(fd)
}