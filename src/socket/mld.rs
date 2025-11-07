//! IPv6/MLD raw socket creation and configuration.

use std::{ffi::OsString, io::Error, os::fd::{AsRawFd, OwnedFd}};

use nix::sys::socket::{AddressFamily, SockFlag, SockProtocol, SockType, setsockopt, socket, sockopt::{BindToDevice, Ipv6MulticastHops}, };

use crate::config::InterfaceConfig;


pub fn create_mld_socket(config: &InterfaceConfig) -> Result<OwnedFd, Error> {
    let fd = socket(
        AddressFamily::Inet6,
        SockType::Raw,
        SockFlag::empty(),
        SockProtocol::IcmpV6
    )?;

    setsockopt(&fd, BindToDevice, &OsString::from(&config.name))?;
    
    setsockopt(&fd, Ipv6MulticastHops, &1)?;

    let hop_opts: &[u8] = &[
      58,   // Next Header: ICMPv6
      0,    // Hdr Ext Len: 0 (8 bytes total)
      0x05, // Router Alert Option Type
      0x02, // Option Data Length: 2 bytes
      0x00, // Router Alert Value: 0x0000 (MLD) - high byte
      0x00, // Router Alert Value: 0x0000 (MLD) - low byte
      0x01, // PadN option type
      0x00, // PadN length: 0 (total 2 bytes for padding)
    ];

    unsafe {
        use libc::{ setsockopt, IPPROTO_IPV6, IPV6_HOPOPTS, c_void, socklen_t };
        let ret = setsockopt(
            fd.as_raw_fd(),
            IPPROTO_IPV6,
            IPV6_HOPOPTS,
            hop_opts.as_ptr() as *const c_void,
            hop_opts.len() as socklen_t,
        );

        if ret < 0 {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(fd)
}