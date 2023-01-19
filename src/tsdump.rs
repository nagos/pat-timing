use std::io::Read;
use std::iter::Iterator;
use std::fs::File;

const BLOCK_SIZE: usize = 7;
const HEADER_SIZE: usize = 4;
const TS_OFFSET: usize = BLOCK_SIZE*HEADER_SIZE;
const MAX_TS: i32  = 0x7FFFFFF;

pub struct TsDump {
    fh: File,
}

#[derive(Debug)]
pub struct TsBlock {
    pub ts: u32,
}

impl Iterator for TsDump {
    type Item = TsBlock;
    fn next(&mut self) -> Option<Self::Item> {
        let data: &mut [u8] = &mut [0; BLOCK_SIZE*HEADER_SIZE+4];
        match self.fh.read_exact(data){
            Ok(_) => Some(self.parse_block(data)),
            Err(_) => None,
        }
    }
}

fn extract_ts(data: &[u8]) -> u32 {
    let ts_data = &data[TS_OFFSET..TS_OFFSET+4];
     (ts_data[3] as u32) <<0
        | (ts_data[2] as u32) <<8
        | (ts_data[1] as u32) <<16
        | (ts_data[0] as u32) <<24
}

fn ts_diff(ts1: u32, ts2: u32) -> i32 {
    let d = ts1 as i32 - ts2 as i32;
    if d > (MAX_TS+1)/2 {
        return d as i32 -(MAX_TS+1) as i32;
    }
    if d < -((MAX_TS+1)/2) {
        return d - (MAX_TS+1);
    }
    return d;
}

impl TsDump {
    fn parse_block(&self, data: &[u8]) -> TsBlock {
        let ts = extract_ts(data);
        TsBlock { 
            ts,
         }
    }

    fn extract_ts(&self, data: &[u8]) -> u32 {
        let ts_data = &data[TS_OFFSET..TS_OFFSET+4];
         (ts_data[3] as u32) <<0
            | (ts_data[2] as u32) <<8
            | (ts_data[1] as u32) <<16
            | (ts_data[0] as u32) <<24
    }
}

impl TsDump {
    pub fn build(fname: &str) -> TsDump {
        let fh = File::open(fname).unwrap();
        TsDump{fh}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn block_parse() {
        let mut i = TsDump::build("testdata/record_1.ts");
        dbg!(i.next());
        dbg!(i.next());
        dbg!(i.next());
    }

    #[test]
    fn ts_diff_test() {
        let t1 :u32 = 123;
        let t2 :u32 = 23;
        let t3 :u32 = (MAX_TS-1) as u32;
        let d1 = ts_diff(t1, t2);
        assert_eq!(d1, 100);
        let d2 = ts_diff(t2, t1);
        assert_eq!(d2, -100);
        let d3 = ts_diff(t3, t2);
        assert_eq!(d3, -25);
    }
}
