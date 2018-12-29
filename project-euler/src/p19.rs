use std::cmp::Ordering;
use algorithms::misc::leap_year::is_leap_year;
/// https://projecteuler.net/problem=19
/// 1 Jan 1900 was a Monday.
/// You are given the following information, but you may prefer to do some research for yourself.
/// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
/// This program runs more than 2 mins on my laptop, hahaha

pub struct SimpleDate {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub weekdays: Vec<String>,
}

impl SimpleDate {
    pub fn new(year: usize, month: usize, day: usize) -> Self {
        SimpleDate {
            year: year,
            month: month,
            day: day,
            weekdays: vec![
                String::from("Mon"),
                String::from("Tues"),
                String::from("Wed"),
                String::from("Thu"),
                String::from("Fri"),
                String::from("Sat"),
                String::from("Sun"),
            ],
        }
    }

    pub fn weekday(&self) -> &String {
        let mut start_date = SimpleDate::new(1900, 1, 1); // this day is monday
        let mut day_diff = 0;
        loop {
            if start_date >= *self {
                break;
            }
            day_diff += 1;
            start_date.next_day();
        }
        let day_in_week = day_diff % 7;
        return self.weekdays.get(day_in_week).unwrap();
    }

    /// add a day to self
    pub fn next_day(&mut self) {
        if self.month == 12 {
            if self.day == 31 {
                // last day of the year, to a new year
                self.year += 1;
                self.month = 1;
                self.day = 1;
            } else {
                // just add one day
                self.day += 1;
            }
        } else if [4, 6, 9, 11].contains(&self.month) {
            // 30-day months
            if self.day == 30 {
                self.month += 1;
                self.day = 1;
            } else {
                self.day += 1;
            }
        } else if [1, 3, 5, 7, 8, 10].contains(&self.month) {
            if self.day == 31 {
                self.month += 1;
                self.day = 1;
            } else {
                self.day += 1;
            }
        } else if self.month == 2 {
            if is_leap_year(self.year) {
                if self.day == 29 {
                    self.day = 1;
                    self.month += 1;
                } else {
                    self.day += 1;
                }
            } else {
                if self.day == 28 {
                    self.day = 1;
                    self.month += 1;
                } else {
                    self.day += 1;
                }
            }
        }
    }
}

impl PartialEq for SimpleDate {
    fn eq(&self, other: &SimpleDate) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
}

impl PartialOrd for SimpleDate {
    fn partial_cmp(&self, other: &SimpleDate) -> Option<Ordering> {
        if self.year < other.year {
            return Some(Ordering::Less);
        } else if self.year == other.year {
            if self.month < other.month {
                return Some(Ordering::Less);
            } else if self.month == other.month {
                if self.day < other.day {
                    return Some(Ordering::Less);
                } else if self.day == other.day {
                    return Some(Ordering::Equal);
                }
                return Some(Ordering::Greater);
            }
            return Some(Ordering::Greater);
        }
        return Some(Ordering::Greater);
    }
}

pub fn count_sundays(s: &SimpleDate, end_date: &SimpleDate) -> usize {
    let mut start_date = SimpleDate::new(s.year, s.month, s.day);
    let mut num = 0;
    while start_date <= *end_date {
        if start_date.weekday() == &"Sun" && start_date.day == 1 {
            println!(
                "{}-{}-{}",
                start_date.year, start_date.month, start_date.day
            );
            num += 1;
        }
        start_date.next_day();
    }
    return num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_day() {
        let mut start_date = SimpleDate::new(1900, 1, 1);
        start_date.next_day();
        assert_eq!(true, SimpleDate::new(1900, 1, 2) == start_date);
        assert_eq!(true, start_date < SimpleDate::new(1900, 1, 3));
    }

    #[test]
    fn test_weekday() {
        let mut start_date = SimpleDate::new(1900, 1, 1);
        start_date.next_day();
        start_date.next_day();

        assert_eq!(&String::from("Wed"), start_date.weekday());
    }

    #[test]
    fn test_count_days() {
        assert_eq!(2, count_sundays(&SimpleDate::new(1900, 1, 1), &SimpleDate::new(1900, 12, 31))); // 1900-4-1, 1900-7-1
        assert_eq!(171, count_sundays(&SimpleDate::new(1900, 1, 1), &SimpleDate::new(2000, 12, 31)));
    }
}
