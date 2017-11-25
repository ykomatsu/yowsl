use std::{env, fs};
use std::path::Path;
use std::process::Command;
use clap::ArgMatches;
use yowsl::Wslapi;

pub fn run(wslapi: &Wslapi, matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap();
    match wslapi.is_distribution_registered(name) {
        Ok(true) => {
            eprintln!("\"{}\" is an already registered WSL distro name", name);
            return;
        }
        Ok(false) => {}
        Err(e) => {
            eprintln!("I cannot register \"{}\"\nError: {}", name, e);
            return;
        }
    }
    let src = Path::new(matches.value_of("src").unwrap());
    if !src.is_file() {
        eprintln!("\"{}\" does not exist", src.to_str().unwrap());
        return;
    }
    let dest = Path::new(matches.value_of("dest").unwrap());
    if !dest.exists() {
        if let Err(e) = fs::create_dir_all(&dest) {
            eprintln!(
                "Error: I cannot create \"{}\"\nError: {}",
                dest.to_str().unwrap(),
                e
            );
            return;
        }
    }
    if !Path::new(dest).is_dir() {
        eprintln!("\"{}\" is not a folder", dest.to_str().unwrap());
        return;
    }
    let src = fs::canonicalize(&src).unwrap();
    let dest = fs::canonicalize(&dest).unwrap();
    let current_exe = env::current_exe().unwrap();
    if current_exe.as_path().parent().unwrap() == dest {
        if let Err(e) = wslapi.register_distro(name, src.to_str().unwrap()) {
            eprintln!("I cannot register \"{}\"\nError: {}", name, e);
        }
        return;
    }
    let new_exe = dest.join(current_exe.file_name().unwrap());
    if let Err(e) = fs::hard_link(current_exe.as_path(), &new_exe) {
        eprintln!(
            "I cannot create a hard link to \"{}\"\nError: {}",
            dest.to_str().unwrap(),
            e
        );
        return;
    }
    Command::new(&new_exe)
        .args(env::args().skip(1))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
