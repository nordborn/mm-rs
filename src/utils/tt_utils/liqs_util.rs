use itertools::Itertools;
use std::collections::HashMap;

use trading_types::Liq;

// merge_liqs merges liqs supposing that they represents side of Depth
pub fn merge_side_liqs(ll1: &[Liq], ll2: &[Liq], is_price_asc: bool, take: usize) -> Vec<Liq> {
    let mut lm1 = HashMap::new();
    for l1 in ll1 {
        lm1.insert(l1.price(), *l1);
    }
    for l2 in ll2 {
        lm1.entry(l2.price())
            .and_modify(|v| *v = Liq::from((v.price(), v.amount() + l2.amount())))
            .or_insert(*l2);
    }
    let mut ret = Vec::with_capacity(lm1.len());
    for k in lm1.keys().sorted_by(|x, y| {
        if is_price_asc {
            x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
        } else {
            y.partial_cmp(x).unwrap_or(std::cmp::Ordering::Equal)
        }
    }) {
        if lm1[k].amount().0 > 0.0 {
            ret.push(lm1[k]);
        }
    }
    ret.into_iter().take(take).collect()
}
