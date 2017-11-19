use std::os::raw::{c_char, c_long, c_ulong};
use failure::Error;
use libloading::{Library, Symbol};
use libloading::Result as LibloadingResult;
use wide_chars;

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
distro_flags = {} # {:#03b}",
            self.name,
            self.version,
            self.default_uid,
            self.flags.bits(),
            self.flags.bits(),
        )
    }
}

pub struct Wslapi {
    wslapi: Library,
}

impl Wslapi {
    pub fn new() -> Result<Wslapi, Error> {
        match Library::new("wslapi") {
            Ok(wslapi) => Ok(Wslapi { wslapi: wslapi }),
            Err(e) => Err(format_err!("Wslapi::new {}", e)),
        }
    }

    fn raw_register_distribution(
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

    fn raw_unregister_distribution(&self, distro_name: *const u16) -> LibloadingResult<i32> {
        unsafe {
            let raw_fn: Symbol<unsafe extern "system" fn(*const u16) -> c_long> =
                self.wslapi.get(b"WslUnregisterDistribution")?;
            Ok(raw_fn(distro_name))
        }
    }

    fn raw_get_distribution_configuration(
        &self,
        distro_name: *const u16,
    ) -> LibloadingResult<
        (
            i32,
            c_ulong,
            c_ulong,
            c_long,
            *mut *mut *mut c_char,
            c_ulong,
        ),
    > {
        let hresult;
        let version = 0;
        let default_uid = 0;
        let wsl_flags = 0;
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
                &wsl_flags,
                default_environment_variables_array as *mut *mut *mut c_char,
                &default_environment_variables_count,
            );
        }
        Ok((
            hresult,
            version,
            default_uid,
            wsl_flags,
            default_environment_variables_array as *mut *mut *mut c_char,
            default_environment_variables_count,
        ))
    }

    fn raw_configure_distribution(
        &self,
        distro_name: *const u16,
        default_uid: c_ulong,
        wsl_flags: u32,
    ) -> LibloadingResult<i32> {
        unsafe {
            let raw_fn: Symbol<
                unsafe extern "system" fn(*const u16, c_ulong, u32) -> c_long,
            > = self.wslapi.get(b"WslConfigureDistribution")?;
            Ok(raw_fn(distro_name, default_uid, wsl_flags))
        }
    }

    pub fn register_distro(&self, distro_name: &str, tar_gz_filename: &str) -> Result<(), Error> {
        let distro_name = wide_chars::to_vec_u16(distro_name);
        let tar_gz_filename = wide_chars::to_vec_u16(tar_gz_filename);
        match self.raw_register_distribution(distro_name.as_ptr(), tar_gz_filename.as_ptr()) {
            Ok(0) => Ok(()),
            Ok(hresult) => Err(format_err!("HRESULT == {:#08X}", hresult)),
            Err(e) => Err(format_err!("Wslapi::register_distro {}", e)),
        }
    }

    pub fn unregister_distro(&self, distro_name: &str) -> Result<(), Error> {
        let distro_name = wide_chars::to_vec_u16(distro_name);
        match self.raw_unregister_distribution(distro_name.as_ptr()) {
            Ok(0) => Ok(()),
            Ok(hresult) => Err(format_err!("HRESULT == {:#08X}", hresult)),
            Err(e) => Err(format_err!("Wslapi::unregister_distro {}", e)),
        }
    }

    pub fn get_distro_configuration(
        &self,
        distro_name: &str,
    ) -> Result<DistroConfiguration, Error> {
        let distro_name_vec_u16 = wide_chars::to_vec_u16(distro_name);
        match self.raw_get_distribution_configuration(distro_name_vec_u16.as_ptr()) {
            Ok((0, version, default_uid, wsl_flags, ..)) => Ok(DistroConfiguration {
                name: distro_name.to_string(),
                version: version,
                default_uid: default_uid,
                flags: DistroFlags::from_bits(wsl_flags as u32).unwrap(),
                default_environment_variables: None,
            }),
            Ok((hresult, ..)) => Err(format_err!("HRESULT == {:#08X}", hresult)),
            Err(e) => Err(format_err!("Wslapi::get_distro_configuration {}", e)),
        }
    }

    pub fn configure_distro(
        &self,
        distro_name: &str,
        default_uid: u32,
        distro_flags: DistroFlags,
    ) -> Result<(), Error> {
        let distro_name = wide_chars::to_vec_u16(distro_name);
        match self.raw_configure_distribution(distro_name.as_ptr(), default_uid, distro_flags.bits)
        {
            Ok(0) => Ok(()),
            Ok(hresult) => Err(format_err!("HRESULT == {:#08X}", hresult)),
            Err(e) => Err(format_err!("Wslapi::configure_distro {}", e)),
        }
    }
}
