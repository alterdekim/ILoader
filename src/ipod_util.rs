use crate::disk_util;

pub fn get_ipod_path() -> Option<String> {
    match disk_util::list() {
        Ok(l) => l.iter()
            .filter(|d| disk_util::is_ipod(d))
            .map(|d| disk_util::get_mount_point(d))
            .filter(|d| d.is_some())
            .map(|d| d.unwrap())
            .last(),
        Err(e) => None
    }
}