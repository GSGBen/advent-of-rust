/// Input is one line with a list of inclusive numeric ranges, comma-separated.
/// Generate all the numbers within all the ranges, and add up the numbers that
/// are made up of exactly two matching strings. E.g. 123123, 22, 5555, but not
/// 55551, 51212.
pub fn puzzle01(input: Vec<String>) {
    let id_ranges = input.get(0).unwrap().split(",").map(|range_str| {
        let mut split = range_str.splitn(2, "-");
        let start: u64 = split.next().unwrap().parse().unwrap();
        let end: u64 = split.next().unwrap().parse().unwrap();
        start..=end
    });
    let mut ids: Vec<u64> = Vec::new();
    for range in id_ranges {
        for id in range {
            ids.push(id);
        }
    }

    let mut total = 0;
    for id in ids {
        let id_str = id.to_string();
        let (left, right) = id_str.split_at(id_str.len() / 2);
        if left == right {
            total += id;
        }
    }
    println!("{}", total);
}
