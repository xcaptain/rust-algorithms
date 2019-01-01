/// https://projecteuler.net/problem=17
/// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
/// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

/// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen)
/// contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.
use std::collections::HashMap;

/// get the sum of all words less or equal than num
pub fn get_all_words_len(num: usize) -> usize {
    let mut sum = 0;
    for i in 1..=num {
        sum += get_single_word_len(i);
    }
    sum
}

/// get the length of characters in a single number
pub fn get_single_word_len(num: usize) -> usize {
    let word = num_to_word(num);
    word.len()
}

fn num_to_word(num: usize) -> String {
    let m: HashMap<String, usize> = [
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
        ("ten".to_string(), 10),
        ("eleven".to_string(), 11),
        ("twelve".to_string(), 12),
        ("thirteen".to_string(), 13),
        ("fourteen".to_string(), 14),
        ("fifteen".to_string(), 15),
        ("sixteen".to_string(), 16),
        ("seventeen".to_string(), 17),
        ("eighteen".to_string(), 18),
        ("nineteen".to_string(), 19),
        ("twenty".to_string(), 20),
        ("thirty".to_string(), 30),
        ("forty".to_string(), 40),
        ("fifty".to_string(), 50),
        ("sixty".to_string(), 60),
        ("seventy".to_string(), 70),
        ("eighty".to_string(), 80),
        ("ninety".to_string(), 90),
    ]
    .iter()
    .cloned()
    .collect();
    let word: String;
    if num >= 1 && num < 100 {
        word = get_word_lt100(&m, num);
    } else if num >= 100 && num < 1000 {
        word = get_word_gt100lt1000(&m, num);
    } else if num >= 1000 {
        word = get_word_gt1000(&m, num);
    } else {
        panic!("not implemented!");
    }
    word
}

fn get_word_gt1000(m: &HashMap<String, usize>, num: usize) -> String {
    let high = num / 1000;
    let mut high_word = direct_get_from_table(m, high);
    high_word = format!("{}thousand", high_word);

    let low = num - high * 1000;
    if low == 0 {
        return high_word;
    }
    let low_word = get_word_gt100lt1000(m, low);
    return format!("{}and{}", high_word, low_word);
}

/// get word from a num which is less then 100(<100)
fn get_word_lt100(m: &HashMap<String, usize>, num: usize) -> String {
    let mut word = direct_get_from_table(&m, num);
    if word.is_empty() {
        // not found in the table, do simple addition
        let low = num % 10;
        let high = num - low;
        let high_word = direct_get_from_table(&m, high);
        let low_word = direct_get_from_table(&m, low);
        word = format!("{}{}", high_word, low_word);
    }
    word
}

fn get_word_gt100lt1000(m: &HashMap<String, usize>, num: usize) -> String {
    let high = num / 100;
    let mut high_word = direct_get_from_table(m, high);
    high_word = format!("{}hundred", high_word);

    let low = num - high * 100;
    if low == 0 {
        return high_word;
    }
    let low_word = get_word_lt100(m, low);
    return format!("{}and{}", high_word, low_word);
}

fn direct_get_from_table(m: &HashMap<String, usize>, num: usize) -> String {
    for (key, val) in m.iter() {
        if *val == num {
            return key.to_owned();
        }
    }
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1000() {
        assert_eq!(21124, get_all_words_len(1000));
    }

    #[test]
    fn test_single_word_len() {
        assert_eq!(23, get_single_word_len(342));
        assert_eq!(20, get_single_word_len(115));
    }
}
