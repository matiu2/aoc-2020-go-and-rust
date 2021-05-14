use std::{collections::HashSet, fs::read_to_string};

use anyhow::Result;

fn day1(input: &str) -> Result<Option<(usize, usize)>> {
    let vals: std::result::Result<HashSet<usize>, _> =
        input.lines().map(|l| l.parse::<usize>()).collect();
    let vals = vals?;
    Ok(vals
        .iter()
        .flat_map(|&a| {
            let b = 2020 - a;
            if vals.contains(&b) {
                return Some((a, b));
            } else {
                return None;
            }
        })
        .next())
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
