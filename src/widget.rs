use iced::{widget::{button, column, container, row, text, Column}, Element, Length::Fill, Padding, Task as Command};

use crate::{theme, Message};


pub fn basic_btn(s: String) -> button::Button<'static, Message> {
    button(container(text(s).center().font(theme::SF_FONT).size(13.0).line_height(1.)).padding(Padding {
        top: 1.5,
        right: 2.,
        bottom: 1.5,
        left: 2.,
    })).style(theme::basic_button_theme)
}

// the value T should be something, that inherits ActionWindow
#[derive(Clone)]
pub enum SidebarTab {
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

impl Into<String> for SidebarTab {
    fn into(self) -> String {
        match self {
            SidebarTab::Youtube(a) => a,
            SidebarTab::Spotify(a) => a,
            SidebarTab::Soundcloud(a) => a,
            SidebarTab::ITunes(a) => a,
            SidebarTab::Playlists(a) => a,
            SidebarTab::FileSystem(a) => a,
            SidebarTab::Metadata(a) => a,
            SidebarTab::FindCopies(a) => a,
            SidebarTab::Settings(a) => a,
        }
    }
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

    pub fn push_tab(&mut self, t: (SidebarTab, Box<dyn ActionWindow>)) {
        self.tabs.push(t);
    }
}

impl From<&SidebarGroup> for Element<'_, Message> {
    fn from(value: &SidebarGroup) -> Self {
        container(column![
            text(value.name.clone()),
            column(
                value.tabs
                    .iter()
                    .map(|i| {
                        let s: String = i.0.clone().into();
                        basic_btn(s).into()
                    })
                    .collect::<Vec<Element<Message>>>()
            )
        ]).into()
    }
}

pub trait ActionWindow {
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
                self.sidebar.iter()
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

    pub fn push_group(&mut self, group: SidebarGroup) {
        self.sidebar.push(group);
    }

    pub fn new() -> Self {
        Self{ sidebar: Vec::new(), selected: None } 
    }
}