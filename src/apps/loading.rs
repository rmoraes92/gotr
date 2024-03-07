use crate::globals;

pub const APP_NAME: &str = "loading";

pub fn view<'a>() -> iced::Element<'a, globals::Message> {
    iced::widget::container(
        iced::widget::text("fake loading...")
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .center_x()
    .center_y()
    .into()
}
