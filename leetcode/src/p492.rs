pub fn construct_rectangle(area: i32) -> Vec<i32> {
    let mut w = (area as f64).sqrt() as i32;
    let mut min_dist = area;
    let mut exp_l = 0;

    while w >= 1 {
        if area % w == 0 {
            let l = area / w;
            let dist = l - w;
            if dist < min_dist {
                min_dist = dist;
                exp_l = l;
            }
        }
        w -= 1;
    }

    vec![exp_l, area / exp_l]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p492() {
        assert_eq!(vec![1, 1], construct_rectangle(1));
        assert_eq!(vec![2, 2], construct_rectangle(4));
        assert_eq!(vec![5, 1], construct_rectangle(5));
    }
}
