#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Loading,
    Map,
    Settlement,
    TravelToSettlement,
    TradeWithSettlement,
    Shipyard,
}
