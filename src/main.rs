use clap::Parser;
use std::process;

use rdmgr::{Args, run_main};

fn main() {
    let args = Args::parse();
    println!("Accepting client requests at UDP port {} / TCP port {} ...", args.udpport, args.tcpport);
    
    // run_main: executes the main logic
    if let Err(err) = run_main(args) {
        eprintln!("Error: {:?}", err);
        process::exit(-1);
    }
}
