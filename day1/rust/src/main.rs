use std::fs::read_to_string;

use anyhow::Result;
use itertools::Itertools;

fn day1(input: &str) -> Result<Option<(usize, usize)>> {
    let vals: std::result::Result<Vec<usize>, _> =
        input.lines().map(|l| l.parse::<usize>()).collect();
    let vals = vals?;
    Ok(vals
        .iter()
        .cartesian_product(vals.iter())
        .find(|(&a, &b)| a + b == 2020)
        .map(|(&a, &b)| (a, b)))
}

fn main() -> Result<()> {
    let data = read_to_string("../input.txt")?;
    let answer = day1(&data)?;
    if let Some((a, b)) = answer {
        println!(
            "Day1 answer: {} + {} = 2020 && {} * {} = {}",
            a,
            b,
            a,
            b,
            a * b
        )
    } else {
        println!("Day1 NO ANSWER FOUND!")
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::day1;

    #[test]
    fn test_day1() -> Result<()> {
        let input = "1721
979
366
299
675
1456";
        let answer = day1(input)?;
        assert_eq!(answer, Some((1721, 299)));
        Ok(())
    }
}
