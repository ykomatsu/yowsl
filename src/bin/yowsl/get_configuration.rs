use clap::ArgMatches;
use yowsl::Wslapi;

pub fn run(wslapi: &Wslapi, matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap();
    match wslapi.get_distro_configuration(name) {
        Ok(distro_configuration) => println!("{}", distro_configuration.to_toml()),
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
