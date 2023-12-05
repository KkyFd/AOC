#[allow(dead_code)]
fn calibration_value(input: &str) -> i32 {
    input
        .lines()
        .scan(0, |state, line| {
            if line.is_empty() {
                return None;
            }
            let (first, last) = (
                line.find(|c: char| c.is_ascii_digit()),
                line.rfind(|c: char| c.is_ascii_digit()),
            );

            let (first_num, last_num) = match (first, last) {
                (Some(fst), Some(lst)) => (fst, lst),
                (Some(fst), None) => (fst, fst),
                _ => return None,
            };

            *state = format!(
                "{}{}",
                line.to_string().as_bytes()[first_num] as char,
                line.to_string().as_bytes()[last_num] as char
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
            "1abc2
        pqr3stu8vwxcargo format
        a1b2c3d4e5f
        treb7uchet",
        );

        assert_eq!(test, 142);
    }
}
