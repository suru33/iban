use std::process::exit;

use clap::{App, Arg};
use rand::seq::SliceRandom;

use iban::IBANMetaData;

mod iban;

static IBAN_METADATA: &str = include_str!("res/metadata.json");

fn main() {
    let arg_matches = App::new("IBAN")
        .version("1.0.0")
        .about("Generate random IBAN")
        .arg(Arg::with_name("country_code")
            .short("c")
            .long("country-code")
            .value_name("COUNTRY CODE")
            .help("IBAN country code")
            .takes_value(true))
        .arg(Arg::with_name("pretty_print")
            .short("p")
            .long("pretty-print")
            .required(false)
            .takes_value(false)
            .help("Pretty print the IBAN"))
        .get_matches();


    let meta: Vec<IBANMetaData> = serde_json::from_str(IBAN_METADATA)
        .expect("error while reading metadata");

    let pretty_print = arg_matches.occurrences_of("pretty_print") == 1;

    let country: Option<&IBANMetaData> = match arg_matches.value_of("country_code") {
        Some(country_code) => meta.iter().find(|&i| i.code == country_code.to_uppercase()),
        None => { meta.choose(&mut rand::thread_rng()) }
    };

    match country {
        Some(c) => if pretty_print {
            print!("{}", c.get_pretty());
        } else {
            print!("{}", c.get());
        },
        None => {
            eprintln!("Country code not found");
            exit(1);
        }
    }
}
