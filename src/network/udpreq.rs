use std::net::Ipv4Addr;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
    ResponseTooLong(usize),
}

pub struct UdpRequestHandler {
    local_ip: Ipv4Addr,
    tcp_port: u16,
}

impl UdpRequestHandler {
    pub fn new(local_ip: Ipv4Addr, tcp_port: u16) -> UdpRequestHandler {
        UdpRequestHandler { local_ip, tcp_port }
    }

    fn handle_pkt_impl(&self, rbuf: &[u8], sbuf: &mut [u8]) -> Result<usize, Error> {
        const MAGIC_HDR: [u8; 4] = [0x2D, 0x0A, 0xA0, 0xD2];

        if rbuf[..4] != MAGIC_HDR {
            return Err(Error::ParseError(String::from("magic number invalid")));
        }

        // Fill in header
        for i in 0..4 {
            sbuf[i] = MAGIC_HDR[i];
        }
        
        // Fill in server IPv4 address
        let ip_octets = self.local_ip.octets();
        for i in 0..4 {
            sbuf[i + 4] = ip_octets[i];
        }

        // Fill in server TCP port (little-endian)
        sbuf[8] = (self.tcp_port & 0xFF) as u8;
        sbuf[9] = (self.tcp_port >> 8) as u8;

        Ok(10)
    }

    /// Handle a UDP packet request sent from a client.
    /// 
    /// # Request format
    /// +------------+-------------------------------------------------------------+
    /// | Bytes      | Usage                                                       |
    /// +------------+-------------------------------------------------------------+
    /// | 0 - 3      | Magic number for rdmgr (0x2D0AA0D2)                         |
    /// +------------+-------------------------------------------------------------+
    /// 
    /// # Response format
    /// 
    /// +------------+-------------------------------------------------------------+
    /// | Bytes      | Usage                                                       |
    /// +------------+-------------------------------------------------------------+
    /// | 0 - 3      | Magic number for rdmgr (0x2D0AA0D2)                         |
    /// | 4 - 7      | IPv4 address of rdmgr server                                |
    /// | 8 - 9      | Specified TCP port for client to connect                    |
    /// +------------+-------------------------------------------------------------+
    /// 
    /// # Examples
    /// ```
    /// let rbuf: [u8; 4] = [0x2D, 0x0A, 0xA0, 0xD2];
    /// let mut sbuf = [0u8; 10];
    /// let handler = UdpRequestHandler::new(Ipv4Addr::new(10, 0, 2, 175), 3370);
    /// 
    /// assert_eq!(handler.handle(&rbuf, &mut sbuf), Ok(10));
    /// assert_eq!(sbuf, [0x2D, 0x0A, 0xA0, 0xD2, 0x0A, 0x00, 0x02, 0xAF, 0x2A, 0x0D]);
    /// ```
    /// 
    pub fn handle(&self, rbuf: &[u8], sbuf: &mut [u8]) -> Result<usize, Error> {
        const RESP_SIZE: usize = 1472;
        match self.handle_pkt_impl(rbuf, sbuf) {
            Ok(size) => if size <= RESP_SIZE { Ok(size) } else { Err(Error::ResponseTooLong(size)) },
            Err(err) => Err(err),
        }
    }
}