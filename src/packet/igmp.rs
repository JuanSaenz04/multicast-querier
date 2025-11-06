//! IGMPv3 packet parsing and construction.

use crate::packet;

pub struct Igmp_packet {
    igmp_type: u8,
    max_resp_code: u8,
    checksum: u16,
    group_address: u32
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

impl Igmp_packet {
    pub fn New() -> Igmp_packet {
        let mut packet = Igmp_packet {
            igmp_type: 0x11,
            max_resp_code: 100,
            group_address: 0,
            checksum: 0
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

        buffer
    }
}