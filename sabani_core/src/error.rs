#![cfg_attr(feature = "wasabi", no_std)]

#[cfg(feature = "wasabi")]
extern crate alloc;
#[cfg(feature = "wasabi")]
extern crate core;

#[cfg(feature = "wasabi")]
use alloc::string::String;
#[cfg(feature = "wasabi")]
use core::error::Error;
#[cfg(feature = "wasabi")]
use core::fmt;

#[cfg(not(feature = "wasabi"))]
use std::error::Error;
#[cfg(not(feature = "wasabi"))]
use std::fmt;

#[derive(Debug)]
pub enum SabaniError {
  Network(String),
}

impl Error for SabaniError {}

impl fmt::Display for SabaniError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      SabaniError::Network(msg) => write!(f, "The network error: \"{msg}\""),
    }
  }
}
