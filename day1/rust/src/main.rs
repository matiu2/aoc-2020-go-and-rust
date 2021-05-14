use std::fs::read_to_string;

use anyhow::Result;
use itertools::Itertools;

fn day1_part1(input: &str) -> Result<Option<(usize, usize)>> {
    let vals: std::result::Result<Vec<usize>, _> =
        input.lines().map(|l| l.parse::<usize>()).collect();
    let vals = vals?;
    Ok(vals
        .iter()
        .cartesian_product(vals.iter())
        .find(|(&a, &b)| a + b == 2020)
        .map(|(&a, &b)| (a, b)))
}

fn day1_part2(input: &str) -> Result<Option<(usize, usize, usize)>> {
    let vals: std::result::Result<Vec<usize>, _> =
        input.lines().map(|l| l.parse::<usize>()).collect();
    let vals = vals?;
    Ok(vals
        .iter()
        .cartesian_product(vals.iter())
        .cartesian_product(vals.iter())
        .find(|((&a, &b), &c)| a + b + c == 2020)
        .map(|((&a, &b), &c)| (a, b, c)))
}

fn main() -> Result<()> {
    let data = read_to_string("../input.txt")?;
    let answer = day1_part1(&data)?;
    if let Some((a, b)) = answer {
        println!(
            "Day1-part1 answer: {} + {} = 2020 && {} * {} = {}",
            a,
            b,
            a,
            b,
            a * b
        )
    } else {
        println!("Day1-part1 NO ANSWER FOUND!")
    }

    // Part 2
    let answer = day1_part2(&data)?;
    if let Some((a, b, c)) = answer {
        println!(
            "Day1-part2 answer: {} + {} + {} = 2020 && {} * {} * {} = {}",
            a,
            b,
            c,
            a,
            b,
            c,
            a * b * c
        )
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{day1_part1, day1_part2};

    #[test]
    fn test_day1_part1() -> Result<()> {
        let input = "1721
979
366
299
675
1456";
        let answer = day1_part1(input)?;
        assert_eq!(answer, Some((1721, 299)));
        Ok(())
    }

    #[test]
    fn test_day1_part2() -> Result<()> {
        let input = "1721
979
366
299
675
1456";
        let answer = day1_part2(input)?;
        assert_eq!(answer, Some((979, 366, 675)));
        Ok(())
    }
}
