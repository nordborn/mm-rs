use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct SingleBotState {
    pub iter: u64,
    pub must_stop: bool,
}
