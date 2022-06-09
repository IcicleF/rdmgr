use clap::Parser;
use std::net::{UdpSocket, TcpListener};
use std::thread;

mod network;
use crate::network::{netif, udpreq, tcpreq};

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


fn tcp_listener(port: u16) {
    let listener = TcpListener::bind(("0.0.0.0", port));
    if let Err(err) = listener {
        eprintln!("cannot bind to TCP port {}: {:?}", port, err);
        return;
    }
    let listener = listener.unwrap();
    
    for stream in listener.incoming() {
        // TODO: handle incoming connection
    }
}

pub fn run_main(args: Args) -> Result<(), Error> {
    // Initialize basic metadata
    // TODO: ask user to specify subnet instead of hardcoding here
    let netif_addrs = netif::get_netif_ipv4addrs(|addr| addr.octets()[..2] == [10u8, 0u8])?;
    if netif_addrs.len() == 0 {
        return Err(Error::NetInterfaceError(netif::Error::NoMatch));
    }
    let netif_ip = netif_addrs[0];

    // Spawn TCP handler thread; UDP handler is the main thread
    let tcpport = args.tcpport;
    thread::spawn(move || tcp_listener(tcpport));

    // Setup UDP socket
    let socket = UdpSocket::bind(("0.0.0.0", args.udpport))?;
    
    // Limit UDP packet to one Ethernet data frame
    const UDP_PKT_SIZE: usize = 1472;
    let mut rbuf = [0; UDP_PKT_SIZE];
    let mut sbuf = [0; UDP_PKT_SIZE];

    // Repeatedly listen for incoming UDP packets
    let handler = udpreq::UdpRequestHandler::new(netif_ip, args.tcpport);
    loop {
        let (rlen, src) = socket.recv_from(&mut rbuf)?;
        let slen = match handler.handle(&rbuf[..rlen], &mut sbuf) {
            Ok(len) => len,
            Err(err) => {
                eprintln!("unexpected UDP packet: {:?}", err);
                continue;
            }
        };
        socket.send_to(&sbuf[..slen], &src)?;
    }
}