use cacao::appkit::window::{TitleVisibility, Window, WindowConfig};
use cacao::appkit::{App, AppDelegate};
use cacao::color::Color;
use cacao::view::View;

use content_view::ScreenView;

mod disk_util;
mod ipod_util;

mod content_view;

const VENDOR_ID: u16 = 1452;
const PRODUCT_ID: u16 = 4617;

struct ILoaderApp {
    window: Window,
    content: View<ScreenView>
}

impl AppDelegate for ILoaderApp {
    /// There should be stuff which loads underlying logic for communication with IPod
    fn will_finish_launching(&self) {}

    fn did_finish_launching(&self) {
        App::activate();

        self.window.set_title("ILoader");
        self.window.set_title_visibility(TitleVisibility::Hidden);
        self.window.set_titlebar_appears_transparent(true);
        self.window.set_movable_by_background(true);
        self.window.set_autosave_name("CacaoILoader");
        self.window.set_content_view(&self.content);
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
        window: Window::new(config),
        content: View::with(ScreenView::new())
    }).run();
}
