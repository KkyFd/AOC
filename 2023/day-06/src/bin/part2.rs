#[allow(dead_code)]
fn margin_of_error(input: &str) -> u64 {
    let mut line = input.lines();
    let time = line
        .next()
        .unwrap()
        .split_whitespace()
        .filter(|substr| substr.parse::<u64>().is_ok())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = line
        .next()
        .unwrap()
        .split_whitespace()
        .filter(|n| n.parse::<u64>().is_ok())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let mut num_of_ways = 0;
    (1..=time).for_each(|time_charging| {
        let distance_travelled = (time - time_charging) * time_charging;
        if distance_travelled > distance {
            num_of_ways += 1;
        }
    });
    num_of_ways
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn yeah() {
        let test = margin_of_error(
            "Time:      71530
            Distance:  940200",
        );
        assert_eq!(test, 71503);
    }
}
