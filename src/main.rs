#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    use winapi::um::processthreadsapi::ExitProcess;

    let _rc = write_stdout("Hello world!");

    unsafe {
        ExitProcess(0);
    }
    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(target_os = "windows")]
fn write_stdout(text: &str) -> Result<(), Error> {
    use winapi::{
        ctypes::c_void,
        shared::minwindef::{DWORD, TRUE},
        um::{
            fileapi::WriteFile, processenv::GetStdHandle, winbase::STD_OUTPUT_HANDLE, winnt::HANDLE,
        },
    };

    let result = unsafe {
        let stdout: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE);

        let mut written: DWORD = 0;

        WriteFile(
            stdout,
            text.as_ptr() as *const c_void,
            text.len() as u32,
            &mut written,
            core::ptr::null_mut(),
        )
    };

    if result == TRUE {
        Ok(())
    } else {
        Err(Error::WriteFileError)
    }
}

#[cfg(target_os = "linux")]
#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    use libc::exit;

    let _rc = write_stdout("Hello, world!\n\0");
    unsafe {
        exit(0);
    }
}

#[cfg(target_os = "linux")]
fn write_stdout(text: &str) -> Result<(), Error> {
    use libc::printf;

    // NOTE(sen) `printf` expects null-terminated strings
    let result = unsafe { printf(text.as_ptr() as *const _) };

    if result as usize == text.len() - 1 {
        Ok(())
    } else {
        Err(Error::WriteFileError)
    }
}

enum Error {
    WriteFileError,
}
