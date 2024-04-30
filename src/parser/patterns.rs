use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref REGEX_WHITESPACE: Regex = Regex::new(r"\s").unwrap();
}
