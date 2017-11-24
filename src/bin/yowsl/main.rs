#![cfg(all(target_arch = "x86_64", target_os = "windows"))]

#[macro_use]
extern crate clap;
extern crate yowsl;

mod register;
mod unregister;
mod get_configuration;
mod set_configuration;
mod launch;

use clap::{App, AppSettings, Arg, SubCommand};
use yowsl::Wslapi;

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn default_uid_validator(s: String) -> Result<(), String> {
    if s.parse::<u64>().is_ok() {
        Ok(())
    } else {
        Err("A 64-bit unsigned integer is expected".to_string())
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
fn flags_validator(s: String) -> Result<(), String> {
    if s.len() == 3 && s.chars().all(|c| c == '0' || c == '1') {
        Ok(())
    } else {
        Err("3 binary digits are expected".to_string())
    }
}

fn run() {
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
                .usage("yowsl.exe register <NAME> -s <source> -d <destination>")
                .arg(Arg::from_usage("<NAME> 'A WSL distro name to register'"))
                .arg(Arg::from_usage(
                    "<src> -s, --src <source> 'A .tar.gz file that contains a root directory'",
                ))
                .arg(Arg::from_usage(
                    "<dest> -d, --dest <destination> 'A directory to register a WSL distro'",
                )),
        )
        .subcommand(
            SubCommand::with_name("unregister")
                .about("Unregisters a WSL distro")
                .usage("yowsl.exe unregister <NAME>")
                .arg(Arg::from_usage("<NAME> 'A WSL distro name to unregister'")),
        )
        .subcommand(
            SubCommand::with_name("get-configuration")
                .about("Get the configuration of a WSL distro")
                .usage("yowsl.exe get-configuration <NAME>")
                .arg(Arg::from_usage(
                    "<NAME> 'A WSL distro name to get the configuration'",
                )),
        )
        .subcommand(
            SubCommand::with_name("set-configuration")
                .about("Set the configuration of a WSL distro")
                .usage("yowsl.exe set-configuration <NAME> [-d <default_uid>] [-f <flags>]")
                .arg(Arg::from_usage(
                    "<NAME> 'A WSL distro name to set the configuration'",
                ))
                .arg(
                    Arg::from_usage(
                        "[default_uid] -d, --default-uid <default_uid>\
'The default Linux user ID (number) for this WSL distro'",
                    ).validator(default_uid_validator),
                )
                .arg(
                    Arg::from_usage(
                        "[flags] -f, --flags <flags>\
'Flags (3 binary digits) for this WSL distro. 0b001: ENABLE_INTEROP, 0b010: APPEND_NT_PATH, 0b100:\
ENABLE_DRIVE_MOUNTING'",
                    ).validator(flags_validator),
                ),
        )
        .subcommand(
            SubCommand::with_name("launch")
                .about("Launches a WSL process")
                .usage("yowsl.exe launch <NAME> [-c <command>] [-u]")
                .arg(Arg::from_usage("<NAME> 'A WSL distro name to launch'"))
                .arg(Arg::from_usage(
                    "[command] -c, --command <command>\
'Command to execute. If no command is supplied, the default shell is executed'",
                ))
                .arg(Arg::from_usage(
                    "[use_cwd] -u, --use-cwd\
'Uses the current working directory as a directory to start'",
                )),
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
        register::run(&wslapi, sub_matches);
    } else if let Some(sub_matches) = matches.subcommand_matches("unregister") {
        unregister::run(&wslapi, sub_matches);
    } else if let Some(sub_matches) = matches.subcommand_matches("get-configuration") {
        get_configuration::run(&wslapi, sub_matches);
    } else if let Some(sub_matches) = matches.subcommand_matches("set-configuration") {
        set_configuration::run(&wslapi, sub_matches);
    } else if let Some(sub_matches) = matches.subcommand_matches("launch") {
        launch::run(&wslapi, sub_matches);
    }
}

fn main() {
    run();
}
