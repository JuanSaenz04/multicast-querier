//! Querier state machine and election logic.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::Instant;

/// State for managing querier election and query scheduling
#[derive(Debug)]
pub struct QuerierState {
    /// Our own IP address on this interface
    local_ip: IpAddr,
    /// Whether we currently believe we are the querier
    am_i_the_querier: bool,
    /// Last time we sent a general query
    last_query_sent: Option<Instant>,
}

impl QuerierState {
    /// Creates a new querier state for the given local IP address
    pub fn new(local_ip: IpAddr) -> Self {
        Self {
            local_ip,
            am_i_the_querier: true, // Assume we're the querier initially
            last_query_sent: None,
        }
    }

    /// Handles receiving a query from another querier
    ///
    /// Returns true if we should back off (other querier has lower IP)
    pub fn handle_received_query(&mut self, source_ip: IpAddr) -> bool {
        // TODO: Implement IP comparison logic
        // If source_ip < our local_ip, we should back off
        // Update am_i_the_querier accordingly
        todo!("Implement querier election logic")
    }

    /// Checks if it's time to send a query
    ///
    /// Returns true if we are the querier and enough time has elapsed
    pub fn should_send_query(&self) -> bool {
        // TODO: Check if am_i_the_querier is true
        // TODO: Check if last_query_sent is None or elapsed >= QUERY_INTERVAL
        todo!("Implement query timing logic")
    }

    /// Marks that we just sent a query
    pub fn mark_query_sent(&mut self) {
        self.last_query_sent = Some(Instant::now());
    }

    /// Returns whether we currently believe we are the querier
    pub fn am_i_querier(&self) -> bool {
        self.am_i_the_querier
    }
}

/// Creates a new IPv4 querier state
pub fn new_ipv4_querier(local_ip: Ipv4Addr) -> QuerierState {
    QuerierState::new(IpAddr::V4(local_ip))
}

/// Creates a new IPv6 querier state
pub fn new_ipv6_querier(local_ip: Ipv6Addr) -> QuerierState {
    QuerierState::new(IpAddr::V6(local_ip))
}
