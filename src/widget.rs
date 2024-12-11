use iced::{widget::{button, container, text}, Length::Fill};

use crate::{theme, Message};


pub fn basic_btn(str: &str) -> button::Button<Message> { // .font(theme::SF_FONT)
    button(container(text(str).center().size(13).line_height(16.)).center_x(Fill)).width(109).height(22).style(theme::basic_button_theme)
}