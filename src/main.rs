mod disk_util;
mod ipod_util;

const VENDOR_ID: u16 = 1452;
const PRODUCT_ID: u16 = 4617;

fn main() {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if VENDOR_ID == device_desc.vendor_id() && PRODUCT_ID == device_desc.product_id() {
            println!("FOUND!");
            println!("{}", ipod_util::get_ipod_path().is_some());
        }
    }
}
