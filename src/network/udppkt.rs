use std::net::Ipv4Addr;
use std::fmt;

pub struct Error(String);

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

pub struct UdpPacketHandler {
    local_ip: Ipv4Addr,
}

impl UdpPacketHandler {
    pub fn new(local_ip: Ipv4Addr) -> UdpPacketHandler {
        UdpPacketHandler { local_ip }
    }

    pub fn handle(&self, rbuf: &[u8], sbuf: &mut [u8]) -> Result<usize, Error> {
        Ok(0)
    }
}