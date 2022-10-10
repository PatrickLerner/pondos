#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Loading,
    Map,
    Travel,
    Settlement(SettlementState),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SettlementState {
    Overview,
    Trade,
    Shipyard,
    Temple,
}
