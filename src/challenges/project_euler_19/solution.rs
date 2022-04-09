use std::time::Instant;

/**
You are given the following information, but you may prefer to do some research for yourself.

1 Jan 1900 was a Monday.
Thirty days has September,
April, June and November.
All the rest have thirty-one,
Saving February alone,
Which has twenty-eight, rain or shine.
And on leap years, twenty-nine.
A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
 */
pub fn project_euler_19() {
    let start = Instant::now();
    let mut answer = 0;

    // January, March, May, July, August, October, December
    let thirty_one_day_months = [0, 2, 4, 6, 7, 9, 11];

    // April, June, September and November
    let thirty_day_months = [3, 5, 8, 10];

    // Number from 0 - 6, representing Mon - Sun
    let mut day_of_week = 0;

    // Number representing date of the month, 1st = 1
    let mut date = 1;

    // Number from 0 -11 representing the January - December
    let mut month = 0;

    let mut year = 1900;

    loop {
        // Increment date
        date +=1;

        // Month ending in 31
        if date > 31 && thirty_one_day_months.contains(&month) {
            // Happy New Year
            if month == 11 {
                year += 1;
            }

            date = 1;
            month = (month + 1) % 12;
        } else if date > 30 && thirty_day_months.contains(&month) {
            date = 1;
            month = (month + 1) % 12;
        } else if month == 1 && date > 28 {
            let is_leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);

            if !is_leap_year || (is_leap_year && date > 29) {
                date = 1;
                month = (month + 1) % 12;
            }
        }

        // Increment day of week
        day_of_week = (day_of_week + 1) % 7;

        if year == 2001 {
            break;
        }

        if year > 1900 && day_of_week == 6 && date == 1 {
            answer += 1;
        }
    }

    println!("Project Euler 19: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}