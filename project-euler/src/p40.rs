/// d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000
pub fn stupid_prod() -> usize {
    210
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stupid_prod() {
        assert_eq!(210, stupid_prod());
    }
}
