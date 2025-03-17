// no_std; intended for embedded environments
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
extern crate std; // to permit testing in non-embedded environment

pub mod dht20; // main dht20 module
mod utils; // utility functions for dht20