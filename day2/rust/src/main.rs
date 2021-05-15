use std::{fs::read_to_string, str::FromStr};

use anyhow::Result;
#[derive(Debug, PartialEq, Eq)]
struct Rule {
    min_chars: usize,
    max_chars: usize,
    char: char,
    password: String,
}

impl Rule {
    fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.char).count();
        count >= self.min_chars && count <= self.max_chars
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        fn get_parts(line: &str) -> Option<(&str, &str, &str, &str)> {
            // Parses the input string:
            // "1-3 a: password"
            // into these parts:
            // ("1", "3", "a", "password")
            let minus = line.find('-')?;
            let space = minus + line[minus..].find(' ')?;
            let colon = space + line[space..].find(": ")?;
            Some((
                &line[..minus],
                &line[(minus + 1)..space],
                &line[(space + 1)..colon],
                &line[(colon + 2)..],
            ))
        }
        let (min, max, chr, password) =
            get_parts(line).expect(&format!("Unable to parse line: {}", line));
        Ok(Rule {
            min_chars: min.parse()?,
            max_chars: max.parse()?,
            char: chr
                .chars()
                .next()
                .ok_or_else(|| anyhow::anyhow!("char part of rule was empty"))?,
            password: password.to_string(),
        })
    }
}

fn main() -> Result<()> {
    let data = read_to_string("../input.txt")?;
    let rules = data
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Rule>>>()?;
    let count = rules.iter().filter(|r| r.is_valid()).count();
    println!("Day 2 Part 2: {}", count);
    Ok(())
}

#[cfg(test)]
mod tests {

    use anyhow::Result;

    use crate::Rule;

    fn get_expected() -> Vec<Rule> {
        vec![
            Rule {
                min_chars: 1,
                max_chars: 3,
                char: 'a',
                password: "abcde".into(),
            },
            Rule {
                min_chars: 1,
                max_chars: 3,
                char: 'b',
                password: "cdefg".into(),
            },
            Rule {
                min_chars: 2,
                max_chars: 9,
                char: 'c',
                password: "ccccccccc".into(),
            },
        ]
    }

    #[test]
    fn test_parse() -> Result<()> {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let got: Result<Vec<Rule>> = input.lines().map(|l| l.parse()).collect();
        let got = got?;
        let expected = get_expected();
        assert_eq!(got, expected);
        Ok(())
    }

    #[test]
    fn test_rule_valid() {
        let rules: Vec<bool> = get_expected().iter().map(Rule::is_valid).collect();
        assert_eq!(rules, vec![true, false, true]);
    }
}
