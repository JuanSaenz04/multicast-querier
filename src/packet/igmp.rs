//! IGMPv3 packet parsing and construction.

use crate::packet;

pub struct Igmp_packet {
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

impl Igmp_packet {
    pub fn New() -> Igmp_packet {
        let mut packet = Igmp_packet {
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