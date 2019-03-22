pub fn is_leap_year(year: usize) -> bool {
    if year % 4 == 0 && year % 100 != 0 {
        return true;
    }
    if year % 400 == 0 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        assert_eq!(true, is_leap_year(2004));
    }
}
