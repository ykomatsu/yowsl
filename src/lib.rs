#![cfg(all(target_arch = "x86_64", target_os = "windows"))]

#[macro_use]
extern crate bitflags;
extern crate libloading;

pub mod wslapi;

pub use wslapi::{DistroFlags, Wslapi};
