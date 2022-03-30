// https://leetcode-cn.com/problems/rectangle-overlap/

pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    !(rec1[2] <= rec2[0] ||   // left
        rec1[3] <= rec2[1] ||   // bottom
        rec1[0] >= rec2[2] ||   // right
        rec1[1] >= rec2[3]) // top
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p836() {
        assert_eq!(
            true,
            is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3])
        );
        assert_eq!(
            false,
            is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 0, 2, 1])
        );
        assert_eq!(
            true,
            is_rectangle_overlap(vec![7, 8, 13, 15], vec![10, 8, 12, 20])
        );
    }
}
