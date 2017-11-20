use std::os::raw::c_void;
use failure::Error;
use libloading::{Library, Symbol};
use libloading::Result as LibloadingResult;
use wide_chars;

type DWORD = u32;
type HRESULT = LONG;
type LONG = i32;
type LPVOID = *const c_void;
type PCWSTR = *mut u16;
type PSTR = *mut u8;
type ULONG = u32;
#[allow(non_camel_case_types)]
type WSL_DISTRIBUTION_FLAGS = u32;

type CoTaskMemFreeFn = unsafe extern "system" fn(LPVOID);
type RegisterDistributionFn = unsafe extern "system" fn(PCWSTR, PCWSTR) -> HRESULT;
type UnregisterDistributionFn = unsafe extern "system" fn(PCWSTR) -> HRESULT;
type GetDistributionConfigurationFn = unsafe extern "system" fn(
    PCWSTR,
    &ULONG,
    &ULONG,
    &WSL_DISTRIBUTION_FLAGS,
    PSTR,
    &ULONG,
) -> HRESULT;
type ConfigureDistributionFn = unsafe extern "system" fn(PCWSTR, ULONG, WSL_DISTRIBUTION_FLAGS)
    -> HRESULT;
type LaunchInteractiveFn = unsafe extern "system" fn(PCWSTR, PCWSTR, bool, *const DWORD)
    -> HRESULT;

