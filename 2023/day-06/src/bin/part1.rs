#[allow(dead_code)]
fn margin_of_error(input: &str) -> i32 {
    let mut line = input.lines();
    let mut result = 1;
    for (time, distance) in line
        .clone()
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|substr| substr.parse::<u32>().ok())
        .zip(
            line.nth(1)
                .unwrap()
                .split_whitespace()
                .filter_map(|n| n.parse::<u32>().ok()),
        )
    {
        let mut num_of_ways = 0;
        (1..=time).for_each(|time_charging| {
            let distance_travelled = (time - time_charging) * time_charging;
            if distance_travelled > distance {
                num_of_ways += 1;
            }
        });
        result *= num_of_ways;
    }
    result
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn yeah() {
        let test = margin_of_error(
            "Time:      7  15   30
        Distance:  9  40  200",
        );
        assert_eq!(test, 288);
    }
}
