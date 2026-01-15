//! IGMPv3 packet parsing and construction.

use std::net::Ipv4Addr;


pub struct IgmpPacket {
    igmp_type: u8,
    max_resp_code: u8,
    checksum: u16,
    group_address: u32,
    resv_s_qrv: u8,  // 4 bits reserved, 1 bit S flag, 3 bits QRV
    qqic: u8,
    num_sources: u16
}

fn calculate_checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;

    // Sum up all 16-bit words
    for chunk in data.chunks(2) {
        let word = if chunk.len() == 2 {
            u16::from_be_bytes([chunk[0], chunk[1]]) as u32
        } else {
            // If odd number of bytes, pad with zero
            (chunk[0] as u32) << 8
        };
        sum += word;
    }

    // Add carry bits back into the sum
    while (sum >> 16) > 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    // Return one's complement
    !sum as u16
}

impl IgmpPacket {
    pub fn new() -> IgmpPacket {
        let mut packet = IgmpPacket {
            igmp_type: 0x11,          // Membership Query
            max_resp_code: 100,       // 10 seconds (in 1/10th second units)
            group_address: 0,         // 0.0.0.0 = general query
            checksum: 0,
            resv_s_qrv: 0x02,        // Reserved=0, S=0, QRV=2 (robustness)
            qqic: 125,               // Query Interval = 125 seconds
            num_sources: 0           // General query has 0 sources
        };

        packet.checksum = calculate_checksum(&packet.serialize());

        packet
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(self.igmp_type);
        buffer.push(self.max_resp_code);
        buffer.extend_from_slice(&self.checksum.to_be_bytes());
        buffer.extend_from_slice(&self.group_address.to_be_bytes());
        // IGMPv3-specific fields
        buffer.push(self.resv_s_qrv);
        buffer.push(self.qqic);
        buffer.extend_from_slice(&self.num_sources.to_be_bytes());

        buffer
    }
}

/// If the raw data corresponds to an IGMP query, it returns the source IP from it.
/// Otherwise, it returns None.
pub fn get_ip4_from_query(data: &[u8]) -> Option<Ipv4Addr> {
    // Need at least 20 bytes for IP header + 12 bytes for minimum IGMPv3 query
    if data.len() < 32 {
        return None;
    }

    // Check IP version (should be 4)
    let version = (data[0] >> 4) & 0x0F;
    if version != 4 {
        return None;
    }

    // Get IP header length in bytes (IHL is in 32-bit words)
    let ihl = (data[0] & 0x0F) as usize;
    let ip_header_len = ihl * 4;

    // Validate IP header length is reasonable
    if ip_header_len < 20 || data.len() < ip_header_len + 12 {
        return None;
    }

    // Check protocol field (byte 9) - should be 2 for IGMP
    if data[9] != 2 {
        return None;
    }

    // Extract source IP address from bytes 12-15 of IP header
    let src_ip = Ipv4Addr::new(data[12], data[13], data[14], data[15]);

    // Check if the IGMP payload is a query (type 0x11)
    let igmp_type = data[ip_header_len];
    if igmp_type != 0x11 {
        return None;
    }

    Some(src_ip)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ip4_from_query_valid() {
        let mut packet = Vec::new();

        // IP Header
        // Version 4, IHL 5
        packet.push(0x45);
        // TOS
        packet.push(0);
        // Total Length (placeholder)
        packet.extend_from_slice(&[0, 0]);
        // ID
        packet.extend_from_slice(&[0, 0]);
        // Flags/Fragment Offset
        packet.extend_from_slice(&[0, 0]);
        // TTL
        packet.push(1);
        // Protocol: 2 (IGMP)
        packet.push(2);
        // Header Checksum (placeholder)
        packet.extend_from_slice(&[0, 0]);
        // Source IP: 192.168.1.50
        packet.extend_from_slice(&[192, 168, 1, 50]);
        // Dest IP: 224.0.0.1
        packet.extend_from_slice(&[224, 0, 0, 1]);

        // IGMPv3 Query
        // Type: 0x11 (Membership Query)
        packet.push(0x11);
        // Max Resp Code
        packet.push(100);
        // Checksum
        packet.extend_from_slice(&[0, 0]);
        // Group Address (0.0.0.0)
        packet.extend_from_slice(&[0, 0, 0, 0]);
        // Resv/S/QRV
        packet.push(0);
        // QQIC
        packet.push(0);
        // Num Sources
        packet.extend_from_slice(&[0, 0]);

        let result = get_ip4_from_query(&packet);
        assert_eq!(result, Some("192.168.1.50".parse().unwrap()));
    }

    #[test]
    fn test_get_ip4_from_query_invalid_type() {
        let mut packet = Vec::new();

        // IP Header
        packet.push(0x45); // Version 4, IHL 5
        packet.push(0); // TOS
        packet.extend_from_slice(&[0, 0]); // Total Length
        packet.extend_from_slice(&[0, 0]); // ID
        packet.extend_from_slice(&[0, 0]); // Flags/Fragment Offset
        packet.push(1); // TTL
        packet.push(2); // Protocol: 2 (IGMP)
        packet.extend_from_slice(&[0, 0]); // Header Checksum
        packet.extend_from_slice(&[192, 168, 1, 50]); // Source IP
        packet.extend_from_slice(&[224, 0, 0, 22]); // Dest IP (IGMPv3 Report)

        // IGMPv3 Membership Report (Type 0x22) - NOT a query (0x11)
        packet.push(0x22); // Type: 0x22 (v3 Membership Report)
        packet.push(0); // Reserved
        packet.extend_from_slice(&[0, 0]); // Checksum
        packet.extend_from_slice(&[0, 0]); // Reserved
        packet.extend_from_slice(&[0, 0]); // Number of Group Records

        let result = get_ip4_from_query(&packet);
        assert_eq!(result, None);
    }
}
