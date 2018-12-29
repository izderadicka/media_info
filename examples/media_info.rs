extern crate media_info;

use std::env;
use std::process::exit;
use media_info::MediaFile;

fn main() {
    media_info::init();
    let args: Vec<_> =  env::args().collect();
    if args.len() < 2 {
        eprintln!("Must provide file path as param");
        exit(1);
    }

    let fname = &args[1];

    let mf = MediaFile::open(fname).expect(&format!("Cannot open file {}", fname));
    println!("File {} has duration {} ms and bitrate {} kbps", fname, mf.duration(), mf.bitrate());




}