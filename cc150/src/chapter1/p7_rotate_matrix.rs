//! Given an image represented by an NxN matrix, where each pixel in the image is 4
//! bytes, write a method to rotate the image by 90 degrees. Can you do this in place?

/// 输入矩阵，返回右转90度之后的矩阵
pub fn rotate_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = mat.clone();
    let size = mat.len();

    for (i, row) in mat.iter().enumerate().take(size) {
        for (j, col) in res.iter_mut().enumerate().take(size) {
            col[size - i - 1] = row[j];
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix() {
        struct Args {
            mat: Vec<Vec<i32>>,
        }
        struct Test {
            name: String,
            args: Args,
            want: Vec<Vec<i32>>,
        }
        let tests = vec![
            Test {
                name: String::from("case1"),
                args: Args {
                    mat: vec![
                        vec![1, 1, 1, 1],
                        vec![2, 2, 2, 2],
                        vec![3, 3, 3, 3],
                        vec![4, 4, 4, 4],
                    ],
                },
                want: vec![
                    vec![4, 3, 2, 1],
                    vec![4, 3, 2, 1],
                    vec![4, 3, 2, 1],
                    vec![4, 3, 2, 1],
                ],
            },
            Test {
                name: String::from("case2"),
                args: Args {
                    mat: vec![
                        vec![1, 2, 3, 4],
                        vec![5, 6, 7, 8],
                        vec![9, 10, 11, 12],
                        vec![13, 14, 15, 16],
                    ],
                },
                want: vec![
                    vec![13, 9, 5, 1],
                    vec![14, 10, 6, 2],
                    vec![15, 11, 7, 3],
                    vec![16, 12, 8, 4],
                ],
            },
        ];
        for test in tests {
            assert_eq!(
                test.want,
                rotate_matrix(test.args.mat),
                "{} fails",
                test.name
            );
        }
    }
}
