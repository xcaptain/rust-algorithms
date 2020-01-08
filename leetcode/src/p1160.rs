// https://leetcode-cn.com/problems/find-words-that-can-be-formed-by-characters/

pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let mut char_arr = chars.chars().collect::<Vec<char>>();
    char_arr.sort();
    let cl = char_arr.len();
    // let sorted_char = char_arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");

    let mut res = 0;
    for word in words {
        let mut word_arr = word.chars().collect::<Vec<char>>();
        word_arr.sort();
        // let sorted_word = word_arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
        // println!("sorted_word: {}", sorted_word);
        let wdl = word_arr.len();
        if wdl > cl {
            continue;
        }
        let mut i = 0;
        let mut j = 0;
        while i < wdl && j < cl {
            if word_arr[i] == char_arr[j] {
                i += 1;
                j += 1;
            } else {
                j += 1;
            }
        }
        // println!("i, j: {}, {}", i, j);
        if i == wdl {
            res += wdl;
        }
    }

    // println!("{:?}", char_arr);
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1160() {
        assert_eq!(
            6,
            count_characters(
                vec![
                    String::from("cat"),
                    String::from("bt"),
                    String::from("hat"),
                    String::from("tree"),
                ],
                String::from("atach"),
            )
        );

        assert_eq!(
            10,
            count_characters(
                vec![
                    String::from("hello"),
                    String::from("world"),
                    String::from("leetcode")
                ],
                String::from("welldonehoneyr"),
            )
        );
    }
}
