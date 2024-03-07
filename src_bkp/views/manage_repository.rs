use iced::{
    widget::*, Element, Length
};

use crate::{messages::app::Message, states::app::State, views::commit_details};

use super::logs_column;

pub fn view<'a>(s: &'a State) -> Element<'a, Message> {
    container(
        row![
            scrollable(
                logs_column::view(s)
            ),
            commit_details::view(s),
        ],
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}
