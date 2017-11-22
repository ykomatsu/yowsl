#![cfg(all(target_arch = "x86_64", target_os = "windows"))]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;
extern crate libloading;

mod wide_chars;
mod wslapi;

pub use wslapi::{DistroConfiguration, DistroFlags, Wslapi};
