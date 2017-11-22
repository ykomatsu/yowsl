use clap::ArgMatches;
use yowsl::Wslapi;

pub fn run(wslapi: &Wslapi, matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap();
    if let Err(e) = wslapi.unregister_distro(name) {
        eprintln!("Error: {}", e);
    }
}
