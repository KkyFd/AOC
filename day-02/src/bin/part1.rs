use day_02::Colors;
fn main() {}
#[allow(dead_code)]
fn parse(input: &str) -> i32 {
    input.lines().enumerate().fold(0, |acc, (i, line)| {
        //i'm lazy and dum dum

        let mut cubes = Colors { r: 0, g: 0, b: 0 };
        for chunk in line
            .split_whitespace()
            .skip(2)
            .collect::<Vec<&str>>()
            .chunks(2)
        {
            match chunk[1].chars().next().unwrap() {
                'r' => cubes.r += chunk[0].parse::<u32>().unwrap(),
                'g' => cubes.g += chunk[0].parse::<u32>().unwrap(),
                'b' => cubes.b += chunk[0].parse::<u32>().unwrap(),
                _ => println!("what?"),
            }
            match chunk[1].chars().next_back().unwrap() {
                //checks for each game set
                ';' => match cubes.check_overflow() {
                    true => continue,
                    false => cubes = cubes.reset(),
                },
                //checks if game round has ended
                'n' | 'e' | 'd' => match cubes.check_overflow() {
                    true => continue,
                    false => return acc + (i + 1) as i32,
                },
                _ => {}
            }
        }
        acc
    })
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

        assert_eq!(test, 8);
    }
}
