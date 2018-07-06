use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn default_counter() -> Vec<u8> {
    let mut now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("now must be after epoch").as_secs();
    now /= 30;

    let mut bytes = vec![0u8; 8];
    let mut i = 8;
    while i > 0 {
        i -= 1;
        bytes[i] = (now & 0xff) as u8;
        now >>= 8;
    }

    bytes
}

pub fn totp(key: &[u8], counter: &[u8], digits: u32) -> u32 {
    let mut mac = Hmac::<Sha1>::new_varkey(key)
        .expect("Hmac::<Sha1> must be able to take key of any size");
    mac.input(counter);
    let code = mac.clone().result().code();

    truncate(&code) % 10u32.pow(digits)
}

fn truncate(bytes: &[u8]) -> u32 {
    assert!(bytes.len() == 20);

    let offset = (bytes[19] & 0xf) as usize;
    let slice = &bytes[offset..(offset + 4)];

    ((slice[0] & 0x7f) as u32) << 24
        | (slice[1] as u32) << 16
        | (slice[2] as u32) << 8
        | (slice[3] as u32)
}
