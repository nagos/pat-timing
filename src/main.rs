use std::env;
use pat_timing::tsdump;

const PAT_PID: u32 = 0;

fn check_pair((a, b): (pat_timing::PacketData, pat_timing::PacketData)) {
    let diff = tsdump::ts_diff(a.0, b.0);
    dbg!(diff);
}

fn filter_data(x: &pat_timing::PacketData) -> bool {
    if x.1 == PAT_PID && x.2 == 0 {
        true
    } else {
        false
    }
}

fn main() {
    let mut args = env::args();
    args.next();

    let fname1 = args.next().unwrap();
    let fname2 = args.next().unwrap();

    let it1 = tsdump::TsDump::build(&fname1).flat_map(pat_timing::block_process).filter(filter_data);
    let it2 = tsdump::TsDump::build(&fname2).flat_map(pat_timing::block_process).filter(filter_data);

    it1.zip(it2).for_each(check_pair);
}
