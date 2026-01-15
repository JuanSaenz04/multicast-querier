//! MLDv2 packet parsing and construction.

use std::net::Ipv6Addr;

pub struct MldQueryPacket {
    message_type: u8,
    code: u8,
    checksum: u16,
    max_response_code: u16,
    reserved: u16,
    multicast_address: [u8; 16],
    resv_s_qrv: u8,
    qqic: u8,
    number_of_sources: u16
}

const GENERAL_QUERY_IP6: [u8; 16] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

impl MldQueryPacket {
    pub fn new() -> MldQueryPacket {
        MldQueryPacket {
            message_type: 130,
            code: 0,
            checksum: 0,
            max_response_code: 10_000,
            reserved: 0,
            multicast_address: GENERAL_QUERY_IP6,
            resv_s_qrv: 0x02,
            qqic: 125,
            number_of_sources: 0
        }
    }

    /// Calculate ICMPv6 checksum including IPv6 pseudo-header
    ///
    /// # Arguments
    /// * `src` - Source IPv6 address (16 bytes)
    /// * `dst` - Destination IPv6 address (16 bytes)
    /// * `packet` - The ICMPv6 packet data (including header with checksum field set to 0)
    ///
    /// # Returns
    /// The calculated checksum value
    pub fn calculate_checksum(&mut self, src: &[u8; 16], dst: &[u8; 16]) {
        let packet = self.serialize();
        let mut sum: u32 = 0;

        // Add source address (16 bytes = 8 words)
        for i in (0..16).step_by(2) {
            sum += u32::from(u16::from_be_bytes([src[i], src[i + 1]]));
        }

        // Add destination address (16 bytes = 8 words)
        for i in (0..16).step_by(2) {
            sum += u32::from(u16::from_be_bytes([dst[i], dst[i + 1]]));
        }

        // Add upper-layer packet length (4 bytes)
        let packet_len = packet.len() as u32;
        sum += (packet_len >> 16) & 0xFFFF;  // High 16 bits
        sum += packet_len & 0xFFFF;           // Low 16 bits

        // Add zeros (3 bytes) - no contribution to sum

        // Add next header value (ICMPv6 = 58 = 0x3A)
        sum += 58;

        // Add packet data as 16-bit words
        let mut i = 0;
        while i < packet.len() {
            if i + 1 < packet.len() {
                sum += u32::from(u16::from_be_bytes([packet[i], packet[i + 1]]));
                i += 2;
            } else {
                // Odd byte - pad with zero
                sum += u32::from(u16::from_be_bytes([packet[i], 0]));
                i += 1;
            }
        }

        // Fold 32-bit sum into 16 bits
        while (sum >> 16) != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        // Return one's complement
        self.checksum = !sum as u16;
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.push(self.message_type);
        buffer.push(self.code);
        buffer.extend_from_slice(&self.checksum.to_be_bytes());
        buffer.extend_from_slice(&self.max_response_code.to_be_bytes());
        buffer.extend_from_slice(&self.reserved.to_be_bytes());
        buffer.extend_from_slice(&self.multicast_address);
        buffer.push(self.resv_s_qrv);
        buffer.push(self.qqic);
        buffer.extend_from_slice(&self.number_of_sources.to_be_bytes());

        buffer
    }
}

