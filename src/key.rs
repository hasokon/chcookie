use keyring::Keyring;
use crypto::pbkdf2::pbkdf2;
use crypto::hmac::Hmac;
use crypto::sha1::Sha1;

use crate::models::{Result, GetChromeCookieError};

const SALT: &str = "saltysalt";
const LENGTH: usize = 16;
const ITERATIONS: u32 = 1003;

pub fn get_chrome_key() -> Result<Vec<u8>> {
    let keyring = Keyring::new("Chrome Safe Storage", "Chrome");
    let password = keyring.get_password().map_err(|err| {
        GetChromeCookieError::new(format!("Fail to get password: {}", err.to_string()))
    })?;

    let mut mac = Hmac::new(Sha1::new(), password.as_bytes());

    let mut key = [0u8; LENGTH];
    pbkdf2(&mut mac, SALT.as_bytes(), ITERATIONS, &mut key);

    Ok(key.to_vec())
}
