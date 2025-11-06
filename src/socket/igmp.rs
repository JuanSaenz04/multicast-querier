//! IPv4/IGMP raw socket creation and configuration.

use std::io::Error;
use std::net::Ipv4Addr;
use std::os::fd::AsRawFd;
use socket2::{Domain, Protocol, Socket, SockAddr, Type};

use crate::config::InterfaceConfig;

pub fn create_igmp_socket(config: &InterfaceConfig) -> Result<Socket, Error> {
    // Create raw socket with IGMP protocol
    // Type::from_raw() lets us use SOCK_RAW (3) directly
    let socket =
        Socket::new(
            Domain::IPV4,
            Type::from(3),
            Some(Protocol::from(2))
        )?;

    // Bind to the specific interface by name using SO_BINDTODEVICE
    // This is a Linux-specific socket option
    unsafe {
        let device_name = config.name.as_bytes();
        let ret = libc::setsockopt(
            socket.as_raw_fd(),
            libc::SOL_SOCKET,
            libc::SO_BINDTODEVICE,
            device_name.as_ptr() as *const libc::c_void,
            device_name.len() as libc::socklen_t,
        );
        if ret < 0 {
            return Err(Error::last_os_error());
        }
    }
    
    // Bind to INADDR_ANY (0.0.0.0) on this interface
    let addr = SockAddr::from(std::net::SocketAddrV4::new(
        Ipv4Addr::UNSPECIFIED,
        0
    ));
    socket.bind(&addr)?;

    Ok(socket)
}