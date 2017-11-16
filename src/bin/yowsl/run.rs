use std::process;
use super::clap::{App, Arg, ArgMatches, SubCommand};
use yowsl::{DistroFlags, Wslapi};

fn run_register(wslapi: Wslapi, matches: &ArgMatches) {
    let distro_name = matches.value_of("DISTRO_NAME").unwrap();
    let src = matches.value_of("src").unwrap();
    let dest = matches.value_of("dest").unwrap();
    // Use CreateHardLink!
    let hresult = wslapi.register_distro(distro_name, src);
    if hresult != 0 {
        eprintln!("Error: HRESULT = 0x{:08X}", hresult);
        process::exit(1)
    }
}

fn run_unregister(wslapi: Wslapi, matches: &ArgMatches) {
    let distro_name = matches.value_of("DISTRO_NAME").unwrap();
    let hresult = wslapi.unregister_distro(distro_name);
    if hresult != 0 {
        eprintln!("Error: HRESULT = 0x{:08X}", hresult);
        process::exit(1)
    }
}

fn run_get_configuration(wslapi: Wslapi, matches: &ArgMatches) {
    let distro_name = matches.value_of("DISTRO_NAME").unwrap();
    let (hresult, distro_configuration) = wslapi.get_distro_configuration(distro_name);
    if hresult == 0 {
        println!(
            "[{}]
version = {}
default_uid = {}
distro_flags = 0b{:04b}
# distro_environment_variables",
            distro_name,
            distro_configuration.version,
            distro_configuration.default_uid,
            distro_configuration.distro_flags
        );
    } else {
        eprintln!("Error: HRESULT = 0x{:08X}", hresult);
        process::exit(1)
    }
}

fn run_set_configuration(wslapi: Wslapi, matches: &ArgMatches) {
    let distro_name = matches.value_of("DISTRO_NAME").unwrap();
    let default_uid = matches.value_of("default_uid").unwrap();
    let default_uid: u64 = default_uid.parse().unwrap();
    let distro_flags = matches.value_of("distro_flags").unwrap();
    let distro_flags = DistroFlags::from_bits(distro_flags.parse().unwrap()).unwrap();
    let hresult = wslapi.configure_distro(distro_name, default_uid, distro_flags);
    if hresult != 0 {
        eprintln!("Error: HRESULT = 0x{:8X}", hresult);
        process::exit(1)
    }
}

fn u32_validator(s: String) -> Result<(), String> {
    if let Ok(_) = s.parse::<u32>() {
        Ok(())
    } else {
        Err("u32 is expected".to_string())
    }
}

fn u64_validator(s: String) -> Result<(), String> {
    if let Ok(_) = s.parse::<u64>() {
        Ok(())
    } else {
        Err("u64 is expected".to_string())
    }
}

pub fn run() {
    let matches = App::new("yowsl")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Yet another Windows Subsystem for Linux tweaker")
        .subcommand(
            SubCommand::with_name("register")
                .about("Registers a WSL distro")
                .arg(
                    Arg::with_name("DISTRO_NAME")
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
                    Arg::with_name("DISTRO_NAME")
                        .help("A WSL distro name to unregister")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get-configuration")
                .about("Get the configuration of a WSL distro")
                .arg(
                    Arg::with_name("DISTRO_NAME")
                        .help("A WSL distro name to get the configuration")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("set-configuration")
                .about("Set the configuration of a WSL distro")
                .arg(
                    Arg::with_name("DISTRO_NAME")
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
                        .validator(u64_validator)
                        .required(true),
                )
                .arg(
                    Arg::with_name("distro_flags")
                        .short("f")
                        .long("distro-flags")
                        .value_name("DISTRO_FLAGS")
                        .help("Flags for this WSL distro")
                        .takes_value(true)
                        .validator(u32_validator)
                        .required(true),
                ),
        )
        .get_matches();
    let wslapi = Wslapi::new();
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
