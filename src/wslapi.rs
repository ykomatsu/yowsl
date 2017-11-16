use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::{process, slice};
use super::libloading::{Library, Symbol};
use super::libloading::Result as libloadingResult;

fn to_vec_u16(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

bitflags! {
    pub struct DistroFlags: u32 {
        const NONE = 0x0;
        const ENABLE_INTEROP = 0x1;
        const APPEND_NT_PATH = 0x2;
        const ENABLE_DRIVE_MOUNTING = 0x4;
    }
}

pub struct DistroConfiguration<'a> {
    pub version: u64,
    pub default_uid: u64,
    pub distro_flags: DistroFlags,
    pub default_environment_variables: Vec<&'a str>,
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
    ) -> libloadingResult<u32> {
        unsafe {
            let raw_fn: Symbol<
                unsafe extern "C" fn(*const u16, *const u16) -> u32,
            > = self.wslapi.get(b"WslRegisterDistribution")?;
            Ok(raw_fn(distro_name, tar_gz_filename))
        }
    }

    fn raw_unregister_distro(&self, distro_name: *const u16) -> libloadingResult<u32> {
        unsafe {
            let raw_fn: Symbol<unsafe extern "C" fn(*const u16) -> u32> =
                self.wslapi.get(b"WslUnregisterDistribution")?;
            Ok(raw_fn(distro_name))
        }
    }

    fn raw_get_distro_configuration(
        &self,
        distro_name: *const u16,
    ) -> libloadingResult<(u32, DistroConfiguration)> {
        let hresult;
        let mut version = 0;
        let mut default_uid = 0;
        let mut distro_flags = 0;
        let mut default_environment_variables_array = Box::into_raw(Box::new(0));
        let mut default_environment_variables_count = 0;
        unsafe {
            let raw_fn: Symbol<
                unsafe extern "C" fn(*const u16, &u64, &u64, &u32, *mut *mut *mut char, &u64)
                    -> u32,
            > = self.wslapi.get(b"WslGetDistributionConfiguration")?;
            hresult = raw_fn(
                distro_name,
                &version,
                &default_uid,
                &distro_flags,
                default_environment_variables_array as *mut *mut *mut char,
                &default_environment_variables_count,
            );
        }
        let distro_flags = DistroFlags::from_bits(distro_flags).unwrap();
        let default_environment_variables = unsafe {
            slice::from_raw_parts(
                default_environment_variables_array as *mut *mut *mut char,
                default_environment_variables_count as usize,
            )
        };
        Ok((
            hresult,
            DistroConfiguration {
                version: version,
                default_uid: default_uid,
                distro_flags: distro_flags,
                default_environment_variables: vec![],
            },
        ))
    }

    fn raw_configure_distro(
        &self,
        distro_name: *const u16,
        default_uid: u64,
        distro_flags: u32,
    ) -> libloadingResult<u32> {
        unsafe {
            let raw_fn: Symbol<
                unsafe extern "C" fn(*const u16, u64, u32) -> u32,
            > = self.wslapi.get(b"WslConfigureDistribution")?;
            Ok(raw_fn(distro_name, default_uid, distro_flags))
        }
    }

    pub fn register_distro(&self, distro_name: &str, tar_gz_filename: &str) -> u32 {
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

    pub fn unregister_distro(&self, distro_name: &str) -> u32 {
        let distro_name = to_vec_u16(distro_name);
        match self.raw_unregister_distro(distro_name.as_ptr()) {
            Ok(hresult) => hresult,
            Err(e) => {
                eprintln!("Wslapi::unregister_distro Error: {}", e);
                process::exit(1)
            }
        }
    }

    pub fn get_distro_configuration(&self, distro_name: &str) -> (u32, DistroConfiguration) {
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
        default_uid: u64,
        distro_flags: DistroFlags,
    ) -> u32 {
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
