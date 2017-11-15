#![cfg(all(target_arch = "x86_64", target_os = "windows"))]

#[macro_use]
extern crate clap;
extern crate yowsl;

mod run;

fn main() {
    run::run();
}
