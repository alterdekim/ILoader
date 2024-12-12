use iced::{widget::{button, container, text}, Length::Fill};

use crate::{theme, Message};


pub fn basic_btn(s: &str) -> button::Button<Message> {
    //button(text(str).center().font(theme::SF_FONT).size(13.0).line_height(16.)).width(109).height(22).style(theme::basic_button_theme)
    button(text(s).center().font(theme::SF_FONT).size(13.0).line_height(1.6)).width(109).style(theme::basic_button_theme)
}