use clap::Parser;

/// A no-dependency RDMA connection manager for academic research
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// UDP port to listen on
    #[clap(short, long, default_value_t = 3369)]
    port: u16,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.port);
}
