pub fn my_pow(x: f64, n: i32) -> f64 {
    x.powi(n)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
