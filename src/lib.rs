//! An HTTP content negotiator for http-rs.

mod charsets;
mod encodings;
mod languages;
mod media_types;

pub fn charset(accept_header: Option<&str>, available: &[&str]) -> Option<String> {
    let set = charsets(accept_header, available);
    if set.len() > 0 {
        Some(set[0].to_owned())
    } else {
        None
    }
}
  
pub fn charsets(accept_header: Option<&str>, available: &[&str]) -> Vec<String> {
    charsets::preferred_charsets(accept_header, available)
}
