use iced::theme::Palette;
use iced::widget::button::{Status, Style};
use iced::{Background, Border, Color, Font};
use iced::Length::Fill;
use iced::{widget::container, window, Theme, Element, Settings, Task as Command};
use iced::widget::{button, column, pick_list, radio, text, Column, Container, scrollable};

pub const SF_FONT: Font = Font {
    family: iced::font::Family::Name("SF Pro Text"),
    weight: iced::font::Weight::Medium,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal
};

pub fn get_default_theme() -> Theme {
    Theme::custom("Glossy".to_string(), Palette {
        background: Color::from_rgb8(255, 255, 255),
        text: Color::from_rgb8(0, 0, 0),
        primary: Color::from_rgb8(0, 122, 255),
        success: Color::from_rgb8(52, 199, 89),
        danger: Color::from_rgb8(255, 59, 48),
    })
}

pub fn basic_button_theme(theme: &Theme, _status: Status) -> Style {
    Style {
        background: Some(Background::Color(Color::from_rgb8(0, 122, 255))),
        text_color: theme.palette().text,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 5.0.into(),
        },
        ..Default::default()
    }
}