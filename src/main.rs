use std::{ffi::c_void, ptr};

use windows_sys::Win32::{
    Foundation::GetLastError,
    System::{
        Diagnostics::Debug::WriteProcessMemory,
        Threading::{OpenProcess, PROCESS_ALL_ACCESS},
    },
};

mod process_info;
fn main() {
    let gun_address_pointer = 0x0056E478 as *const c_void;
    let process_id = process_info::process_id("neoragex").expect("No process found");
    let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, process_id) };
    let gun_value_pointer = &(0xFF070000 as usize) as *const usize;

    unsafe {
        let result = WriteProcessMemory(
            handle,
            gun_address_pointer,
            gun_value_pointer as *const c_void,
            4,
            ptr::null::<usize>() as *mut usize,
        );

        println!("{}", result);
    };

    let err_code = unsafe { GetLastError() };
    println!("{}", err_code);
}
