use super::Exmo;
use anyhow::{Context, Result};
use trading_types::Symbol;

impl Exmo {
    // exmo:BTC/USD -> BTC_USD
    pub fn symbol_to_pair(s: &Symbol) -> String {
        format!("{}_{}", s.bs.to_uppercase(), s.qt.to_uppercase())
    }

    // BTC_USD -> Symbol("exmo:BTC/USD")
    pub fn pair_to_symbol(pair: &str) -> Result<Symbol> {
        let ctx = format!("pair_to_symbol: {pair}");
        let (b, q) = pair.split_once("_").with_context(|| ctx.clone())?;
        Ok(Symbol::new("exmo".into(), b.into(), q.into()))
    }
}
