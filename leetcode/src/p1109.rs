// https://leetcode-cn.com/problems/corporate-flight-bookings/

pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let mut res = vec![0; n as usize];
    for booking in &bookings {
        for i in booking[0] - 1..booking[1] {
            res[i as usize] += booking[2];
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1109() {
        assert_eq!(
            vec![10, 55, 45, 25, 25],
            corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5),
        );
    }
}
