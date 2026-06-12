use trading_types::{Depth, OrderPlaced};

/// Represents parsed data of one ws message
#[derive(Debug)]
pub enum WsMarketData {
    DepthSnapshot(Depth),
    DepthDiff(Depth),
    OrderPlacedId(String),
    OrdersMyDiff(Vec<OrderPlaced>),
    OrdersMySnapshot(Vec<OrderPlaced>),
    Unknown(String),
}
