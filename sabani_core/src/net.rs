#[cfg(feature = "wasabi")]
use alloc::string::{String, ToString};
#[cfg(feature = "wasabi")]
use alloc::vec::Vec;

#[cfg(feature = "linux")]
mod linux;

#[cfg(feature = "windows")]
mod windows;

#[cfg(feature = "mac")]
mod mac;

#[cfg(feature = "wasabi")]
mod wasabi;

#[cfg(not(feature = "wasabi"))]
use super::error::SabaniError;

#[derive(Clone)]
pub struct IPAddr {
  addr: String,
}

impl IPAddr {
  pub(crate) fn new(addr: String) -> Self {
    Self { addr }
  }

  pub(crate) fn get(&self) -> String {
    self.addr.clone()
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
    let addr_lst = IPAddr::lookup_host(&"localhost".to_string()).unwrap();
    let is_ok = addr_lst.iter().any(|addr| addr.get() == "127.0.0.1");
    assert!(is_ok)
  }
}

pub struct HttpClient {}

impl HttpClient {
  pub fn new() -> Self {
    Self {}
  }

  pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, SabaniError> {
    let ips = match IPAddr::lookup_host(&host) {
      Ok(ips) => ips,
      Err(e) => {
        return Err(SabaniError::Network(format!(
          "Faild to find IP address: {:#?}",
          e
        )));
      }
    };

    if ips.len() < 1 {
      return Err(SabaniError::Network(
        "Failed to find IP addresses".to_string(),
      ));
    }
    todo!()
  }
}
