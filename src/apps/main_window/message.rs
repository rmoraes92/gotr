use git2;
use crate::globals;

#[derive(Debug, Clone)]
pub enum Message {
    // PaneGrid(globals::PaneGridEvent),
    IncrementCounter,
    CommitSelected(git2::Oid),
    PaneGridEvent(globals::PaneGridEvent),
}