bitflags! {
    pub struct DistroFlags: WSL_DISTRIBUTION_FLAGS {
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
    ole32: Library,
    wslapi: Library,
}

impl Wslapi {
    pub fn new() -> Result<Wslapi, Error> {
        let ole32 = match Library::new("ole32") {
            Ok(library) => library,
            Err(e) => return Err(format_err!("Wslapi::new {}", e)),
        };
        let wslapi = match Library::new("wslapi") {
            Ok(library) => library,
            Err(e) => return Err(format_err!("Wslapi::new {}", e)),
        };
        Ok(Wslapi {
            ole32: ole32,
            wslapi: wslapi,
        })
    }

    #[allow(dead_code)]
    fn raw_co_task_mem_free(&self, v: LPVOID) -> LibloadingResult<()> {
        unsafe {
            let raw_fn: Symbol<CoTaskMemFreeFn> = self.ole32.get(b"CoTaskMemFree")?;
            raw_fn(v);
        }
        Ok(())
    }

    fn raw_register_distribution(
        &self,
        distro_name: PCWSTR,
        tar_gz_filename: PCWSTR,
    ) -> LibloadingResult<HRESULT> {
        unsafe {
            let raw_fn: Symbol<RegisterDistributionFn> =
                self.wslapi.get(b"WslRegisterDistribution")?;
            Ok(raw_fn(distro_name, tar_gz_filename))
        }
    }

    fn raw_unregister_distribution(&self, distro_name: PCWSTR) -> LibloadingResult<HRESULT> {
        unsafe {
            let raw_fn: Symbol<UnregisterDistributionFn> =
                self.wslapi.get(b"WslUnregisterDistribution")?;
            Ok(raw_fn(distro_name))
        }
    }

    fn raw_get_distribution_configuration(
        &self,
        distro_name: PCWSTR,
    ) -> LibloadingResult<(HRESULT, ULONG, ULONG, WSL_DISTRIBUTION_FLAGS, PSTR, ULONG)> {
        let hresult;
        let version = 0;
        let default_uid = 0;
        let wsl_flags = 0;
        let default_environment_variables_array = Box::into_raw(Box::new(0));
        let default_environment_variables_count = 0;
        unsafe {
            let raw_fn: Symbol<GetDistributionConfigurationFn> =
                self.wslapi.get(b"WslGetDistributionConfiguration")?;
            hresult = raw_fn(
                distro_name,
                &version,
                &default_uid,
                &wsl_flags,
                default_environment_variables_array as PSTR,
                &default_environment_variables_count,
            );
        }
        Ok((
            hresult,
            version,
            default_uid,
            wsl_flags,
            default_environment_variables_array as PSTR,
            default_environment_variables_count,
        ))
    }

    fn raw_configure_distribution(
        &self,
        distro_name: PCWSTR,
        default_uid: ULONG,
        wsl_flags: WSL_DISTRIBUTION_FLAGS,
    ) -> LibloadingResult<HRESULT> {
        unsafe {
            let raw_fn: Symbol<ConfigureDistributionFn> =
                self.wslapi.get(b"WslConfigureDistribution")?;
            Ok(raw_fn(distro_name, default_uid, wsl_flags))
        }
    }

    fn raw_launch_interactive(
        &self,
        distro_name: PCWSTR,
        command: PCWSTR,
        use_current_working_directory: bool,
    ) -> LibloadingResult<(i32, DWORD)> {
        unsafe {
            let exit_code = 0;
            let raw_fn: Symbol<LaunchInteractiveFn> = self.wslapi.get(b"WslLaunchInteractive")?;
            let hresult = raw_fn(
                distro_name,
                command,
                use_current_working_directory,
                &exit_code,
            );
            Ok((hresult, exit_code))
        }
    }

    pub fn register_distro(&self, distro_name: &str, tar_gz_filename: &str) -> Result<(), Error> {
        let mut distro_name = wide_chars::to_vec_u16(distro_name);
        let mut tar_gz_filename = wide_chars::to_vec_u16(tar_gz_filename);
        match self.raw_register_distribution(distro_name.as_mut_ptr(), tar_gz_filename.as_mut_ptr())
        {
            Ok(0) => Ok(()),
            Ok(hresult) => Err(format_err!("HRESULT == {:#08X}", hresult)),
            Err(e) => Err(format_err!("Wslapi::register_distro {}", e)),
        }
    }

    pub fn unregister_distro(&self, distro_name: &str) -> Result<(), Error> {
        let mut distro_name = wide_chars::to_vec_u16(distro_name);
        match self.raw_unregister_distribution(distro_name.as_mut_ptr()) {
            Ok(0) => Ok(()),
            Ok(hresult) => Err(format_err!("HRESULT == {:#08X}", hresult)),
            Err(e) => Err(format_err!("Wslapi::unregister_distro {}", e)),
        }
    }

    pub fn get_distro_configuration(
        &self,
        distro_name_original: &str,
    ) -> Result<DistroConfiguration, Error> {
        let mut distro_name = wide_chars::to_vec_u16(distro_name_original);
        match self.raw_get_distribution_configuration(distro_name.as_mut_ptr()) {
            Ok((0, version, default_uid, wsl_flags, ..)) => Ok(DistroConfiguration {
                name: distro_name_original.to_string(),
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
        let mut distro_name = wide_chars::to_vec_u16(distro_name);
        match self.raw_configure_distribution(
            distro_name.as_mut_ptr(),
            default_uid,
            distro_flags.bits,
        ) {
            Ok(0) => Ok(()),
            Ok(hresult) => Err(format_err!("HRESULT == {:#08X}", hresult)),
            Err(e) => Err(format_err!("Wslapi::configure_distro {}", e)),
        }
    }

    pub fn launch(&self, distro_name: &str, command: &str, use_cwd: bool) -> Result<DWORD, Error> {
        let mut distro_name = wide_chars::to_vec_u16(distro_name);
        let mut command = wide_chars::to_vec_u16(command);
        match self.raw_launch_interactive(distro_name.as_mut_ptr(), command.as_mut_ptr(), use_cwd) {
            Ok((0, exit_code)) => Ok(exit_code),
            Ok((hresult, _)) => Err(format_err!("HRESULT == {:#08X}", hresult)),
            Err(e) => Err(format_err!("Wslapi::launch {}", e)),
        }
    }
}
