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

  pub fn lookup_host(url: &String) -> Vec<Self> {
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
  fn check_example_com() {
    let addr_lst = IPAddr::lookup_host(&"example.com".to_string());
    let is_ok = addr_lst.iter().any(|addr| addr.get() == "23.192.228.80");
    assert!(is_ok)
  }
}
