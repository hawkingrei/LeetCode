
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() == 0 {
        return 0;
    }
    let mut minprices = prices.get(0).unwrap();
    let mut profit = 0;
    for price in &prices {
        if minprices > &price {
            minprices = &price;
            continue;
        }
        if profit < price - minprices {
            profit = price - minprices;
        }
    }
    return profit;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
