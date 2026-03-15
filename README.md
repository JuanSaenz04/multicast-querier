# Multicast Querier

A lightweight Rust tool that acts as an IGMP (IPv4) and MLD (IPv6) querier on specified network interfaces. It helps maintain multicast group state in a network by periodically sending queries to discover active multicast listeners.

## Features

- **IGMPv3 Support**: Automatically starts an IGMP querier if an IPv4 address is present on the interface.
- **MLDv2 Support**: Automatically starts an MLD querier if an IPv6 link-local address is present on the interface.
- **Multi-interface**: Can monitor and query multiple interfaces simultaneously.
- **Automatic Detection**: Detects available IP addresses and enables the appropriate protocol(s) accordingly.
- **Graceful Shutdown**: Handles termination signals (Ctrl+C) to clean up resources.

## Prerequisites

- **Rust**: Version 1.75 or later (Edition 2024 is used).
- **Permissions**: Sending raw multicast queries usually requires root/administrator privileges (e.g., `sudo`). Alternatively, you can grant the necessary capabilities to the binary:
  ```bash
  sudo setcap cap_net_raw+ep target/release/multicast-querier
  ```

## Building

To build the project from source:

```bash
cargo build --release
```

The compiled binary will be located at `target/release/multicast-querier`.

## Usage

Run the tool by specifying one or more interface names as arguments:

```bash
sudo ./target/release/multicast-querier <interface1> [interface2] ...
```

### Example

```bash
sudo ./target/release/multicast-querier eth0 wlan0
```

## How it Works

1. **Interface Initialization**: For each provided interface name, the tool looks up available IPv4 and IPv6 link-local addresses.
2. **Querier Threads**: It spawns dedicated threads for IGMP and MLD based on the found addresses.
3. **Query Cycle**:
   - Sends General Queries at a regular interval (default: 125 seconds).
   - Listens for queries from other routers.
   - If another querier with a lower IP address is detected, it may temporarily stop querying to avoid conflicts (following standard IGMP/MLD elections).

## Configuration

Core intervals and settings are defined in `src/config.rs`:

- `QUERY_INTERVAL`: How often to send general queries (default: 125s).
- `OTHER_QUERIER_PRESENT_INTERVAL`: Timeout before taking over as querier if no other queries are seen (default: 255s).

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
