// Returns whether a given year is a leap year
pub fn is_leap_year(year: i32) -> bool {
    let mut leap = false;
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                leap = true;
            } else {
                leap = false;
            }
        } else {
            leap = true;
        }
    } else {
        leap = false;
    }
    return leap;
}