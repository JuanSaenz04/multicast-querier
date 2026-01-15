//! Querier state machine and election logic.

use std::io::Error;
use std::net::{Ipv6Addr, SocketAddrV6};
use std::os::fd::{AsRawFd, OwnedFd};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use nix::sys::socket::{MsgFlags, SockaddrIn6, recv, sendto};

use crate::config::{MLD_ALL_NODES, QUERY_INTERVAL};
use crate::packet::mld::{MldQueryPacket, get_ip6_from_query};

/// State for managing querier election and query scheduling
#[derive(Debug)]
pub struct QuerierV6State {
    /// Our local IP on the interface
    local_ip: Ipv6Addr,
    /// Whether we currently believe we are the querier
    am_i_the_querier: bool,
    /// Last time we sent a general query
    last_query_sent: Option<Instant>,
}

impl QuerierV6State {
    /// Creates a new querier state for the given local IP address
    pub fn new(local_ip: Ipv6Addr) -> Self {
        Self {
            local_ip,
            am_i_the_querier: true, // Assume we're the querier initially
            last_query_sent: None,
        }
    }

    /// Handles receiving a query from another querier
    ///
    /// Returns true if we should back off (other querier has lower IP)
    pub fn handle_received_query(&mut self, query_data: &[u8]) {
        
        let src_ip = get_ip6_from_query(query_data);

        if let Some(ip) = src_ip {
            println!("Received query from {}", ip);
            if ip < self.local_ip {
                if self.am_i_the_querier {
                    println!("Lower IP querier detected, backing off...");
                }
                self.am_i_the_querier = false;
            }
        }
    }

    /// Checks if it's time to send a query
    ///
    /// Returns true if we are the querier and enough time has elapsed
    pub fn should_send_query(&self) -> bool {
        let time_has_passed = self.last_query_sent.map_or(true, |t| t.elapsed() >= QUERY_INTERVAL);

        time_has_passed && self.am_i_the_querier
    }

    /// Marks that we just sent a query
    pub fn mark_query_sent(&mut self) {
        self.last_query_sent = Some(Instant::now());
    }

    pub fn start(&mut self, fd: &OwnedFd, running: Arc<AtomicBool>) {
        let mut buffer = [0u8; 1500];
        while running.load(Ordering::SeqCst) {
            match recv(fd.as_raw_fd(), &mut buffer, MsgFlags::empty()) {
                Ok(n) => self.handle_received_query(&buffer[..n]),
                Err(nix::errno::Errno::EAGAIN) => {}
                Err(e) => eprintln!("Failed to receive packet: {}", e),
            }

            if self.should_send_query() {
                match send_mld_packet(fd) {
                    Ok(_) => self.mark_query_sent(),
                    Err(e) => eprintln!("Failed to send MLD query: {}", e)
                }
            }
        }
    }
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