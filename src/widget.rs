use iced::{widget::{button, column, container, row, text}, Element, Length::Fill, Padding, Task as Command};

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
enum SidebarTab {
    Youtube(String),
    Spotify(String),
    Soundcloud(String),
    ITunes(String),
    Playlists(String),
    FileSystem(String),
    Metadata(String),
    FindCopies(String),
    Settings(String)
}

pub struct SidebarGroup {
    tabs: Vec<(SidebarTab, Box<dyn ActionWindow>)>,
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

pub struct YTWindow {}

impl ActionWindow for YTWindow {
    fn view(&self) -> Element<Message> {
        todo!()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        todo!()
    }
}

pub struct SplitView {
    sidebar: Vec<SidebarGroup>,
    selected: Option<(SidebarTab, Box<dyn ActionWindow>)>
}

impl SplitView {
    pub fn view(&self) -> Element<Message> {
        if self.selected.is_none() {
            return container("hey").into();
        }
        let a = &self.selected.as_ref().unwrap().1;
        row![
            column! [container("lol")],
            if self.selected.is_some() { container(a.view()) } else { container(text!("None")) }
        ].into()
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        if self.selected.is_some() {
            let a = &mut self.selected.as_mut().unwrap().1;
            return a.update(message);
        }
        Command::none()
    }

    pub fn new() -> Self {
        Self{ sidebar: Vec::new(), selected: None } 
    }
}