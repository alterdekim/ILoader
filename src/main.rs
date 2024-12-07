use cacao::appkit::window::{TitleVisibility, Window, WindowConfig, WindowController};
use cacao::appkit::{App, AppDelegate};
use cacao::button::Button;
use cacao::view::{SplitViewController, View, ViewDelegate};

use crate::view::details_view::Details;
use crate::view::sidebar::MainSidebar;
use crate::view::content_view::ScreenView;
use crate::window::main_window::MainWindow;

mod disk_util;
mod ipod_util;

mod view;
mod window;

const VENDOR_ID: u16 = 1452;
const PRODUCT_ID: u16 = 4617;

struct ILoaderApp {
    window: WindowController<MainWindow>
}

impl AppDelegate for ILoaderApp {
    /// There should be stuff which loads underlying logic for communication with IPod
    fn will_finish_launching(&self) {}

    fn did_finish_launching(&self) {
        App::activate();
        self.window.show();
    }

    fn should_terminate_after_last_window_closed(&self) -> bool {
        true
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
    let config = WindowConfig::default();

    App::new("com.alterdekim.iloader", ILoaderApp {
        window: WindowController::with(config, MainWindow::default())
    }).run();
}
