extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("SaaS CPQ")
        .version("0.0.1")
        .author("Magnus Palm <lmp.consulting@hotmail.com>")
        .about("Calculates the price of the SaaS selected subscription level and options")
        .arg(
            Arg::with_name("level")
                .short("l")
                .long("level")
                .takes_value(true)
                .help("The level of the subscription: Free, Pro, Enterprise"),
        )
        .arg(
            Arg::with_name("mod")
                .short("m")
                .long("modules")
                .takes_value(true)
                .help("A comma separated list of selected modules: Forecast,Planning,Rolling12"),
        )
        .get_matches();

    let level_input = matches
        .value_of("level")
        .unwrap_or("Missing input on subscription level");
    let modules_list_input = matches
        .value_of("mod")
        .unwrap_or("Missing input on list of selected modules");
    println!(
        "The level input passed is: {}, and the list of modules are: {}",
        level_input, modules_list_input
    );
}
