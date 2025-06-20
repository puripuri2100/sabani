#![cfg_attr(feature = "wasabi", no_std)]

#[cfg(feature = "wasabi")]
extern crate alloc;

pub mod browser;
pub mod error;
pub mod http;
pub mod net;
pub mod renderer;
pub mod url;
pub mod utils;
