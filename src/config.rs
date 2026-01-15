//! Configuration constants and types for the multicast querier.

use std::{net::{Ipv4Addr, Ipv6Addr}, time::Duration};

/// How often to send general queries (125 seconds per IGMPv3/MLDv2 spec)
pub const QUERY_INTERVAL: Duration = Duration::from_secs(125);

/// MLD All-Nodes multicast address (ff02::1)
pub const MLD_ALL_NODES: [u8; 16] = [
    0xff, 0x02, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0x01
];

/// Configuration for monitoring a specific interface
#[derive(Debug)]
pub struct InterfaceConfig {
    /// Interface name (e.g., "eth0")
    pub name: String,
    /// Interface IPv4 address
    pub ipv4_addresses: Vec<Ipv4Addr>,
    /// Interface IPv6 address
    pub ipv6_addresses: Vec<Ipv6Addr>,
    /// Enable IGMPv3 querying on this interface
    pub enable_igmp: bool,
    /// Enable MLDv2 querying on this interface
    pub enable_mld: bool,
}
