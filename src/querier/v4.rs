//! Querier state machine and election logic.

use std::io::Error;
use std::net::Ipv4Addr;
use std::os::fd::{AsRawFd, OwnedFd};
use std::time::Instant;

use nix::sys::socket::{MsgFlags, SockaddrIn, recv, sendto};

use crate::config::QUERY_INTERVAL;
use crate::packet::igmp::{IgmpPacket, get_ip4_from_query};

/// State for managing querier election and query scheduling
#[derive(Debug)]
pub struct QuerierV4State {
    /// Our local IP on the interface
    local_ip: Ipv4Addr,
    /// Whether we currently believe we are the querier
    am_i_the_querier: bool,
    /// Last time we sent a general query
    last_query_sent: Option<Instant>,
}

impl QuerierV4State {
    /// Creates a new querier state for the given local IP address
    pub fn new(local_ip: Ipv4Addr) -> Self {
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
        
        let src_ip = get_ip4_from_query(query_data);

        if let Some(ip) = src_ip {
            if ip < self.local_ip {
                self.am_i_the_querier = false;
            }
        }
    }

    /// Checks if it's time to send a query
    ///
    /// Returns true if we are the querier and enough time has elapsed
    pub fn should_send_query(&self) -> bool {
        let time_has_passed = self.last_query_sent.is_some_and(|t| t.elapsed() >= QUERY_INTERVAL);

        time_has_passed && self.am_i_the_querier
    }

    /// Marks that we just sent a query
    pub fn mark_query_sent(&mut self) {
        self.last_query_sent = Some(Instant::now());
    }

    pub fn start(&mut self, fd: &OwnedFd) {
        loop {
            let mut buffer = Vec::new();
            recv(fd.as_raw_fd(), &mut buffer, MsgFlags::empty())
                .inspect(|_| self.handle_received_query(&buffer));

            if self.should_send_query() {
                match send_igmp_packet(fd) {
                    Ok(_) => self.mark_query_sent(),
                    Err(e) => eprintln!("Failed to send IGMP query: {}", e)
                }
            }
        }
    }
}

fn send_igmp_packet(fd: &OwnedFd) -> Result<(), Error> {
    let igmp_packet = IgmpPacket::new();

    sendto(fd.as_raw_fd(), &igmp_packet.serialize(), &SockaddrIn::new(224, 0, 0, 1, 0), MsgFlags::empty())?;

    Ok(())
}