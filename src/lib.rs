pub mod tsdump;

pub type PacketData = (u32, u32, u32);
const PAT_PID: u32 = 0;

fn parse_packet(packet: &[u8]) -> (u32, u32) {
    let pid = ((packet[1]&0x1f) as u32) << 8
        | (packet[2] as u32) ;
    let cc = packet[3]&0x0f;
    (pid, cc as u32)
}

pub fn block_process(block: tsdump::TsBlock) -> Vec<PacketData> {
    let mut ret = vec![];

    for i in 0..tsdump::BLOCK_SIZE {
        let (pid, cc) = parse_packet(block.packet(i));
        ret.push((block.ts, pid, cc));
    }
    ret
}

pub fn filter_data(x: &PacketData) -> bool {
    if x.1 == PAT_PID && x.2 == 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tsdump;

    const DATA: &[u8] = &[
            0x47, 0x00, 0x12, 0x14, 
            0x47, 0x15, 0x37, 0x16, 
            0x47, 0x15, 0x41, 0x12, 
            0x47, 0x15, 0x2d, 0x1f, 
            0x47, 0x00, 0xc9, 0x1f, 
            0x47, 0x0f, 0xe8, 0x1c, 
            0x47, 0x00, 0x65, 0x10, 
            0x06, 0xde, 0x6b, 0xb8,
        ];

    #[test]
    fn block_process_test() {
        let mut i = tsdump::TsDump::build(DATA);
        let p = i.next().unwrap();
        let d = block_process(p);
        assert_eq!(d[0], (115239864, 0x12, 4));
        assert_eq!(d[1], (115239864, 0x1537, 6));
    }
    
    #[test]
    fn block_iterator_test() {
        let i = tsdump::TsDump::build(DATA);
        let mut q = i.flat_map(block_process);
        let d1 = q.next().unwrap();
        assert_eq!(d1, (115239864, 0x12, 4));
        let d2 = q.next().unwrap();
        assert_eq!(d2, (115239864, 0x1537, 6));
    }
}
