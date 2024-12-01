use rusb::{Device, DeviceDescriptor, GlobalContext};

fn main() {
    // vendor_id: 3141
    // product_id: 32777

    loop {
        for device in rusb::devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            println!("VendorId: {:?}; productId: {:?}", device_desc.vendor_id(), device_desc.product_id());
        }
    }
}
