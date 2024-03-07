pub const PANE_LEFT: usize = 0;
pub const PANE_RIGHT: usize = 1;

#[derive(Debug, Clone, Copy)]
pub struct CustomGridPane {
    pub id: usize,
    pub is_pinned: bool,
}

// TODO so far this is pretty much a placeholder
impl CustomGridPane {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}