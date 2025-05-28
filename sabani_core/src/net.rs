#[cfg(feature = "wasabi")]
use alloc::string::{String, ToString};
#[cfg(feature = "wasabi")]
use alloc::vec::Vec;

#[cfg(feature = "wasabi")]
use core::str;
#[cfg(feature = "wasabi")]
use noli::net::TcpStream;
#[cfg(feature = "wasabi")]
use noli::net::{IpAddr, SocketAddr};

#[cfg(not(feature = "wasabi"))]
use std::str;

use std::io::{Read, Write};
#[cfg(not(feature = "wasabi"))]
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream};

#[cfg(feature = "linux")]
mod linux;

#[cfg(feature = "windows")]
mod windows;

#[cfg(feature = "mac")]
mod mac;

#[cfg(feature = "wasabi")]
mod wasabi;

use super::error::SabaniError;
use super::http::HttpResponse;

#[derive(Clone, Debug)]
pub struct SabaniIpAddr {
  ip_addr: IpAddr,
  addr_text: String,
}

impl SabaniIpAddr {
  pub(crate) fn new(addr_text: String) -> Result<Self, SabaniError> {
    if let Ok(v4) = addr_text.parse::<Ipv4Addr>() {
      Ok(Self {
        addr_text,
        ip_addr: IpAddr::V4(v4),
      })
    } else if let Ok(v6) = addr_text.parse::<Ipv6Addr>() {
      Ok(Self {
        addr_text,
        ip_addr: IpAddr::V6(v6),
      })
    } else {
      Err(SabaniError::Network(
        "Faild to parse IP addresses.".to_string(),
      ))
    }
  }

  pub(crate) fn get_addr_text(&self) -> String {
    self.addr_text.clone()
  }

  pub(crate) fn get_ip_addr(&self) -> IpAddr {
    self.ip_addr
  }

  pub fn lookup_host(url: &String) -> Result<Vec<Self>, SabaniError> {
    #[cfg(feature = "linux")]
    let res = linux::lookup_host(url);

    #[cfg(feature = "windows")]
    let res = windows::lookup_host(url);

    #[cfg(feature = "mac")]
    let res = mac::lookup_host(url);

    #[cfg(feature = "wasabi")]
    let res = wasabi::lookup_host(url);

    res
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_localhost() {
    let addr_lst = SabaniIpAddr::lookup_host(&"localhost".to_string()).unwrap();
    let is_ok = addr_lst
      .iter()
      .any(|addr| addr.get_addr_text() == "127.0.0.1");
    assert!(is_ok)
  }
}

pub struct HttpClient {}

impl HttpClient {
  pub fn new() -> Self {
    Self {}
  }

  pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, SabaniError> {
    let ips = match SabaniIpAddr::lookup_host(&host) {
      Ok(ips) => ips,
      Err(e) => {
        return Err(SabaniError::Network(format!(
          "Faild to find IP address: {:#?}",
          e
        )));
      }
    };

    if ips.is_empty() {
      return Err(SabaniError::Network(
        "Failed to find IP addresses".to_string(),
      ));
    }

    let socket_addr = SocketAddr::new(ips[0].get_ip_addr(), port);

    let mut stream = match TcpStream::connect(socket_addr) {
      Ok(stream) => stream,
      Err(_) => {
        return Err(SabaniError::Network(
          "Failed to connect to TCP stream".to_string(),
        ));
      }
    };

    let request = format!(
      "GET /{} HTTP/1.1\nHost: {}\nAccept: text/html\nConnection: close\n\n",
      &path, &host
    );

    let _bytes_written = match stream.write(request.as_bytes()) {
      Ok(bytes) => bytes,
      Err(_) => {
        return Err(SabaniError::Network(
          "Failed to send a request to TCP stream".to_string(),
        ));
      }
    };

    let mut received = Vec::new();
    loop {
      let mut buf = [0u8; 4096];
      let bytes_read = match stream.read(&mut buf) {
        Ok(bytes) => bytes,
        Err(_) => {
          return Err(SabaniError::Network(
            "Failed to receive a request from TCP stream".to_string(),
          ));
        }
      };
      if bytes_read == 0 {
        break;
      }
      received.extend_from_slice(&buf[..bytes_read]);
    }

    match str::from_utf8(&received) {
      Ok(response) => HttpResponse::new(response.to_string()),
      Err(e) => Err(SabaniError::Network(format!(
        "Invalid received response: {e}"
      ))),
    }
  }
}
