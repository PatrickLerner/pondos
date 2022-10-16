#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LoadingState {
    Loading,
    Loaded,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Map,
    Travel,
    Settlement(SettlementState),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RunningState {
    Running,
    Paused,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SettlementState {
    Overview,
    Trade,
    Shipyard,
    Temple,
}
