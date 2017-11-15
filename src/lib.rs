#![cfg(all(target_arch = "x86_64", target_os = "windows"))]

#[macro_use]
extern crate lazy_static;
extern crate libloading;

pub mod wslapi;
