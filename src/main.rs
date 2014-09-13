
#![feature(globs)]

extern crate winapi;
extern crate time;

use std::io::{Append, Write};
use std::io::process::{Command, Ignored};
use std::io::fs::File;
use std::io::timer::sleep;
use std::mem;
use std::str::from_utf8;
use std::time::duration::Duration;
use time::precise_time_s;
use winapi::*;

unsafe fn timings(name: &str, source: &str, out: &str) {
    let outp = Path::new(out);
    //.stdout(Ignored)
    let process = Command::new(name).arg(source).arg("-Ztime-passes").env("CFG_VERSION", "shitversion").env("CFG_RELEASE", "shitrelease").spawn().unwrap();
    let id = process.id();
    spawn(proc() {
        let hand = OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, id);
        let mut memc: PROCESS_MEMORY_COUNTERS = mem::zeroed();
        let size = mem::size_of_val(&memc) as DWORD;
        memc.cb = size;
        let mut file = File::create(&outp);
        (writeln!(file, "time, WorkingSetSize, PagefileUsage, lpKernelTime, lpUserTime")).unwrap();
        let mut creation = mem::zeroed();
        let mut exit = mem::zeroed();
        let mut kernel = mem::zeroed();
        let mut user = mem::zeroed();
        let start = precise_time_s();
        loop {
            if K32GetProcessMemoryInfo(hand, &mut memc, size) == 0 {
                fail!("Can't get memory info!")
            }
            if GetProcessTimes(hand, &mut creation, &mut exit, &mut kernel, &mut user) == 0 {
                fail!("Can't get process timings!")
            }
            let time = precise_time_s() - start;
            let ktime = (kernel.dwHighDateTime as u64 << 32) | (kernel.dwLowDateTime as u64);
            let utime = (user.dwHighDateTime as u64 << 32) | (user.dwLowDateTime as u64);
            (writeln!(file, "{}, {}, {}, {}, {}", time, memc.WorkingSetSize, memc.PagefileUsage, ktime, utime)).unwrap();
            sleep(Duration::milliseconds(10));
            if memc.PagefileUsage == 0 {
                break
            }
        }
    });
    let output = process.wait_with_output().unwrap();
    let mut file = File::open_mode(&Path::new("timings.txt"), Append, Write);
    (writeln!(file, "{}", out)).unwrap();
    (writeln!(file, "{}", from_utf8(output.output.as_slice()).unwrap_or("Invalid UTF-8"))).unwrap();
    (writeln!(file, "{}", from_utf8(output.error.as_slice()).unwrap_or("Invalid UTF-8"))).unwrap();
}

fn main() {
    unsafe { timings(r"A:\msys64\home\retep998\rust\x86_64-w64-mingw32\stage2\bin\rustc.exe", r"A:\msys64\home\retep998\rust\src\librustc\lib.rs", "after.csv") }
    unsafe { timings(r"A:\rust64\bin\rustc.exe", r"A:\msys64\home\retep998\rust\src\librustc\lib.rs", "before.csv") }
}
