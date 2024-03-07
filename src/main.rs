use gotr::apps;
use gotr::globals;
use iced::Application;
use iced;

static INITIAL_WIDTH: f32 = 800.0;
static INITIAL_HEIGHT: f32 = 600.0;

pub fn main() -> iced::Result {
    App::run(iced::Settings {
        window: iced::window::Settings {
            size: iced::Size::new(INITIAL_WIDTH, INITIAL_HEIGHT),
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    })
}

enum App {
    Loading,
    SelectRepository(globals::State),
    MainWindow(globals::State),
}

type State = globals::State;

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = globals::Message;
    type Theme = iced::Theme;
    type Flags = ();
    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        // TODO wtf is flags for?
        (
            App::Loading,
            iced::Command::batch(vec![iced::Command::perform(
                State::load(),
                |state| {
                    Self::Message::NavigateTo(apps::select_repository::APP_NAME.to_string(), state)
                },
            )]),
            // TODO Is it possible to NOT pass a Command?
            // Command::none() // TODO Yes it is
        )
    }
    fn title(&self) -> String {
        format!("Git On The Rocks - {}", "random-repo-name")
    }
    fn update(&mut self, msg: Self::Message) -> iced::Command<Self::Message> {
        match &msg {
            // String, Self::State
            Self::Message::NavigateTo(app_name, state) => {
                match app_name.as_str() {
                    apps::select_repository::APP_NAME => {
                        *self = App::SelectRepository(state.clone())
                    }
                    apps::main_window::APP_NAME => {
                        *self = App::MainWindow(state.clone())
                    }
                    apps::loading::APP_NAME => *self = App::Loading,
                    _ => *self = App::Loading,
                }
            }
            _ => (),
        }
        let new_state;
        let mut cmd = iced::Command::none();
        match self {
            Self::Loading => {
                println!("loading configuration");
            }
            Self::SelectRepository(state) => {
                (new_state, cmd) =
                    apps::select_repository::update(state, msg.clone());
                match new_state {
                    Some(s) => *self = Self::SelectRepository(s),
                    _ => (),
                }
            }
            Self::MainWindow(state) => {
                match msg {
                    globals::Message::MainWindow(app_msg) => {
                        cmd =
                        apps::main_window::update(state.main_window.as_mut().unwrap(), app_msg);
                        *self = Self::MainWindow(state.clone());
                    },
                    _ => (),
                }
            },
        }
        return cmd;
    }
    fn view(&self) -> iced::Element<Self::Message> {
        match self {
            App::Loading => apps::loading::view(),
            App::SelectRepository(state) => {
                apps::select_repository::view(state)
            }
            App::MainWindow(state) => apps::main_window::view2(
                state.main_window.as_ref().unwrap()
            ),
        }
    }
}


