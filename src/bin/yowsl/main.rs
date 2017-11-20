#![cfg(all(target_arch = "x86_64", target_os = "windows"))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
extern crate yowsl;

mod run;

fn main() {
    run::run();
}
