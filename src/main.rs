
extern crate kernel32;
extern crate winapi;

use kernel32::*;
use std::fs::File;
use std::io::{Error, Write};
use std::mem;
use std::process::{Command};
use winapi::*;

trait Filetime {
    fn to_u64(&self) -> u64;
}
impl Filetime for FILETIME {
    fn to_u64(&self) -> u64 {
        ((self.dwHighDateTime as u64) << 32) | (self.dwLowDateTime as u64)
    }
}
unsafe fn timings(mut command: Command, out: &str) {
    let mut process = command.spawn().unwrap();
    let id = process.id();
    let hand = OpenProcess(PROCESS_QUERY_INFORMATION | SYNCHRONIZE , FALSE, id);
    let mut memc: PROCESS_MEMORY_COUNTERS = mem::zeroed();
    let size = mem::size_of_val(&memc) as DWORD;
    memc.cb = size;
    let mut file = File::create(&out).unwrap();
    writeln!(&mut file, "time, WorkingSetSize, PagefileUsage, KernelTime, UserTime").unwrap();
    let mut creation = mem::zeroed();
    let mut exit = mem::zeroed();
    let mut kernel = mem::zeroed();
    let mut user = mem::zeroed();
    let mut current = mem::zeroed();
    loop {
        assert!(K32GetProcessMemoryInfo(hand, &mut memc, size) != 0);
        assert!(GetProcessTimes(hand, &mut creation, &mut exit, &mut kernel, &mut user) != 0);
        GetSystemTimeAsFileTime(&mut current);
        let ktime = kernel.to_u64();
        let utime = user.to_u64();
        let ctime = current.to_u64();
        let stime = creation.to_u64();
        (writeln!(file, "{}, {}, {}, {}, {}", ctime - stime, memc.WorkingSetSize, memc.PagefileUsage, ktime, utime)).unwrap();
        match WaitForSingleObject(hand, 10) {
            WAIT_TIMEOUT => (),
            WAIT_OBJECT_0 => {
                assert!(GetProcessTimes(hand, &mut creation, &mut exit, &mut kernel, &mut user) != 0);
                let stime = creation.to_u64();
                let etime = exit.to_u64();
                println!("CreationTime: {}", stime);
                println!("ExitTime: {}", etime);
                println!("Total: {}", ((etime - stime) as f64) / 10_000.0);
                let mut cycles = 0;
                assert!(QueryProcessCycleTime(hand, &mut cycles) != 0);
                println!("Cycles: {}", cycles);
                break
            },
            WAIT_FAILED => {
                let err = Error::last_os_error();
                panic!("{:?}", err)
            },
            x => {
                unreachable!("{}", x)
            },
        };
    }
    let _status = process.wait().unwrap();
}

fn main() {
    unsafe {
        timings(Command::new(r"C:\msys64\home\Peter\hello-rs\windows\target\release\hello.exe"), "time.csv");
    }
}
