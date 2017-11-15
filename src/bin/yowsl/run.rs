use std::process;
use super::clap::{App, Arg, ArgMatches, SubCommand};
use yowsl::wslapi;

fn run_register(matches: &ArgMatches) {
    let distro_name = matches.value_of("DISTRO_NAME").unwrap();
    let src = matches.value_of("src").unwrap();
    // let dest = matches.value_of("dest").unwrap();
    let hresult = wslapi::register_distribution(distro_name, src);
    if hresult != 0 {
        eprintln!("Error: HRESULT = 0x{:X}", hresult);
        process::exit(1)
    }
}

fn run_unregister(matches: &ArgMatches) {
    let distro_name = matches.value_of("DISTRO_NAME").unwrap();
    let hresult = wslapi::unregister_distribution(distro_name);
    if hresult != 0 {
        eprintln!("Error: HRESULT = 0x{:X}", hresult);
        process::exit(1)
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
                ), /*
                .arg(
                    Arg::with_name("dest")
                        .short("d")
                        .long("dest")
                        .value_name("DESTINATION")
                        .help("A destination directory")
                        .takes_value(true)
                        .required(true),
                ),
                */
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
        .get_matches();
    if let Some(sub_matches) = matches.subcommand_matches("register") {
        run_register(sub_matches);
    } else if let Some(sub_matches) = matches.subcommand_matches("unregister") {
        run_unregister(sub_matches);
    }
}
