//! Configuration constants and types for the multicast querier.

use std::time::Duration;

use socket2::Socket;

/// How often to send general queries (125 seconds per IGMPv3/MLDv2 spec)
pub const QUERY_INTERVAL: Duration = Duration::from_secs(125);

/// Socket read timeout for the main loop (1 second)
pub const SOCKET_TIMEOUT: Duration = Duration::from_secs(1);

/// IGMP All-Hosts multicast address (224.0.0.1)
pub const IGMP_ALL_HOSTS: [u8; 4] = [224, 0, 0, 1];

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
    /// Interface index
    pub index: usize,
    /// Enable IGMPv3 querying on this interface
    pub enable_igmp: bool,
    /// Enable MLDv2 querying on this interface
    pub enable_mld: bool,
}
