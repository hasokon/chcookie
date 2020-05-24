#[macro_use]
extern crate clap;

mod key;
mod decrypter;
mod models;
mod db;

use models::Result;
use clap::{App, Arg, ArgMatches};
use crate::models::{DecryptedCookie, GetChromeCookieError};

const ARG_HOST: &str = "host";
const ARG_LIMIT: &str = "limit";
const ARG_SILENT: &str = "silent";
const ARG_COOKIE_NAME: &str = "name";

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn exec(matches: &ArgMatches) -> Result<Vec<DecryptedCookie>> {
    let name = matches.value_of(ARG_COOKIE_NAME);
    let host = matches.value_of(ARG_HOST);

    let limit: u32 = value_t!(matches, ARG_LIMIT, u32).map_err(|err| {
        GetChromeCookieError::new(format!("Error: limit must be integer value: {}", err))
    })?;

    let values = db::get_cookie(&name, &host, limit)?;
    decrypter::decrypt(&values)
}

fn main() {
    let matches = App::new("chcookie")
        .version(VERSION)
        .author("Kentaro Iwata")
        .about("Get Chrome Cookie")
        .arg(Arg::with_name(ARG_SILENT)
            .help("display only cookie values.")
            .short("s")
            .long("silent"))
        .arg(Arg::with_name(ARG_HOST)
            .help("specify host name for search cookie value.")
            .short("h")
            .long("host")
            .takes_value(true))
        .arg(Arg::with_name(ARG_LIMIT)
            .help("count to get cookies at once.")
            .short("l")
            .long("limit")
            .default_value("20")
            .takes_value(true))
        .arg(Arg::with_name(ARG_COOKIE_NAME)
            .help("specify cookie name for search cookie value.")
            .short("n")
            .long("name")
            .takes_value(true))
        .get_matches();

    match exec(&matches) {
        Ok(result) => {
            if matches.is_present(ARG_SILENT) {
                result.iter().for_each(|r| println!("{}", r.value));
            } else {
                result.iter().for_each(|r| println!("{}", r));
            }
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    };
}
