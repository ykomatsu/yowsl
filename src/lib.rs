#![cfg(all(target_arch = "x86_64", target_os = "windows"))]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;
extern crate libloading;

pub mod wide_chars;
pub mod wslapi;

pub use wslapi::{DistroFlags, Wslapi};
