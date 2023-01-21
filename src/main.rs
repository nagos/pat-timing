use std::env;
use pat_timing::tsdump;

fn check_pair((a, b): (pat_timing::PacketData, pat_timing::PacketData)) {
    let diff = tsdump::ts_to_us(tsdump::ts_diff(a.0, b.0));
    println!("{}", diff);
}

fn main() {
    let mut args = env::args();
    args.next();

    let fname1 = args.next().unwrap();
    let fname2 = args.next().unwrap();

    let it1 = tsdump::TsDump::build(&fname1).flat_map(pat_timing::block_process).filter(pat_timing::filter_data);
    let it2 = tsdump::TsDump::build(&fname2).flat_map(pat_timing::block_process).filter(pat_timing::filter_data);

    it1.zip(it2).for_each(check_pair);
}
