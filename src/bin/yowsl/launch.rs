use clap::ArgMatches;
use yowsl::Wslapi;

pub fn run(wslapi: &Wslapi, matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap();
    let command = match matches.value_of("command") {
        Some(command) => command,
        None => "",
    };
    let use_cwd = matches.is_present("use_cwd");
    if let Err(e) = wslapi.launch(name, command, use_cwd) {
        eprintln!("Error: {}", e);
    }
}
