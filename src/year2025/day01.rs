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
                pointing_at += amount.parse().unwrap_or(0);
                // handle negative modulus results
                pointing_at = pointing_at.rem_euclid(DIAL_MAX + 1);
            }
            ("L", amount) => {
                pointing_at -= amount.parse().unwrap_or(0);
                pointing_at = pointing_at.rem_euclid(DIAL_MAX + 1);
            }
            (_, _) => continue,
        }
        if pointing_at == 0 {
            num_times_at_zero += 1;
        }
    }
    println!("num times at 0: {}", num_times_at_zero);
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
