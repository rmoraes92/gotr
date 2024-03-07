use iced;

#[derive(Debug, Clone)]
pub enum PaneOrientation {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct MyPane {
    pub id: usize,
    pub orientation: PaneOrientation,
}

impl MyPane {
    pub fn new_left() -> Self {
        Self {
            id: 0,
            orientation: PaneOrientation::Left,
        }
    }
    pub fn new_right() -> Self {
        Self {
            id: 1,
            orientation: PaneOrientation::Right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MainWindowState {
    // TODO do we want to "load" this right at the initial boot?
    // pub panegrid_state: iced::widget::pane_grid::State<MyPane>
    pub counter: usize,
    pub repository_path: Option<String>,
    pub panegrid_state: iced::widget::pane_grid::State<MyPane>,
}

impl MainWindowState {
    pub fn new<S: Into<String>>(repository_path: S) -> Self {
        Self {
            repository_path: Some(repository_path.into()),
            ..Self::default()
        }
    }
}

impl Default for MainWindowState {
    fn default() -> Self {
        let (mut panegrid_state, _left_pane) = iced::widget::pane_grid::State::new(MyPane::new_left());
        let axis = iced::widget::pane_grid::Axis::Vertical;
        panegrid_state.split(axis, _left_pane, MyPane::new_right());
        Self { counter: 0, repository_path: None, panegrid_state: panegrid_state }
    }
}
