use git2::Oid;

// use anyhow::Result as AnyResult;
use crate::states::app::State;


#[derive(Debug, Clone)]
pub enum Message {
    StateLoaded(State),
    RepositorySelected(String),
    CommitSelected(Oid),
    ShowHEADSummary,
    StageFile(String),
    UnstageFile(String),
}
