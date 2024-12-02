use std::{str, process::Command, error::Error, str::FromStr};
use regex::Regex;

pub fn list() -> Result<Vec<String>, Box<dyn Error>> {
    let mut disks = Vec::new();
    let r = match Command::new("diskutil").arg("list").output() {
        Ok(s) => s,
        Err(e) => return Err(Box::new(e))
    };
    if !r.status.success() { return Ok(disks); }
    let rg = Regex::new(r"\d:.+   [a-zA-Z0-9].+").unwrap();
    let a = match str::from_utf8(&r.stdout) {
        Ok(r) => r,
        Err(e) => return Err(Box::new(e))
    };
    for cap in Regex::new(r"\/dev\/.+\(external\, physical\):").unwrap().find_iter(a) {
        let mut b = &a[cap.end()..];
        let i = match b.find("\n\n") {
            Some(r) => r,
            None => return Ok(disks)
        };
        b = &b[..i];
        for gap in rg.find_iter(b) {
            let j = match gap.as_str().rfind(" ") {
                Some(r) => r + 1,
                None => return Ok(disks)
            };

            let g= &gap.as_str()[j..];
            disks.push(String::from_str(g).unwrap());
        }
    }
    Ok(disks)
}

pub fn is_ipod(name: &str) -> bool {
    let r = match Command::new("diskutil").arg("info").arg(name).output() {
        Ok(s) => s,
        Err(_e) => return false
    };
    if !r.status.success() { return false; }
    let a = match str::from_utf8(&r.stdout) {
        Ok(r) => r,
        Err(_e) => return false
    };
    let cap = Regex::new(r"Media Type:.+\n").unwrap().find(a);
    if let Some(g) = cap {
        let mut b = g.as_str();
        let f = b.rfind(" ").unwrap() + 1;
        b = &b[f..b.len()-1];
        return b == "iPod";
    }
    false
}

pub fn get_mount_point(name: &str) -> Option<String> {
    let r = match Command::new("diskutil").arg("info").arg(name).output() {
        Ok(s) => s,
        Err(_e) => return None
    };
    if !r.status.success() { return None; }
    let a = match str::from_utf8(&r.stdout) {
        Ok(r) => r,
        Err(_e) => return None
    };
    let cap = Regex::new(r"Mount Point:.+\n").unwrap().find(a);
    match cap {
        Some(g) => {
            let i = g.as_str();
            let j = i.rfind(" ").unwrap() + 1;
            Some(i[j..i.len()-1].to_string())
        },
        None => None
    }
}