use iced::{widget::{button, column, container, row, text, Column}, Element, Length::Fill, Padding, Task as Command};

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

/*impl Into<Element<'_, Message>> for SidebarGroup {
    fn into(self) -> Element<'static, Message> {
        container(row![
            text!(),

        ]).into()
    }
}*/

impl From<SidebarGroup> for Element<'_, Message> {
    fn from(value: SidebarGroup) -> Self {
        container(row![
            text(value.name),
            column(
                value.tabs
                    .iter()
                    .map(|i| container(text(i.0)))
                    .collect()
            )
        ]).into()
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
        row![
            column(
                self.sidebar.iter_mut()
                    .map(|f| f.into())
                    .collect::<Vec<Element<Message>>>()
            ),
            if self.selected.is_some() { container((&self.selected.as_ref().unwrap().1).view()) } else { container(text!("")) }
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