pub mod state;
pub mod message;
pub mod views;

pub use state::MainWindowState;
// pub use message::Message;

use iced;
use crate::globals;
use crate::git;

pub const APP_NAME: &str = "main_window";

pub fn update(
    state: &mut MainWindowState,
    msg: message::Message,
) -> iced::Command<globals::Message> {
    println!("updating main window");
    let cmd_none = iced::Command::none();
    match msg {
        message::Message::IncrementCounter => {
            state.counter += 1;
        },
        message::Message::PaneGridEvent(
            globals::PaneGridEvent::Resized(iced::widget::pane_grid::ResizeEvent{ split, ratio })
        ) => {
            state.panegrid_state.resize(split, ratio);
        },
        _ => (),
    };
    cmd_none
}

// TODO we might want to set <Into<MainWindowState>> 
pub fn view<'a>(state: &MainWindowState) -> iced::Element<'a, globals::Message> {
    // let c: usize = state.counter;
    let repo_path: &String = state.repository_path.as_ref().unwrap();
    let repo: git2::Repository = git::open(repo_path);
    iced::widget::container(
            iced::widget::row![
                views::commit_list_column::view(&repo),
                // iced::widget::text(format!("Clicked {} times", c)),
                // iced::widget::button("Increment Counter")
                //     .on_press(
                //         globals::Message::MainWindow(
                //             Message::IncrementCounter
                //         )
                //     ),
            ]
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x()
        .center_y()
        .into()
}

pub fn view2<'a>(state: &'a MainWindowState) -> iced::Element<'a, globals::Message> {
    let panegrid: iced::widget::PaneGrid<'_, globals::Message> = iced::widget::PaneGrid::new(
        &state.panegrid_state,
        |pane: iced::widget::pane_grid::Pane, custom_pane: &state::MyPane, _is_maximized: bool| {
            return iced::widget::pane_grid::Content::new(
                iced::widget::responsive(move |size| {
                // view_content(id, total_panes, pane.is_pinned, size)
                    handle_responsive(
                        state,
                        pane,
                        custom_pane.orientation.clone(),
                        size,
                    )
                })
            );
        }
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .spacing(10)
    .on_click(|pane: iced::widget::pane_grid::Pane| {
        globals::Message::MainWindow(
            message::Message::PaneGridEvent(
                globals::PaneGridEvent::Clicked(pane)
            )
        )
    })
    .on_drag(|event| {
        globals::Message::MainWindow(
            message::Message::PaneGridEvent(
                globals::PaneGridEvent::Dragged(event)
            )
        )
    })
    // // .on_resize(10, Message::PaneGridResized);
    .on_resize(
        5,
        |event| {
            globals::Message::MainWindow(
                message::Message::PaneGridEvent(
                    globals::PaneGridEvent::Resized(event)
                )
            )
        }
    )
    ;

    iced::widget::container(panegrid)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
}

/// iced::widget::pane_grid::Pane for event triggers
pub fn handle_responsive<'a>(
    state: &state::MainWindowState,
    _pane: iced::widget::pane_grid::Pane,
    pane_orientation: state::PaneOrientation,
    _size: iced::Size,
) -> iced::Element<'a, globals::Message>{
    match pane_orientation {
        state::PaneOrientation::Left => {
            let repo_path: &String = state.repository_path.as_ref().unwrap();
            let repo: git2::Repository = git::open(repo_path);
            views::commit_list_column::view(&repo)
        },
        state::PaneOrientation::Right => iced::widget::text("right panel").into(),
    }
}
