pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() == 0 {
        return 0;
    }
    let mut max_profile = 0;
    let mut index = 1;
    while index < prices.len() - 1 {
        if prices[index] > prices[index - 1] {
            max_profile += prices[index] - prices[index - 1];
        }

        index += 1;
    }
    return max_profile;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
