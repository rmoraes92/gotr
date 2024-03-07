use crate::{
    app, messages::{app::Message, pane_grid::PaneGridEvent}, states::{app::State, custom_gridpane::{self, CustomGridPane, PANE_LEFT}, 
    //custom_gridpane::CustomGridPane
}};
use iced::{widget::{container, pane_grid::{self, 
    // self, 
    Content}, responsive, scrollable, text, PaneGrid}, Element, Length, Renderer, Size};

use super::{commit_details, logs_column};

pub fn update(app)

pub fn view<'a>(state: &'a State) -> Element<'a, Message, Renderer> {
    let panegrid = PaneGrid::new(
        &state.main_window_panegrid_state.as_ref().unwrap(),
        |_pane, custom_pane, _is_maximized| {
            return Content::new(responsive(move |_size| {
                // view_content(id, total_panes, pane.is_pinned, size)
                handle_responsive(_pane, state.main_window_panegrid_pane_count, false, _size, state, custom_pane)
            }));
        }
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .spacing(10)
    .on_click(Message::PaneGridClicked)
    .on_drag(Message::PaneGridDragged)
    // .on_resize(10, Message::PaneGridResized);
    .on_resize(5, |pane| Message::PaneGrid(PaneGridEvent::Resized(pane)));

    return container(panegrid)
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
}

fn handle_responsive<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
    state: &'a State,
    custom_pane: &CustomGridPane,
) -> Element<'a, Message> {
    let body: Element<'_, Message> = if custom_pane.id == PANE_LEFT {
        scrollable(
            logs_column::view(state)
        ).into()
    } else {
        // TODO this needs to be scrollable horizontally
        commit_details::view(state)
    };

    return body;
}