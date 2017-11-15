use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::process;
use libloading;

const E_UNEXPECTED: u32 = 0x8000FFFF;

lazy_static!{
    static ref WSLAPI: libloading::Library = match libloading::Library::new("wslapi") {
        Ok(wslapi) => wslapi,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1)
        },
    };
}

fn to_vec_u16(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

fn raw_register_distribution(
    distro_name: *const u16,
    tar_gz_filename: *const u16,
) -> libloading::Result<u32> {
    unsafe {
        let wslapi_fn: libloading::Symbol<
            unsafe extern "C" fn(*const u16, *const u16) -> u32,
        > = WSLAPI.get(b"WslRegisterDistribution")?;
        Ok(wslapi_fn(distro_name, tar_gz_filename))
    }
}

fn raw_unregister_distribution(distro_name: *const u16) -> libloading::Result<u32> {
    unsafe {
        let wslapi_fn: libloading::Symbol<unsafe extern "C" fn(*const u16) -> u32> =
            WSLAPI.get(b"WslUnregisterDistribution")?;
        Ok(wslapi_fn(distro_name))
    }
}

pub fn register_distribution(distro_name: &str, tar_gz_filename: &str) -> u32 {
    let distro_name = to_vec_u16(distro_name);
    let tar_gz_filename = to_vec_u16(tar_gz_filename);
    match raw_register_distribution(distro_name.as_ptr(), tar_gz_filename.as_ptr()) {
        Ok(hresult) => hresult,
        Err(e) => {
            eprintln!("Error: {}", e);
            E_UNEXPECTED
        }
    }
}

pub fn unregister_distribution(distro_name: &str) -> u32 {
    let distro_name = to_vec_u16(distro_name);
    match raw_unregister_distribution(distro_name.as_ptr()) {
        Ok(hresult) => hresult,
        Err(e) => {
            eprintln!("Error: {}", e);
            E_UNEXPECTED
        }
    }
}
