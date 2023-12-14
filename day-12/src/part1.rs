use crate::custom_error::AocError;
use std::collections::HashMap;
use std::fs;

fn calculate_solutions(characters: &Vec<char>, integers: &Vec<u128>, memoization: &mut HashMap<(Vec<u128>, Vec<char>), u128>) -> u128 {
    if characters.is_empty() {
        if integers.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    match characters[0] {
        '.' => calculate_solutions(&characters[1..].to_vec(), integers, memoization),
        '#' => calculate_hash_solutions(integers, characters, memoization),
        '?' => calculate_solutions(&characters[1..].to_vec(), integers, memoization) + calculate_hash_solutions(integers, characters, memoization),
        _ => panic!(">.> WHAT DID YOU DO?"),
    }
}

fn calculate_hash_solutions(integers: &Vec<u128>, characters: &Vec<char>, memoization: &mut HashMap<(Vec<u128>, Vec<char>), u128>) -> u128 {
    if let Some(&result) = memoization.get(&(integers.clone(), characters.clone())) {
        return result;
    }

    if integers.is_empty() {
        return 0;
    }

    let x = integers[0] as usize;
    if characters.len() < x {
        return 0;
    }
    for i in 0..x {
        if characters[i] == '.' {
            return 0;
        }
    }
    if characters.len() == x {
        if integers.len() == 1 {
            return 1;
        }
        return 0;
    }
    if characters[x] == '#' {
        return 0;
    }
    let result = calculate_solutions(&characters[(x + 1)..].to_vec(), &integers[1..].to_vec(), memoization);
    memoization.insert((integers.clone(), characters.clone()), result);
    result
}


#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let lines: Vec<&str> = input.lines().collect();
    let mut data_rows = Vec::new();
    let mut memoization = HashMap::new();

    for line in &lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let springs = parts[0].chars().collect();
        let groups: Vec<u128> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();
        data_rows.push((springs, groups));
    }

    let total: u128 = data_rows.iter().map(|(springs, groups)| calculate_solutions(springs, groups, &mut memoization)).sum();

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "?###???????? 3,2,1
        .###.##.#...
        .###.##..#..
        .###.##...#.
        .###.##....#
        .###..##.#..
        .###..##..#.
        .###..##...#
        .###...##.#.
        .###...##..#
        .###....##.#";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
