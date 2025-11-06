//! Per-interface thread logic and main loop.

use crate::config::InterfaceConfig;
use std::io;

/// Main function executed by each interface thread
///
/// This function contains the core loop that:
/// 1. Calls recv_from() with a short timeout
/// 2. Processes received queries (if any)
/// 3. Checks the independent timer
/// 4. Sends queries when appropriate
///
/// # Arguments
/// * `config` - Configuration for this interface
///
/// # Returns
/// Only returns if there's a fatal error
pub fn run_interface_thread(config: InterfaceConfig) -> io::Result<()> {
    println!("Starting interface thread for: {}", config.name);

    // TODO: Determine local IP address for this interface
    // TODO: Create IGMP socket (if config.enable_igmp)
    // TODO: Create MLD socket (if config.enable_mld)
    // TODO: Initialize QuerierState for IPv4 and/or IPv6
    // TODO: Set socket read timeout to SOCKET_TIMEOUT

    // Main loop structure:
    loop {
        // TODO: Call recv_from() on the socket(s) - this blocks for up to SOCKET_TIMEOUT

        // TODO: If we received a packet:
        //   - Parse it to extract source IP
        //   - Call querier_state.handle_received_query(source_ip)

        // TODO: If recv_from() timed out:
        //   - This is expected! It's our loop "tick"

        // TODO: After either case, check querier_state.should_send_query()
        //   - If true, construct and send IGMP/MLD General Query
        //   - Call querier_state.mark_query_sent()

        // TODO: Handle shutdown signal (e.g., from main thread)
    }

    // Unreachable in normal operation
}
