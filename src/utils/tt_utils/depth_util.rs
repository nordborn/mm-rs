use trading_types::{Amount, Liq, Price, Worth};

// Merges liqs(orders) by price (L2)
pub fn liqs_l2(liqs: &[Liq]) -> Vec<Liq> {
    let mut l2: Vec<Liq> = Vec::with_capacity(liqs.len());
    let mut prev_opt: Option<Liq> = None;
    for &l in liqs {
        match prev_opt {
            None => prev_opt = Some(l),
            Some(prev) => {
                if prev.price() == l.price() {
                    prev_opt = Some(Liq::from((prev.price(), prev.amount() + l.amount())));
                } else {
                    l2.push(prev);
                    prev_opt = Some(l);
                }
            }
        }
    }
    if let Some(prev) = prev_opt {
        l2.push(prev);
    }
    l2
}

pub fn drop_worth(ll: &[Liq], ll_must_drop: &[Liq], drop_worth: Worth) -> Vec<Liq> {
    // unique prices
    let ll_must_drop_l2 = liqs_l2(ll_must_drop);
    let ll_wo_dropped: Vec<Liq> = liqs_l2(ll)
        .iter()
        .filter_map(|&l| {
            let mut l = l;
            for &l_to_drop in &ll_must_drop_l2 {
                if l.price() == l_to_drop.price() && l.amount() >= l_to_drop.amount() {
                    l = Liq::from((l.price(), l.amount() - l_to_drop.amount()));
                }
            }
            if l.amount() > Amount(0.0) { Some(l) } else { None }
        })
        .collect();

    // will use len of dropped to skip then
    let dropped: Vec<_> = ll_wo_dropped
        .iter()
        .scan(drop_worth, |w_drop_remains: &mut Worth, l: &Liq| {
            if *w_drop_remains >= l.worth() {
                *w_drop_remains -= l.worth();
                Some(true)
            } else {
                None
            }
        })
        .collect();

    ll_wo_dropped.into_iter().skip(dropped.len()).collect::<Vec<_>>()
}

pub fn worst_execution_price(ll: &[Liq], w: Worth) -> Option<Price> {
    // will use len of dropped to skip then
    let dropped: Vec<_> = ll
        .iter()
        .scan(w, |w_remains: &mut Worth, l: &Liq| {
            if *w_remains > l.worth() {
                *w_remains -= l.worth();
                Some(true)
            } else {
                None
            }
        })
        .collect();

    if dropped.len() == ll.len() {
        return None;
    }

    ll.iter()
        .skip(dropped.len())
        .collect::<Vec<_>>()
        .first()
        .map(|x| x.price())
}

#[cfg(test)]
mod test {
    use super::*;

    fn ll() -> Vec<Liq> {
        vec![
            Liq::from((Price(1.0), Amount(10.0))), // W(10)
            Liq::from((Price(2.0), Amount(10.0))), // W(20)
        ]
    }

    #[test]
    pub fn test_worst_execution_price_0() {
        let w = Worth(5.0);
        assert_eq!(worst_execution_price(&ll(), w), Some(Price(1.0)));
    }

    #[test]
    pub fn test_worst_execution_price_1() {
        let w = Worth(10.0);
        assert_eq!(worst_execution_price(&ll(), w), Some(Price(1.0)));
    }

    #[test]
    pub fn test_worst_execution_price_2() {
        let w = Worth(20.0);
        assert_eq!(worst_execution_price(&ll(), w), Some(Price(2.0)));
    }

    #[test]
    pub fn test_worst_execution_price_3() {
        let w = Worth(30.0);
        assert_eq!(worst_execution_price(&ll(), w), Some(Price(2.0)));
    }

    #[test]
    pub fn test_worst_execution_price_4() {
        let w = Worth(50.0);
        assert_eq!(worst_execution_price(&ll(), w), None);
    }
}
