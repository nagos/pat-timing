use std::io::Read;
use std::iter::Iterator;
use std::fs::File;

pub const BLOCK_SIZE: usize = 7;
const HEADER_SIZE: usize = 4;
const TS_OFFSET: usize = BLOCK_SIZE*HEADER_SIZE;
const MAX_TS: i32  = 0x7FFFFFF;
const BLOCK_LEN: usize = BLOCK_SIZE*HEADER_SIZE+4;

pub struct TsDump {
    fh: File,
    ts_init: bool,
    ts: u32,
    ts_prev: u32,
}

#[derive(Debug)]
pub struct TsBlock {
    pub ts: u32,
    data: Vec<u8>,
}

impl Iterator for TsDump {
    type Item = TsBlock;
    fn next(&mut self) -> Option<Self::Item> {
        let mut data = vec![0; BLOCK_LEN];
        match self.fh.by_ref().read_exact(&mut data){
            Ok(_) => {
                Some(self.parse_block(data))
            },
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

pub fn ts_diff(ts1: u32, ts2: u32) -> i32 {
    let d = ts1 as i32 - ts2 as i32;
    if d > (MAX_TS+1)/2 {
        return d as i32 -(MAX_TS+1) as i32;
    }
    if d < -((MAX_TS+1)/2) {
        return d - (MAX_TS+1);
    }
    return d;
}

pub fn ts_to_us(ts: i32) -> i32 {
    ts / 75
}

impl TsDump {
    fn parse_block(&mut self, data: Vec<u8>) -> TsBlock {
        let ts = extract_ts(data.as_slice());
        if self.ts_init {
            self.ts = ts;
            self.ts_prev = ts;
            self.ts_init = false;
        }
        self.ts = self.ts.overflowing_add(ts_diff(ts, self.ts_prev) as u32).0 & (MAX_TS as u32);
        self.ts_prev = ts;
        TsBlock { 
            ts: self.ts,
            data,
         }
    }
}

impl TsDump {
    pub fn build(fname: &str) -> TsDump {
        let fh = File::open(fname).unwrap();
        TsDump{
            fh,
            ts_prev: 0,
            ts_init: true,
            ts: 0,
        }
    }
}

impl TsBlock {
    pub fn packet(&self, i: usize) -> &[u8] {
        let start = HEADER_SIZE*i;
        let end = start+HEADER_SIZE;
        &self.data.as_slice()[start..end]
    }
}

impl Default for TsBlock {
    fn default() -> Self {
        TsBlock { 
            ts: 0, 
            data: vec![0; BLOCK_LEN]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn block_parse() {
        let mut i = TsDump::build("testdata/record_1.ts");
        let q = i.next().unwrap();
        assert_eq!(q.packet(0), [71, 0, 18, 20]);
        assert_eq!(q.packet(1), [71, 21, 55, 22]);
        assert_eq!(q.ts, 115239864);
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

        let t4: u32 = 2801941399;
        let t5: u32 = 2802805678;
        let d4 = ts_diff(t4, t5);
        assert_eq!(d4, -864279);
    }

    #[test]
    fn ts_to_us_test() {
        let v = ts_to_us(-865109);
        assert_eq!(v, -11534);
    }
}
