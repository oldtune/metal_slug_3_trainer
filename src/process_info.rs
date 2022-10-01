use sysinfo::{PidExt, Process, ProcessExt, System, SystemExt};

pub fn process_id(name: &str) -> Option<u32> {
    let system = System::new_all();
    let process: Option<&Process> = system.processes_by_name(name).nth(0);
    match process {
        Some(p) => Some(p.pid().as_u32()),
        None => None,
    }
}
