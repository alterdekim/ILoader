use core::str;
use std::process::Command;

use regex::Regex;

pub fn list() {
    let r = Command::new("diskutil")
        .arg("list")
        .output()
        .expect("failed to access Disk Utility");
    if !r.status.success() { panic!("Unable to get result from Disk Utility"); }
    let a = str::from_utf8(&r.stdout).unwrap();
    for cap in Regex::new(r"\/dev\/.+\(external\, physical\):").unwrap().find_iter(a) {
        println!("{:#?}", cap);
        let b = &a[cap.end()..];
        // split the string and get before double \n.
        // then parse using diskutil info /dev/disk2
    }
}