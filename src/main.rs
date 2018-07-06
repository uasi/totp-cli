extern crate base32;
extern crate hmac;
extern crate regex;
extern crate sha1;
extern crate toml;

mod config;
mod totp;

use std::error::Error;
use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("usage: totp <item>");
        exit(1);
    }

    let item_name = &args[1];

    let config = match config::load_config() {
        Ok(config) => {
            config
        }
        Err(e) => {
            eprintln!("error: could not load config ({})", e.description());
            exit(1)
        }
    };

    match config.get(item_name) {
        Some(value) => {
            match value.as_str().and_then(config::parse_secret) {
                Some(secret) => {
                    println!("{}", totp::totp(&secret, &totp::default_counter(), 6));
                }
                None => {
                    eprintln!("error: secret for item '{}' is invalid", item_name);
                    exit(1);
                }
            }
        }
        None => {
            eprintln!("error: item '{}' not found", item_name);
            exit(1);
        }
    }
}
