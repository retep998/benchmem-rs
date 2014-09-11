
#![feature(globs)]

extern crate winapi;
extern crate time;

use std::io::process::Command;
use std::io::fs::File;
use std::io::timer::sleep;
use std::mem;
use std::str::from_utf8;
use std::time::duration::Duration;
use time::precise_time_s;
use winapi::*;

unsafe fn timings() {
    // r"A:\msys64\home\retep998\rust\x86_64-w64-mingw32\stage2\bin\rustc.exe"
    // r"A:\rust64\bin\rustc.exe"
    let process = Command::new(r"A:\rust64\bin\rustc.exe").arg(r"A:\msys64\home\retep998\rust\src\librustc\lib.rs").arg("-Ztime-passes").env("CFG_VERSION", "shitversion").env("CFG_RELEASE", "shitrelease").spawn().unwrap();
    let id = process.id();
    spawn(proc() {
        let hand = OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, id);
        let mut memc: PROCESS_MEMORY_COUNTERS_EX = mem::zeroed();
        let size = mem::size_of_val(&memc) as DWORD;
        memc.cb = size;
        let mut file = File::create(&Path::new("dump.csv"));
        (writeln!(file, "time, WorkingSetSize, PagefileUsage")).unwrap();
        let start = precise_time_s();
        loop {
            if K32GetProcessMemoryInfo(hand, mem::transmute(&memc), size) == 0 {
                fail!("Can't get memory info!")
            }
            let time = precise_time_s() - start;
            (writeln!(file, "{}, {}, {}", time, memc.WorkingSetSize, memc.PagefileUsage)).unwrap();
            sleep(Duration::milliseconds(1));
            if memc.PrivateUsage == 0 {
                break
            }
        }
    });
    let output = process.wait_with_output().unwrap();
    println!("====stdout====");
    println!("{}", from_utf8(output.output.as_slice()));
    println!("====stderr====");
    println!("{}", from_utf8(output.error.as_slice()));
}

fn main() {
    unsafe { timings() }
}