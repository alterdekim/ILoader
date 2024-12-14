use iced::theme::Palette;
use iced::widget::button::{Status, Style};
use iced::{Background, Border, Color};
use iced::Length::Fill;
use iced::{widget::container, window, Theme, Element, Settings, Task as Command};
use iced::widget::{button, column, pick_list, radio, row, scrollable, text, Column, Container};

mod disk_util;
mod ipod_util;

mod theme;
mod widget;

const VENDOR_ID: u16 = 1452;
const PRODUCT_ID: u16 = 4617;


#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(u8),
    ChangeUI
}


struct State {
}

impl State {
    fn new() -> Self {
        Self {  }
    }
}

enum App {
    Preloaded,
    Loaded(State)
}

impl App {
    pub fn new() -> (Self, Command<Message>) {
        (Self::Preloaded, Command::done(Message::ChangeUI))
    }

    pub fn view(&self) -> Element<Message> {
        match self {
            App::Preloaded => {
                return container(text("Loading")).into();
            }
            App::Loaded(state) => {
                //return state.tab_panel.view();
                
                return container(row![ widget::basic_btn("About iLoader") ]).into();
            }
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            App::Preloaded => {
                *self = App::Loaded(State {});
                return Command::done(Message::ChangeUI);
            }
            App::Loaded(state) => {
                //state.tab_panel.update(message);
            }
        }
        Command::none()
    }

    fn theme(&self) -> Theme {
        theme::get_default_theme()
    }
}

fn main() -> iced::Result {
    /*for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if VENDOR_ID == device_desc.vendor_id() && PRODUCT_ID == device_desc.product_id() {
            println!("FOUND!");
            println!("{}", ipod_util::get_ipod_path().is_some());
        }
    }*/
    // sidebar width 200px
    iced::application("iLoader", App::update, App::view)
        .theme(App::theme)
        //.transparent(true)
        .window_size((980.0, 700.0))
        .run_with(App::new)
}
