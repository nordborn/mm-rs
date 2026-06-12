use super::Okx;
use anyhow::{Context, Result};
use trading_types::Symbol;

impl Okx {
    // okx:BTC/USD -> BTC-USD
    pub fn symbol_to_pair(s: &Symbol) -> String {
        format!("{}-{}", s.bs.to_uppercase(), s.qt.to_uppercase())
    }

    // BTC-USD -> Symbol("okx:BTC/USD")
    pub fn pair_to_symbol(pair: &str) -> Result<Symbol> {
        let ctx = format!("pair_to_symbol: {pair}");
        let (b, q) = pair.split_once("-").with_context(|| ctx.clone())?;
        Ok(Symbol::new("okx".into(), b.into(), q.into()))
    }
}
