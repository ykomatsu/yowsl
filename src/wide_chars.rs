use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::{slice, usize};
use failure::Error;

pub fn to_vec_u16(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

pub fn from_vec_u16(p: *const u16) -> Result<String, Error> {
    if p.is_null() {
        return Err(format_err!("p is a null"));
    };
    unsafe {
        match (0..usize::max_value()).position(|i| *p.offset(i as isize) == 0) {
            Some(len) => Ok(
                OsString::from_wide(slice::from_raw_parts(p, len))
                    .to_string_lossy()
                    .into_owned(),
            ),
            None => Err(format_err!("p does not end with a null")),
        }
    }
}
