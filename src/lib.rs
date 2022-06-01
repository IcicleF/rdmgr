use clap::Parser;
use std::net::{IpAddr, UdpSocket};

mod network;
use crate::network::{udppkt, netif};

/// An RDMA connection manager for academic research use
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to RDMA configuration file
    #[clap(short, long)]
    pub config: Option<String>,

    /// UDP port to listen on
    #[clap(short, long, default_value_t = 3369)]
    pub udpport: u16,

    /// TCP port to listen on
    #[clap(short, long, default_value_t = 3370)]
    pub tcpport: u16,
}

#[derive(Debug)]
pub enum Error {
    NetInterfaceError(netif::Error),
    IOError(std::io::Error),
}

impl From<netif::Error> for Error {
    fn from(err: netif::Error) -> Self {
        Error::NetInterfaceError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}

pub fn run_main(args: Args) -> Result<(), Error> {
    // Initialize basic metadata
    // TODO: ask user to specify subnet instead of hardcoding here
    let netif_addrs = netif::get_netif_ipv4addrs(|addr| addr.octets()[0..2] == [10u8, 0u8])?;
    if netif_addrs.len() == 0 {
        return Err(Error::NetInterfaceError(netif::Error::NoMatch));
    }
    let netif_ip = netif_addrs[0];

    // Setup UDP socket
    let socket = UdpSocket::bind(("0.0.0.0", args.udpport))?;
    
    // Limit UDP packet to one Ethernet data frame
    const UDP_PKT_SIZE: usize = 1472;
    let mut rbuf = [0; UDP_PKT_SIZE];
    let mut sbuf = [0; UDP_PKT_SIZE];

    // Repeatedly listen for incoming UDP packets
    let parser = udppkt::UdpPacketHandler::new(netif_ip);
    loop {
        let (rlen, src) = socket.recv_from(&mut rbuf)?;
        let slen = match parser.handle(&rbuf[..rlen], &mut sbuf) {
            Ok(len) => len,
            Err(err) => {
                println!("unexpected UDP packet: {:?}", err);
                continue;
            }
        };
        socket.send_to(&sbuf[..slen], &src)?;
    }
}