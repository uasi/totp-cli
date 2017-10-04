use base32;
use regex;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use toml::Value as TomlValue;

pub fn load_config() -> io::Result<TomlValue> {
    let mut path = env::home_dir().expect("home dir must exist");
    path.push(".totp.toml");

    let mut file = File::open(&path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    content.parse::<TomlValue>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.description()))
}

pub fn parse_secret(secret: &str) -> Option<Vec<u8>> {
    let re = regex::Regex::new(r"(?i)\bsecret=([0-9a-z]+)").expect("regex must compile");

    if let Some(s) = re.captures(secret).and_then(|c| c.get(1)).map(|m| m.as_str()) {
        decode_secret(s)
    } else {
        decode_secret(secret)
    }
}

fn decode_secret(secret: &str) -> Option<Vec<u8>> {
    let alphabet = base32::Alphabet::RFC4648 { padding: false };
    base32::decode(alphabet, secret)
}
