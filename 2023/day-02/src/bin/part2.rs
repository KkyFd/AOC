use day_02::Colors;
use std::cmp::Ordering;
fn main() {}
#[allow(dead_code)]
fn parse(input: &str) -> i32 {
    let mut result = 0;

    for line in input.lines() {
        //this sucks
        let last_chars = ['n', 'e', 'd'];
        let mut cubes = Colors { r: 0, g: 0, b: 0 };
        for chunk in line
            .split_whitespace()
            .skip(2)
            .collect::<Vec<&str>>()
            .chunks(2)
        {
            let parsed_cubes = chunk[0].parse::<u32>().unwrap_or(0);
            let color_initial = chunk[1].chars().next().unwrap();

            match color_initial {
                'r' => {
                    if cubes.r.cmp(&parsed_cubes) == Ordering::Less {
                        cubes.r = parsed_cubes;
                    }
                }
                'g' => {
                    if cubes.g.cmp(&parsed_cubes) == Ordering::Less {
                        cubes.g = parsed_cubes;
                    }
                }
                'b' => {
                    if cubes.b.cmp(&parsed_cubes) == Ordering::Less {
                        cubes.b = parsed_cubes;
                    }
                }
                _ => println!("what?"),
            }
            if last_chars
                .iter()
                .any(|c| *c == chunk[1].chars().next_back().unwrap())
            {
                result += cubes.product()
            }
        }
    }

    result as i32
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn yeah() {
        let test = parse(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        assert_eq!(test, 2286);
    }
}
