use iced::{widget::{button, container, text}, Length::Fill, Padding};

use crate::{theme, Message};


pub fn basic_btn(s: &str) -> button::Button<Message> {
    button(container(text(s).center().font(theme::SF_FONT).size(13.0).line_height(1.)).padding(Padding {
        top: 1.5,
        right: 2.,
        bottom: 1.5,
        left: 2.,
    })).style(theme::basic_button_theme)
}