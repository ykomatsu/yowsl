use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::{slice, usize};
use failure;
use failure::Error;

pub fn to_vec_u16(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

#[allow(dead_code)]
pub unsafe fn from_vec_u16(p: *const u16) -> Result<String, Error> {
    if p.is_null() {
        return Err(failure::err_msg("p is a null"));
    };
    match (0..usize::max_value()).position(|i| *p.offset(i as isize) == 0) {
        Some(len) => Ok(
            OsString::from_wide(slice::from_raw_parts(p, len))
                .to_string_lossy()
                .into_owned(),
        ),
        None => Err(failure::err_msg("p does not end with a null")),
    }
}
