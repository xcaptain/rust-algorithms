use std::cmp::Ordering;

pub fn third_largest_elem(arr: Vec<i32>) -> i32 {
    let mut first = i32::min_value();
    for num in arr.iter() {
        if *num > first {
            first = *num;
        }
    }
    let mut second = i32::min_value();
    for num in arr.iter() {
        if *num > second && *num < first {
            second = *num;
        }
    }

    let mut third = i32::min_value();
    for num in arr.iter() {
        if *num > third && *num < second {
            third = *num;
        }
    }

    third
}

pub fn third_largest_elem_v2(arr: Vec<i32>) -> i32 {
    let mut first = i32::min_value();
    let mut second = i32::min_value();
    let mut third = i32::min_value();

    for num in arr {
        match num.cmp(&first) {
            Ordering::Greater => {
                third = second;
                second = first;
                first = num;
            }
            Ordering::Less => {
                if num > second {
                    third = second;
                    second = num;
                } else if num < second && num > third {
                    third = num;
                }
            }
            _ => {}
        }
    }
    third
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_third_largest_elem() {
        assert_eq!(1, third_largest_elem(vec![1, -1, 1, 0, 3, 2]));
        assert_eq!(1, third_largest_elem(vec![1, -1, 2, 0, 3, 2]));

        assert_eq!(1, third_largest_elem_v2(vec![1, -1, 1, 0, 3, 2]));
        assert_eq!(1, third_largest_elem_v2(vec![1, -1, 2, 0, 3, 2]));
    }
}
