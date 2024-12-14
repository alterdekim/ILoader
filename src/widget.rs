use iced::{widget::{button, container, text}, Length::Fill, Padding};

use crate::{theme, Message};


pub fn basic_btn(s: &str) -> button::Button<Message> {
    button(container(text(s).center().font(theme::SF_FONT).size(13.0).line_height(1.)).padding(Padding {
        top: 6.,
        right: 8.,
        bottom: 6.,
        left: 8.,
    })).style(theme::basic_button_theme)
}