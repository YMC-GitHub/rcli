pub fn zero() -> String {
    "[zero] Hello, world!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = zero();
        assert_eq!(result.contains("zero"), true);
    }
}
