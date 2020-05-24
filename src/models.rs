use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Hash, Default)]
pub struct CookieDataRaw {
    pub name: String,
    pub host_key: String,
    pub encrypted_value: Vec<u8>,
}

#[derive(Debug, Eq, PartialEq, Hash, Default)]
pub struct DecryptedCookie {
    pub name: String,
    pub host_key: String,
    pub value: String,
}

impl Display for DecryptedCookie {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.name, self.host_key, self.value)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Default)]
pub struct GetChromeCookieError {
    pub reason: String,
}

impl GetChromeCookieError {
    pub fn new(reason: String) -> Self {
        GetChromeCookieError{
            reason,
        }
    }
}

impl Display for GetChromeCookieError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl std::error::Error for GetChromeCookieError {

}

pub type Result<T> = std::result::Result<T, GetChromeCookieError>;