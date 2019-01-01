use algorithms::misc::leap_year::is_leap_year;
use std::cmp::Ordering;
/// https://projecteuler.net/problem=19
/// 1 Jan 1900 was a Monday.
/// You are given the following information, but you may prefer to do some research for yourself.
/// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

pub struct SimpleDate {
    pub year: usize,
    pub month: usize,
    pub day: usize,
}

impl SimpleDate {
    pub fn new(year: usize, month: usize, day: usize) -> Self {
        SimpleDate {
            year: year,
            month: month,
            day: day,
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}-{}-{}", self.year, self.month, self.day);
    }

    /// get the day in week, Sunday -> 0, Monday -> 1 ... Sat -> 6
    pub fn weekday(&self) -> usize {
        let mut start_date = SimpleDate::new(1900, 1, 1);
        let mut day_diff = 1; // this day is monday
        loop {
            if start_date >= *self {
                break;
            }
            day_diff += 1;
            start_date = start_date.next_day();
        }
        let day_in_week = day_diff % 7; // sunday is 0
        return day_in_week;
    }

    /// add a day to self
    pub fn next_day(&mut self) -> SimpleDate {
        if self.month == 12 {
            if self.day == 31 {
                // last day of the year, to a new year
                return SimpleDate::new(self.year + 1, 1, 1);
            } else {
                // just add one day
                return SimpleDate::new(self.year, self.month, self.day + 1);
            }
        } else if [4, 6, 9, 11].contains(&self.month) {
            // 30-day months
            if self.day == 30 {
                return SimpleDate::new(self.year, self.month + 1, 1);
            } else {
                return SimpleDate::new(self.year, self.month, self.day + 1);
            }
        } else if [1, 3, 5, 7, 8, 10].contains(&self.month) {
            if self.day == 31 {
                return SimpleDate::new(self.year, self.month + 1, 1);
            } else {
                return SimpleDate::new(self.year, self.month, self.day + 1);
            }
        } else if self.month == 2 {
            if is_leap_year(self.year) {
                if self.day == 29 {
                    return SimpleDate::new(self.year, self.month + 1, 1);
                } else {
                    return SimpleDate::new(self.year, self.month, self.day + 1);
                }
            } else {
                if self.day == 28 {
                    return SimpleDate::new(self.year, self.month + 1, 1);
                } else {
                    return SimpleDate::new(self.year, self.month, self.day + 1);
                }
            }
        } else {
            panic!("error here, invalid month");
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
    let mut weekday = start_date.weekday();
    let mut num = 0;
    while start_date < *end_date {
        if weekday == 0 && start_date.day == 1 {
            num += 1;
        }
        start_date = start_date.next_day();
        weekday += 1;
        weekday = weekday % 7;
    }
    return num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_day() {
        let mut start_date = SimpleDate::new(1900, 1, 1);
        start_date = start_date.next_day();
        assert_eq!(true, SimpleDate::new(1900, 1, 2) == start_date);
        assert_eq!(true, start_date < SimpleDate::new(1900, 1, 3));
    }

    #[test]
    fn test_weekday() {
        let mut start_date = SimpleDate::new(1900, 1, 1);
        start_date = start_date.next_day();
        start_date = start_date.next_day();

        assert_eq!(3, start_date.weekday());
    }

    #[test]
    fn test_count_days() {
        assert_eq!(
            2,
            count_sundays(&SimpleDate::new(1900, 1, 1), &SimpleDate::new(1900, 12, 31))
        ); // 1900-4-1, 1900-7-1
        assert_eq!(
            171,
            count_sundays(&SimpleDate::new(1901, 1, 1), &SimpleDate::new(2000, 12, 31))
        );
    }
}
