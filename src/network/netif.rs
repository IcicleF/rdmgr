use std::net::Ipv4Addr;
use nix;

#[derive(Debug)]
pub enum Error {
    InterfaceNotExist,
    IpAddrNotExist,
    NoMatch,
}

pub fn get_netif_ipv4addrs<F: Fn(&Ipv4Addr) -> bool>(filt: F) -> Result<Vec<Ipv4Addr>, Error> {
    let addrs = nix::ifaddrs::getifaddrs();
    match addrs {
        Err(_) => Err(Error::InterfaceNotExist),
        Ok(ifaddrs) => Ok(ifaddrs.filter(|x| x.address.is_some())
                            .map(|x| x.address.unwrap())
                            .filter(|x| x.as_sockaddr_in().is_some())
                            .map(|addr| {
                                let ip_inaddr = addr.as_sockaddr_in().unwrap().ip();
                                let s4 = (ip_inaddr & 0xFF) as u8;
                                let s3 = ((ip_inaddr >> 8) & 0xFF) as u8;
                                let s2 = ((ip_inaddr >> 16) & 0xFF) as u8;
                                let s1 = ((ip_inaddr >> 24) & 0xFF) as u8;
                                Ipv4Addr::new(s1, s2, s3, s4)
                            })
                            .filter(filt)
                            .collect())
    }
}