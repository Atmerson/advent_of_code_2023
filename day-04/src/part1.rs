use std::collections::HashSet;


use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    
    let mut sum = 0;
    let lines:Vec<String> = input.lines().map(|s| s.to_string()).collect();
    for line in lines.iter() {
        // split the line
        let mut parts = line.split(" | ");
        // get winning numbers
        let winning_numbers_vec = parts.next().unwrap().split(": ").nth(1).unwrap()
            .split(" ")
            .filter(|&n| !n.trim().is_empty())
            .collect::<Vec<&str>>();
        let winning_numbers_set: HashSet<&str> = winning_numbers_vec.iter().cloned().collect();
        // count numbers after | that are in set
        // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        let checked_numbers_vec = parts.next().unwrap().split(" ")
            .filter(|&n| !n.trim().is_empty())
            .collect::<Vec<&str>>();
        let intersect :Vec<&str>= checked_numbers_vec.iter()
            .filter(|&n| winning_numbers_set.contains(n)).cloned().collect();
        let count = intersect.len();
        if count > 0 {
            let add = 1 << (count - 1);
            sum += add;
        }
    }
    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
