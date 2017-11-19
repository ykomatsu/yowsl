use std::{env, fs};
use std::path::Path;
use std::process::Command;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use yowsl::{DistroFlags, Wslapi};

fn run_register(wslapi: Wslapi, matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap();
    let src = matches.value_of("src").unwrap();
    let dest = matches.value_of("dest").unwrap();
    let current_exe = env::current_exe().unwrap();
    if Path::new(src).is_file() == false {
        eprintln!("Error: \"{}\" does not exist", src);
        return;
    }
    if Path::new(dest).is_dir() == false {
        eprintln!("Error: \"{}\" does not exist", dest);
        return;
    }
    let src = fs::canonicalize(Path::new(src)).unwrap();
    let dest = fs::canonicalize(Path::new(dest)).unwrap();
    if current_exe.as_path().parent().unwrap() == dest {
        match wslapi.register_distro(name, src.to_str().unwrap()) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        }
    } else {
        let new_exe = dest.join(current_exe.file_name().unwrap());
        match fs::hard_link(current_exe.as_path(), &new_exe) {
            Ok(()) => (),
            Err(_) => {
                eprintln!(
                    "Error: I cannot create a hard link to \"{}\".",
                    dest.to_str().unwrap()
                );
                return;
            }
        }
        Command::new(&new_exe)
            .args(env::args().skip(1))
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

fn run_unregister(wslapi: Wslapi, matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap();
    match wslapi.unregister_distro(name) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    }
}

fn run_get_configuration(wslapi: Wslapi, matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap();
    match wslapi.get_distro_configuration(name) {
        Ok(distro_configuration) => println!("{}", distro_configuration.to_toml()),
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    }
}

fn run_set_configuration(wslapi: Wslapi, matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap();
    let default_uid = matches.value_of("default_uid").unwrap().parse().unwrap();
    let flags = DistroFlags::from_bits(
        u32::from_str_radix(matches.value_of("flags").unwrap(), 2).unwrap(),
    ).unwrap();
    match wslapi.configure_distro(name, default_uid, flags) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    }
}

fn default_uid_validator(s: String) -> Result<(), String> {
    if let Ok(_) = s.parse::<u64>() {
        Ok(())
    } else {
        Err("A 64-bit unsigned integer is expected".to_string())
    }
}

fn flags_validator(s: String) -> Result<(), String> {
    if s.len() == 3 && s.chars().all(|c| c == '0' || c == '1') {
        Ok(())
    } else {
        Err("3 binary digits are expected".to_string())
    }
}

pub fn run() {
    let matches = App::new("yowsl")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Yet another Windows Subsystem for Linux tweaker")
        .global_settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::DeriveDisplayOrder,
        ])
        .subcommand(
            SubCommand::with_name("register")
                .about("Registers a WSL distro")
                .arg(
                    Arg::with_name("NAME")
                        .help("A WSL distro name to register")
                        .required(true),
                )
                .arg(
                    Arg::with_name("src")
                        .short("s")
                        .long("src")
                        .value_name("SOURCE")
                        .help("A source .tar.gz file")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("dest")
                        .short("d")
                        .long("dest")
                        .value_name("DESTINATION")
                        .help("A destination directory")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("unregister")
                .about("Unregisters a WSL distro")
                .arg(
                    Arg::with_name("NAME")
                        .help("A WSL distro name to unregister")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get-configuration")
                .about("Get the configuration of a WSL distro")
                .arg(
                    Arg::with_name("NAME")
                        .help("A WSL distro name to get the configuration")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("set-configuration")
                .about("Set the configuration of a WSL distro")
                .arg(
                    Arg::with_name("NAME")
                        .help("A WSL distro name to set the configuration")
                        .required(true),
                )
                .arg(
                    Arg::with_name("default_uid")
                        .short("u")
                        .long("default-uid")
                        .value_name("DEFAULT_UID")
                        .help("The default Linux user ID for this WSL distro")
                        .takes_value(true)
                        .validator(default_uid_validator)
                        .required(true),
                )
                .arg(
                    Arg::with_name("flags")
                        .short("f")
                        .long("flags")
                        .value_name("FLAGS")
                        .help("Flags for this WSL distro")
                        .takes_value(true)
                        .validator(flags_validator)
                        .required(true),
                ),
        )
        .get_matches();
    let wslapi = match Wslapi::new() {
        Ok(wslapi) => wslapi,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    if let Some(sub_matches) = matches.subcommand_matches("register") {
        run_register(wslapi, sub_matches);
    } else if let Some(sub_matches) = matches.subcommand_matches("unregister") {
        run_unregister(wslapi, sub_matches);
    } else if let Some(sub_matches) = matches.subcommand_matches("get-configuration") {
        run_get_configuration(wslapi, sub_matches);
    } else if let Some(sub_matches) = matches.subcommand_matches("set-configuration") {
        run_set_configuration(wslapi, sub_matches);
    }
}
