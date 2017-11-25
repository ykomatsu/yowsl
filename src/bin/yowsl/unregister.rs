use clap::ArgMatches;
use yowsl::Wslapi;

pub fn run(wslapi: &Wslapi, matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap();
    match wslapi.is_distribution_registered(name) {
        Ok(true) => {}
        Ok(false) => {
            eprintln!("\"{}\" is not a registered WSL distro name", name);
            return;
        }
        Err(e) => {
            eprintln!("I cannot unregister \"{}\"\nError: {}", name, e);
            return;
        }
    }
    if let Err(e) = wslapi.unregister_distro(name) {
        eprintln!("I cannot unregister \"{}\"\nError: {}", name, e);
    }
}
