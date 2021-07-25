#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use core::panic::PanicInfo;

use winapi::um::processthreadsapi::ExitProcess;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    let _rc = write_stdout("Hello world!");

    unsafe {
        ExitProcess(0);
    }
    #[allow(clippy::empty_loop)]
    loop {}
}

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

enum Error {
    WriteFileError,
}
