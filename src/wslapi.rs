use std::fmt;
use std::os::raw::{c_char, c_void};
use std::ffi::CStr;
use failure::Error;
use libloading::{Library, Symbol};
use libloading::Result as LibloadingResult;
use wide_chars;

type DWORD = u32;
type HRESULT = LONG;
type LONG = i32;
type LPVOID = *const c_void;
type PCWSTR = *mut u16;
type PSTR = *mut c_char;
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
    *const *const PSTR,
    &ULONG,
) -> HRESULT;
type ConfigureDistributionFn = unsafe extern "system" fn(PCWSTR, ULONG, WSL_DISTRIBUTION_FLAGS)
    -> HRESULT;
type LaunchInteractiveFn = unsafe extern "system" fn(PCWSTR, PCWSTR, bool, *const DWORD) -> HRESULT;
type IsDistributionRegisteredFn = unsafe extern "system" fn(PCWSTR) -> bool;

bitflags! {
    #[derive(Default)]
    pub struct DistroFlags: WSL_DISTRIBUTION_FLAGS {
        // const NONE = 0;
        const ENABLE_INTEROP = 1;
        const APPEND_NT_PATH = 2;
        const ENABLE_DRIVE_MOUNTING = 4;
    }
}

impl fmt::Display for DistroFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = vec![];
        if self.is_empty() {
            return write!(f, "NONE (0)");
        }
        if self.contains(DistroFlags::ENABLE_INTEROP) {
            v.push("ENABLE_INTEROP (1)");
        }
        if self.contains(DistroFlags::APPEND_NT_PATH) {
            v.push("APPEND_NT_PATH (2)");
        }
        if self.contains(DistroFlags::ENABLE_DRIVE_MOUNTING) {
            v.push("ENABLE_DRIVE_MOUNTING (4)");
        }
        write!(f, "{}", &v[..].join(" | "))
    }
}

pub struct DistroConfiguration {
    pub name: String,
    pub version: u32,
    pub default_uid: u32,
    pub flags: DistroFlags,
    pub default_environment_variables: Vec<String>,
}

