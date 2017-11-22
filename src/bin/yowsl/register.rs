use std::{env, fs};
use std::path::Path;
use std::process::Command;
use clap::ArgMatches;
use yowsl::Wslapi;

pub fn run(wslapi: &Wslapi, matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap();
    let src = matches.value_of("src").unwrap();
    let dest = matches.value_of("dest").unwrap();
    let current_exe = env::current_exe().unwrap();
    if !Path::new(src).is_file() {
        eprintln!("Error: \"{}\" does not exist", src);
        return;
    }
    if !Path::new(dest).is_dir() {
        eprintln!("Error: \"{}\" does not exist", dest);
        return;
    }
    let src = fs::canonicalize(Path::new(src)).unwrap();
    let dest = fs::canonicalize(Path::new(dest)).unwrap();
    if current_exe.as_path().parent().unwrap() == dest {
        if let Err(e) = wslapi.register_distro(name, src.to_str().unwrap()) {
            eprintln!("Error: {}", e);
        }
        return;
    }
    let new_exe = dest.join(current_exe.file_name().unwrap());
    if fs::hard_link(current_exe.as_path(), &new_exe).is_err() {
        eprintln!(
            "Error: I cannot create a hard link to \"{}\".",
            dest.to_str().unwrap()
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
