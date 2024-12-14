use iced::{widget::{button, container, text}, Length::Fill, Padding, Task as Command, Element};

use crate::{theme, Message};


pub fn basic_btn(s: &str) -> button::Button<Message> {
    button(container(text(s).center().font(theme::SF_FONT).size(13.0).line_height(1.)).padding(Padding {
        top: 1.5,
        right: 2.,
        bottom: 1.5,
        left: 2.,
    })).style(theme::basic_button_theme)
}

// the value T should be something, that inherits ActionWindow
enum SidebarTab<T> where T: ActionWindow {
    Youtube(T),
    /*Spotify(T),
    Soundcloud(T),
    ITunes(T),
    Playlists(T),
    FileSystem(T),
    Metadata(T),
    FindCopies(T),*/
    Settings(SettingsWindow)
}

pub struct SidebarGroup {
    tabs: Vec<SidebarTab<dyn ActionWindow>>,
    name: String
}

impl SidebarGroup {
    pub fn new(name: String) -> Self {
        Self{tabs: Vec::new(), name}
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

trait ActionWindow {
    fn view(&self) -> Element<Message>;
    fn update(&mut self, message: Message) -> Command<Message>;
}

pub struct SettingsWindow {}

impl ActionWindow for SettingsWindow {
    fn view(&self) -> Element<Message> {
        todo!()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        todo!()
    }
}

pub struct SplitView<T> where T: ActionWindow {
    sidebar: Vec<SidebarGroup>,
    main: Option<T>
}

impl<T: ActionWindow> SplitView<T> {
    pub fn view(&self) -> Element<Message> {

    }

    pub fn update(&mut self, message: Message) -> Command<Message> {

    }

    pub fn new() -> Self {
        Self{ sidebar: Vec::new(), main: None } 
    }
}