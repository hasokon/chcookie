use aes::Aes128;
use block_modes::{Cbc, BlockMode};
use block_modes::block_padding::Pkcs7;

type AesCbc = Cbc<Aes128, Pkcs7>;

use crate::models::{CookieDataRaw, DecryptedCookie, Result, GetChromeCookieError};
use crate::key::get_chrome_key;

const IV: [u8; 16] = [0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20];

pub fn decrypt(src: &Vec<CookieDataRaw>) -> Result<Vec<DecryptedCookie>> {
    let key = get_chrome_key()?;
    src.iter().map(|rcookie| {
        let decrypter = AesCbc::new_var(&key, &IV).map_err(|err| {
            GetChromeCookieError::new(format!("Fail to create decrypter: {}", err.to_string()))
        })?;

        let encrypted_value = &rcookie.encrypted_value;
        let result = decrypter.decrypt_vec(&encrypted_value[3..]).map_err(|err| {
            GetChromeCookieError::new(format!("Fail to decrypt value: {}", err.to_string()))
        })?;

        let cookie_value = String::from_utf8(result).map_err(|err| {
            GetChromeCookieError::new(format!("Fail to decode from byte to UTF-8 string: {}", err.to_string()))
        })?;

        Ok(DecryptedCookie {
            name: rcookie.name.clone(),
            host_key: rcookie.host_key.clone(),
            value: cookie_value,
        })
    }).collect::<Result<Vec<DecryptedCookie>>>()
}