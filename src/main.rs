use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::Window;
use cacao::color::Color;

mod disk_util;
mod ipod_util;

const VENDOR_ID: u16 = 1452;
const PRODUCT_ID: u16 = 4617;

#[derive(Default)]
struct BasicApp {
    window: Window
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        self.window.set_minimum_content_size(400., 400.);
        self.window.set_title("Hello World!");
        self.window.set_background_color(Color::rgb(255, 0,0));
        self.window.show();
    }
}

fn main() {
    /*for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if VENDOR_ID == device_desc.vendor_id() && PRODUCT_ID == device_desc.product_id() {
            println!("FOUND!");
            println!("{}", ipod_util::get_ipod_path().is_some());
        }
    }*/
    App::new("com.hello.world", BasicApp::default()).run();
}