pub fn get_ip6_from_query(data: &[u8]) -> Option<Ipv6Addr> {
    if data.len() < 40 {
        return None;
    }

    // Check IP version (should be 6)
    let version = (data[0] >> 4) & 0x0F;
    if version != 6 {
        return None;
    }

    // Extract Source IP (bytes 8-24)
    let mut src_bytes = [0u8; 16];
    src_bytes.copy_from_slice(&data[8..24]);
    let src_ip = Ipv6Addr::from(src_bytes);

    let mut next_header = data[6];
    let mut current_offset = 40;

    // Traverse extension headers to find ICMPv6
    // We limit the loop to avoid infinite loops with malformed packets
    for _ in 0..10 {
        if current_offset >= data.len() {
            return None;
        }

        if next_header == 58 { // ICMPv6
            if current_offset + 1 > data.len() {
                return None;
            }
            // Check if it is a Multicast Listener Query (Type 130)
            if data[current_offset] == 130 {
                return Some(src_ip);
            }
            return None;
        }

        // Handle generic extension headers (Hop-by-Hop: 0, Routing: 43, DestOpts: 60)
        // These headers share the same structure: [NextHeader, HdrExtLen, ...data]
        match next_header {
            0 | 43 | 60 => {
                if current_offset + 2 > data.len() {
                    return None;
                }
                next_header = data[current_offset];
                // HdrExtLen is in 8-octet units, not including the first 8 octets
                let len = (data[current_offset + 1] as usize + 1) * 8;
                current_offset += len;
            }
            _ => return None, // Unknown or unhandled next header
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ip6_from_query_with_hbh() {
        let mut packet = Vec::new();

        // IPv6 Header
        // Version 6, Traffic Class 0, Flow Label 0
        packet.extend_from_slice(&[0x60, 0x00, 0x00, 0x00]);
        // Payload Length (will be set later, just placeholder)
        packet.extend_from_slice(&[0x00, 0x00]);
        // Next Header: 0 (Hop-by-Hop Options)
        packet.push(0);
        // Hop Limit
        packet.push(1);
        // Source IP: fe80::1
        packet.extend_from_slice(&[0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        // Dest IP: ff02::1 (All Nodes)
        packet.extend_from_slice(&[0xff, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

        // Hop-by-Hop Options Header
        // Next Header: 58 (ICMPv6)
        packet.push(58);
        // Hdr Ext Len: 0 ( (0 + 1) * 8 = 8 bytes total length)
        packet.push(0);
        // Padding (6 bytes to make it 8 bytes aligned)
        packet.extend_from_slice(&[0; 6]);

        // ICMPv6 MLD Query
        // Type: 130 (Multicast Listener Query)
        packet.push(130);
        // Code: 0
        packet.push(0);
        // Checksum (placeholder)
        packet.extend_from_slice(&[0, 0]);
        // Max Resp Code
        packet.extend_from_slice(&[0, 0]);
        // Reserved
        packet.extend_from_slice(&[0, 0]);
        // Multicast Address (::)
        packet.extend_from_slice(&[0; 16]);
        // Resv/S/QRV
        packet.push(0);
        // QQIC
        packet.push(0);
        // Num Sources
        packet.extend_from_slice(&[0, 0]);

        let result = get_ip6_from_query(&packet);
        assert_eq!(result, Some("fe80::1".parse().unwrap()));
    }

    #[test]
    fn test_get_ip6_from_query_not_a_query() {
        let mut packet = Vec::new();

        // IPv6 Header
        packet.extend_from_slice(&[0x60, 0x00, 0x00, 0x00]); // Version 6
        packet.extend_from_slice(&[0x00, 0x00]); // Payload Length
        packet.push(0); // Next Header: 0 (Hop-by-Hop Options)
        packet.push(1); // Hop Limit
        packet.extend_from_slice(&[0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]); // Source IP
        packet.extend_from_slice(&[0xff, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]); // Dest IP

        // Hop-by-Hop Options Header
        packet.push(58); // Next Header: 58 (ICMPv6)
        packet.push(0); // Hdr Ext Len: 0 (8 bytes)
        packet.extend_from_slice(&[0; 6]); // Padding

        // ICMPv6 Echo Request (Not a query)
        packet.push(128); // Type: 128 (Echo Request)
        packet.push(0); // Code: 0
        packet.extend_from_slice(&[0, 0]); // Checksum
        packet.extend_from_slice(&[0, 0]); // ID
        packet.extend_from_slice(&[0, 0]); // Sequence

        let result = get_ip6_from_query(&packet);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_ip6_from_query_real_packet() {
        let packet: Vec<u8> = vec![
            0x60, 0x0a, 0xf9, 0xff, 0x00, 0x24, 0x00, 0x01, // IPv6 Header start
            0xfe, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Src IP
            0x8c, 0xaa, 0xa4, 0xff, 0xfe, 0xbb, 0x71, 0x4b,
            0xff, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Dst IP
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x3a, 0x00, 0x05, 0x02, 0x00, 0x00, 0x01, 0x00, // HbH Options (Next: 3a/58)
            0x82, 0x00, 0xb4, 0xe5, 0x27, 0x10, 0x00, 0x00, // MLD Query (Type 82/130)
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x7d,
            0x00, 0x00
        ];

        let result = get_ip6_from_query(&packet);
        // Src IP: fe80::8caa:a4ff:febb:714b
        assert_eq!(result, Some("fe80::8caa:a4ff:febb:714b".parse().unwrap()));
    }
}