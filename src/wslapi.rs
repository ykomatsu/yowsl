use std::ffi::{OsStr, OsString};
use std::os::raw::{c_char, c_long, c_ulong, c_void};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::{process, slice, usize};
use super::libloading::{Library, Symbol};
use super::libloading::Result as LibloadingResult;
use super::ole32;

fn to_vec_u16(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

fn from_vec_u16(p: *const u16) -> String {
    if p.is_null() {
        return String::new();
    };
    unsafe {
        let len = (0..usize::max_value())
            .position(|i| *p.offset(i as isize) == 0)
            .unwrap();
        OsString::from_wide(slice::from_raw_parts(p, len))
            .to_string_lossy()
            .into_owned()
    }
}

bitflags! {
    pub struct DistroFlags: u32 {
        const NONE = 0;
        const ENABLE_INTEROP = 1;
        const APPEND_NT_PATH = 2;
        const ENABLE_DRIVE_MOUNTING = 4;
    }
}

pub struct DistroConfiguration<'a> {
    pub name: String,
    pub version: u32,
    pub default_uid: u32,
    pub flags: DistroFlags,
    pub default_environment_variables: Option<Vec<&'a str>>,
}

impl<'a> DistroConfiguration<'a> {
    pub fn to_toml(&self) -> String {
        format!(
            "[{}]
version = {}
default_uid = {}
distro_flags = 0b{:03b}",
            self.name,
            self.version,
            self.default_uid,
            self.flags.bits()
        )
    }
}

pub struct Wslapi {
    wslapi: Library,
}

impl Wslapi {
    pub fn new() -> Wslapi {
        Wslapi {
            wslapi: match Library::new("wslapi") {
                Ok(wslapi) => wslapi,
                Err(e) => {
                    eprintln!("Wslapi::new Error: {}", e);
                    process::exit(1)
                }
            },
        }
    }

    fn raw_register_distro(
        &self,
        distro_name: *const u16,
        tar_gz_filename: *const u16,
    ) -> LibloadingResult<i32> {
        unsafe {
            let raw_fn: Symbol<
                unsafe extern "system" fn(*const u16, *const u16) -> c_long,
            > = self.wslapi.get(b"WslRegisterDistribution")?;
            Ok(raw_fn(distro_name, tar_gz_filename))
        }
    }

    fn raw_unregister_distro(&self, distro_name: *const u16) -> LibloadingResult<i32> {
        unsafe {
            let raw_fn: Symbol<unsafe extern "system" fn(*const u16) -> c_long> =
                self.wslapi.get(b"WslUnregisterDistribution")?;
            Ok(raw_fn(distro_name))
        }
    }

    fn raw_get_distro_configuration(
        &self,
        distro_name: *const u16,
    ) -> LibloadingResult<(i32, DistroConfiguration)> {
        let hresult;
        let version = 0;
        let default_uid = 0;
        let distro_flags = 0;
        let default_environment_variables_array = Box::into_raw(Box::new(0));
        let default_environment_variables_count = 0;
        unsafe {
            let raw_fn: Symbol<
                unsafe extern "system" fn(
                    *const u16,
                    &c_ulong,
                    &c_ulong,
                    &c_long,
                    *mut *mut *mut c_char,
                    &c_ulong,
                ) -> c_long,
            > = self.wslapi.get(b"WslGetDistributionConfiguration")?;
            hresult = raw_fn(
                distro_name,
                &version,
                &default_uid,
                &distro_flags,
                default_environment_variables_array as *mut *mut *mut c_char,
                &default_environment_variables_count,
            );
        }
        let distro_flags = DistroFlags::from_bits(distro_flags as u32).unwrap();
        let distro_configuration = DistroConfiguration {
            name: from_vec_u16(distro_name),
            version: version,
            default_uid: default_uid,
            flags: distro_flags,
            default_environment_variables: None,
        };
        unsafe {
            ole32::CoTaskMemFree(default_environment_variables_array as *mut c_void);
        }
        Ok((hresult, distro_configuration))
    }

    fn raw_configure_distro(
        &self,
        distro_name: *const u16,
        default_uid: c_ulong,
        distro_flags: u32,
    ) -> LibloadingResult<i32> {
        unsafe {
            let raw_fn: Symbol<
                unsafe extern "system" fn(*const u16, c_ulong, u32) -> c_long,
            > = self.wslapi.get(b"WslConfigureDistribution")?;
            Ok(raw_fn(distro_name, default_uid, distro_flags))
        }
    }

    pub fn register_distro(&self, distro_name: &str, tar_gz_filename: &str) -> i32 {
        let distro_name = to_vec_u16(distro_name);
        let tar_gz_filename = to_vec_u16(tar_gz_filename);
        match self.raw_register_distro(distro_name.as_ptr(), tar_gz_filename.as_ptr()) {
            Ok(hresult) => hresult,
            Err(e) => {
                eprintln!("Wslapi::register_distro Error: {}", e);
                process::exit(1)
            }
        }
    }

    pub fn unregister_distro(&self, distro_name: &str) -> i32 {
        let distro_name = to_vec_u16(distro_name);
        match self.raw_unregister_distro(distro_name.as_ptr()) {
            Ok(hresult) => hresult,
            Err(e) => {
                eprintln!("Wslapi::unregister_distro Error: {}", e);
                process::exit(1)
            }
        }
    }

    pub fn get_distro_configuration(&self, distro_name: &str) -> (i32, DistroConfiguration) {
        let distro_name = to_vec_u16(distro_name);
        match self.raw_get_distro_configuration(distro_name.as_ptr()) {
            Ok((hresult, distro_configuration)) => (hresult, distro_configuration),
            Err(e) => {
                eprintln!("Wslapi::get_distro_configuration Error: {}", e);
                process::exit(1)
            }
        }
    }

    pub fn configure_distro(
        &self,
        distro_name: &str,
        default_uid: u32,
        distro_flags: DistroFlags,
    ) -> i32 {
        let distro_name = to_vec_u16(distro_name);
        match self.raw_configure_distro(distro_name.as_ptr(), default_uid, distro_flags.bits) {
            Ok(hresult) => hresult,
            Err(e) => {
                eprintln!("Wslapi::configure_distro Error: {}", e);
                process::exit(1)
            }
        }
    }
}