impl DistroConfiguration {
    pub fn to_toml(&self) -> String {
        let flags_bits = self.flags.bits();
        format!(
            "[{}]
version = {}
default_uid = {}
# flags: {:#03b}
# {}
flags = {}
default_environment_values = [{}]",
            self.name,
            self.version,
            self.default_uid,
            flags_bits,
            self.flags,
            flags_bits,
            &self.default_environment_variables[..].join(", ")
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

    fn raw_co_task_mem_free(&self, v: LPVOID) -> LibloadingResult<()> {
        let raw_fn: Symbol<CoTaskMemFreeFn> = unsafe { self.ole32.get(b"CoTaskMemFree\0")? };
        unsafe { raw_fn(v) };
        Ok(())
    }

    fn raw_register_distribution(
        &self,
        distro_name: PCWSTR,
        tar_gz_filename: PCWSTR,
    ) -> LibloadingResult<HRESULT> {
        let raw_fn: Symbol<RegisterDistributionFn> =
            unsafe { self.wslapi.get(b"WslRegisterDistribution\0")? };
        Ok(unsafe { raw_fn(distro_name, tar_gz_filename) })
    }

    fn raw_unregister_distribution(&self, distro_name: PCWSTR) -> LibloadingResult<HRESULT> {
        let raw_fn: Symbol<UnregisterDistributionFn> =
            unsafe { self.wslapi.get(b"WslUnregisterDistribution\0")? };
        Ok(unsafe { raw_fn(distro_name) })
    }

    fn raw_get_distribution_configuration(
        &self,
        distro_name: PCWSTR,
    ) -> LibloadingResult<
        (
            HRESULT,
            ULONG,
            ULONG,
            WSL_DISTRIBUTION_FLAGS,
            *const *const PSTR,
            ULONG,
        ),
    > {
        let version = 0;
        let default_uid = 0;
        let wsl_flags = 0;
        let default_environment_variables_array = Box::into_raw(Box::new(0)) as *const *const PSTR;
        let default_environment_variables_count = 0;
        let raw_fn: Symbol<GetDistributionConfigurationFn> =
            unsafe { self.wslapi.get(b"WslGetDistributionConfiguration\0")? };
        let hresult = unsafe {
            raw_fn(
                distro_name,
                &version,
                &default_uid,
                &wsl_flags,
                default_environment_variables_array,
                &default_environment_variables_count,
            )
        };
        Ok((
            hresult,
            version,
            default_uid,
            wsl_flags,
            default_environment_variables_array,
            default_environment_variables_count,
        ))
    }

    fn raw_configure_distribution(
        &self,
        distro_name: PCWSTR,
        default_uid: ULONG,
        wsl_flags: WSL_DISTRIBUTION_FLAGS,
    ) -> LibloadingResult<HRESULT> {
        let raw_fn: Symbol<ConfigureDistributionFn> =
            unsafe { self.wslapi.get(b"WslConfigureDistribution\0")? };
        Ok(unsafe { raw_fn(distro_name, default_uid, wsl_flags) })
    }

    fn raw_launch_interactive(
        &self,
        distro_name: PCWSTR,
        command: PCWSTR,
        use_current_working_directory: bool,
    ) -> LibloadingResult<(HRESULT, DWORD)> {
        let exit_code = 0;
        let raw_fn: Symbol<LaunchInteractiveFn> =
            unsafe { self.wslapi.get(b"WslLaunchInteractive\0")? };
        let hresult = unsafe {
            raw_fn(
                distro_name,
                command,
                use_current_working_directory,
                &exit_code,
            )
        };
        Ok((hresult, exit_code))
    }

    fn raw_is_distribution_registered(&self, distro_name: PCWSTR) -> LibloadingResult<bool> {
        let raw_fn: Symbol<IsDistributionRegisteredFn> =
            unsafe { self.wslapi.get(b"WslIsDistributionRegistered\0")? };
        Ok(unsafe { raw_fn(distro_name) })
    }

    pub fn register_distro(&self, distro_name: &str, tar_gz_filename: &str) -> Result<(), Error> {
        match self.raw_register_distribution(
            wide_chars::to_vec_u16(distro_name).as_mut_ptr(),
            wide_chars::to_vec_u16(tar_gz_filename).as_mut_ptr(),
        ) {
            Ok(0) => Ok(()),
            Ok(hresult) => Err(format_err!("HRESULT == {:#08X}", hresult)),
            Err(e) => Err(format_err!("Wslapi::register_distro {}", e)),
        }
    }

    pub fn unregister_distro(&self, distro_name: &str) -> Result<(), Error> {
        match self.raw_unregister_distribution(wide_chars::to_vec_u16(distro_name).as_mut_ptr()) {
            Ok(0) => Ok(()),
            Ok(hresult) => Err(format_err!("HRESULT == {:#08X}", hresult)),
            Err(e) => Err(format_err!("Wslapi::unregister_distro {}", e)),
        }
    }

    pub fn get_distro_configuration(
        &self,
        distro_name: &str,
    ) -> Result<DistroConfiguration, Error> {
        match self.raw_get_distribution_configuration(
            wide_chars::to_vec_u16(distro_name).as_mut_ptr(),
        ) {
            Ok((
                0,
                version,
                default_uid,
                wsl_flags,
                default_environment_variables_array,
                default_environment_variables_count,
            )) => {
                let original_array = unsafe { *default_environment_variables_array } as *const PSTR;
                let p_vec = (0..default_environment_variables_count)
                    .map(|i| unsafe { *original_array.offset(i as isize) })
                    .collect::<Vec<PSTR>>();
                let string_vec = p_vec
                    .iter()
                    .map(|p| {
                        format!(
                            "\"{}\"",
                            unsafe { CStr::from_ptr(*p as *const c_char) }
                                .to_str()
                                .unwrap()
                        )
                    })
                    .collect::<Vec<String>>();
                for p in p_vec {
                    self.raw_co_task_mem_free(p as LPVOID).unwrap();
                }
                self.raw_co_task_mem_free(original_array as LPVOID).unwrap();
                Ok(DistroConfiguration {
                    name: distro_name.to_string(),
                    version: version,
                    default_uid: default_uid,
                    flags: DistroFlags::from_bits(wsl_flags as WSL_DISTRIBUTION_FLAGS).unwrap(),
                    default_environment_variables: string_vec,
                })
            }
            Ok((hresult, ..)) => Err(format_err!("HRESULT == {:#08X}", hresult)),
            Err(e) => Err(format_err!("Wslapi::get_distro_configuration {}", e)),
        }
    }

    pub fn configure_distro(
        &self,
        distro_configuration: &DistroConfiguration,
    ) -> Result<(), Error> {
        match self.raw_configure_distribution(
            wide_chars::to_vec_u16(&distro_configuration.name[..]).as_mut_ptr(),
            distro_configuration.default_uid,
            distro_configuration.flags.bits,
        ) {
            Ok(0) => Ok(()),
            Ok(hresult) => Err(format_err!("HRESULT == {:#08X}", hresult)),
            Err(e) => Err(format_err!("Wslapi::configure_distro {}", e)),
        }
    }

    pub fn launch(&self, distro_name: &str, command: &str, use_cwd: bool) -> Result<DWORD, Error> {
        match self.raw_launch_interactive(
            wide_chars::to_vec_u16(distro_name).as_mut_ptr(),
            wide_chars::to_vec_u16(command).as_mut_ptr(),
            use_cwd,
        ) {
            Ok((0, exit_code)) => Ok(exit_code),
            Ok((hresult, _)) => Err(format_err!("HRESULT == {:#08X}", hresult)),
            Err(e) => Err(format_err!("Wslapi::launch {}", e)),
        }
    }

    pub fn is_distribution_registered(&self, distro_name: &str) -> Result<bool, Error> {
        match self.raw_is_distribution_registered(wide_chars::to_vec_u16(distro_name).as_mut_ptr())
        {
            Ok(result) => Ok(result),
            Err(e) => Err(format_err!("Wslapi::is_distribution_registered {}", e)),
        }
    }
}
