use iced::{
    widget::{container, text},
    alignment,
    Length,
    Element,
};

use crate::messages::app::Message;

pub fn view<'a>() -> Element<'a, Message> {
    container(
        text("Loading...").horizontal_alignment(alignment::Horizontal::Center).size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}