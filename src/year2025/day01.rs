const DIAL_MAX: i32 = 99;

///  There's a dial from 0 to 99 that wraps. it starts pointing at 50. L means
///  move lower, R means move higher. Calculate the number of times it lands on 0.
///
///  Example input:
///  - R9
///  - L40
///  - R6
///  - L6
///  - R26
///  - L44
pub fn puzzle01(input: Vec<String>) {
    let mut pointing_at: i32 = 50;
    let mut num_times_at_zero: u32 = 0;
    for line in input {
        match line.split_at(1) {
            ("R", amount) => {
                let amount: i32 = amount.parse().expect("couldn't parse amount");
                pointing_at += amount;
                pointing_at = pointing_at.rem_euclid(DIAL_MAX + 1);
            }
            ("L", amount) => {
                let amount: i32 = amount.parse().expect("couldn't parse amount");
                pointing_at -= amount;
                pointing_at = pointing_at.rem_euclid(DIAL_MAX + 1);
            }
            (_, _) => panic!("line '{}' didn't match <R or L><num>", line),
        }
        if pointing_at == 0 {
            num_times_at_zero += 1;
        }
    }
    println!("num times at 0: {}", num_times_at_zero);
}

/// same as 01, but count the number of times pointing_at crosses zero, not just
/// lands on it.
pub fn puzzle02(input: Vec<String>) {
    let mut pointing_at: i32 = 50;
    let mut num_times_crossed_zero: u32 = 0;
    for line in input {
        let (direction, amount_text) = line.split_at(1);
        let amount: i32 = amount_text.parse().expect("couldn't parse amount");
        let direction_num: i32;
        if direction == "R" {
            direction_num = 1;
        } else if direction == "L" {
            direction_num = -1;
        } else {
            panic!("line '{}' didn't start with R or L", line)
        };

        // starting with the naive method - I wonder how slow brute-forcing it is
        for _ in 0..amount {
            pointing_at += direction_num;
            pointing_at = pointing_at.rem_euclid(DIAL_MAX + 1);
            if pointing_at == 0 {
                num_times_crossed_zero += 1
            }
        }
        // turns out it's not slow at all. Sticking with this
    }
    println!("num times crossed 0: {}", num_times_crossed_zero);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_wrap_example_04() {
        let result: i32 = 99 + 1;
        let result = result.rem_euclid(DIAL_MAX + 1);
        assert_eq!(result, 0);
    }

    #[test]
    fn add_wrap_example_05() {
        let result: i32 = 5 - 10;
        let result = result.rem_euclid(DIAL_MAX + 1);
        assert_eq!(result, 95);
    }

    #[test]
    fn add_wrap_example_06() {
        let result: i32 = 95 + 5;
        let result = result.rem_euclid(DIAL_MAX + 1);
        assert_eq!(result, 0);
    }
}
