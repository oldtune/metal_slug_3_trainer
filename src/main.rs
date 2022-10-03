use std::{ffi::c_void, mem, ops::Range, ptr, thread, time::Duration};

use rand::rngs::ThreadRng;
use windows_sys::Win32::{
    Foundation::GetLastError,
    System::{
        Diagnostics::Debug::WriteProcessMemory,
        Threading::{OpenProcess, PROCESS_ALL_ACCESS},
    },
};

mod process_info;
fn main() {
    let mut rng = rand::thread_rng();

    let gun_address_pointer = 0x0056E478 as *const c_void;
    let process_id = process_info::process_id("neoragex").expect("No process found");
    let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, process_id) };

    //exclude 0x08 because it shoots nothing and 0x00 handgun.
    let guns = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
    ];
    let gun_count = guns.len() as u8;

    let gun_value: &mut [u8; 4] = &mut [0x00, 0x00, 0x07, 0xFF];
    let gun_value_pointer = gun_value as *const _ as *const u32;
    loop {
        let ran = random_number(&mut rng, 0..gun_count);
        let next_gun = guns[ran as usize];

        gun_value[2] = next_gun;
        unsafe {
            let result = WriteProcessMemory(
                handle,
                gun_address_pointer,
                gun_value_pointer as *const c_void,
                4,
                ptr::null::<usize>() as *mut usize,
            );
        };

        thread::sleep(Duration::new(3, 0));
    }
}

fn random_number(rng: &mut ThreadRng, range: Range<u8>) -> u8 {
    use rand::Rng;
    rng.gen_range(range)
}
