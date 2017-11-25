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
            eprintln!("I cannot get a configuration of \"{}\"\nError: {}", name, e);
            return;
        }
    }
    match wslapi.get_distro_configuration(name) {
        Ok(distro_configuration) => println!("{}", distro_configuration.to_toml()),
        Err(e) => {
            eprintln!("I cannot get a configuration of \"{}\"\nError: {}", name, e);
        }
    }
}
