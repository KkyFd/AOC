#[allow(dead_code)]
fn calibration_value(input: &str) -> i32 {
    input
        .lines()
        .scan(0, |state, line| {
            if line.is_empty() {
                return None;
            }

            let written_numbers = [
                ("one", "o1e"),
                ("two", "t2o"),
                ("three", "t3e"),
                ("four", "f4r"),
                ("five", "f5e"),
                ("six", "s6x"),
                ("seven", "s7n"),
                ("eight", "e8t"),
                ("nine", "n9e"),
            ];
            let mut parsed = line.trim().to_owned();
            for (element, replacement) in written_numbers.iter() {
                if parsed.contains(element) {
                    parsed = parsed.replace(element, replacement);
                }
            }
            let (firstnumber, lastnumber) = (
                parsed.find(|c: char| c.is_ascii_digit()),
                parsed.rfind(|c: char| c.is_ascii_digit()),
            );

            let (first_num, last_num) = match (firstnumber, lastnumber) {
                (Some(fst), Some(lst)) => (fst, lst),
                (Some(fst), None) => (fst, fst),
                _ => return None,
            };

            *state = format!(
                "{}{}",
                parsed.to_string().as_bytes()[first_num] as char,
                parsed.to_string().as_bytes()[last_num] as char
            )
            .parse::<i32>()
            .unwrap();

            Some(*state)
        })
        .sum::<i32>()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yeah() {
        let test = calibration_value(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );

        assert_eq!(test, 281);
    }
}
