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
    // TODO
    None
}