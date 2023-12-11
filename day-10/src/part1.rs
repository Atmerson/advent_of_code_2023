use crate::custom_error::AocError;
use glam::*;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, Read};
use std::ops::*;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {

    let mut grid: Vec<Vec<Cell>> = input
        .lines()
        .map(|s| s.chars().map(Cell::from_char).collect())
        .collect();

    let start = (0..grid.len())
        .find_map(|i| (0..grid[i].len()).find_map(|j| (grid[i][j].start == true).then_some((i, j))))
        .unwrap();

    // Calculate starting entries
    grid[start.0][start.1].north = start.0 > 0 && grid[start.0 - 1][start.1].south;
    grid[start.0][start.1].south = start.0 < grid.len() - 1 && grid[start.0 + 1][start.1].north;
    grid[start.0][start.1].west = start.1 > 0 && grid[start.0][start.1 - 1].east;
    grid[start.0][start.1].east =
        start.1 < grid[start.0].len() - 1 && grid[start.0][start.1 + 1].west;

    let mut distance = 0;
    let mut frontier = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    frontier.push((Reverse(0), start));
    while let Some((Reverse(k), (i, j))) = frontier.pop() {
        if !visited.insert((i, j)) {
            continue;
        }
        distance = distance.max(k);
        let cell = &grid[i][j];
        if cell.north {
            frontier.push((Reverse(k + 1), (i - 1, j)));
        }
        if cell.south {
            frontier.push((Reverse(k + 1), (i + 1, j)));
        }
        if cell.west {
            frontier.push((Reverse(k + 1), (i, j - 1)));
        }
        if cell.east {
            frontier.push((Reverse(k + 1), (i, j + 1)));
        }
    }
    Ok(distance.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Cell {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
    start: bool,
    outside: bool,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self {
                north: true,
                south: true,
                ..Default::default()
            },
            '-' => Self {
                east: true,
                west: true,
                ..Default::default()
            },
            'L' => Self {
                north: true,
                east: true,
                ..Default::default()
            },
            'J' => Self {
                north: true,
                west: true,
                ..Default::default()
            },
            '7' => Self {
                south: true,
                west: true,
                ..Default::default()
            },
            'F' => Self {
                south: true,
                east: true,
                ..Default::default()
            },
            'O' | 'I' | '.' => Default::default(),
            'S' => Self {
                start: true,
                ..Default::default()
            },
            _ => panic!("{:?}", c),
        }
    }

    fn expand(&self) -> [[Cell; 2]; 2] {
        [
            [
                *self,
                Cell {
                    west: self.east,
                    east: self.east,
                    ..Default::default()
                },
            ],
            [
                Cell {
                    north: self.south,
                    south: self.south,
                    ..Default::default()
                },
                Default::default(),
            ],
        ]
    }

    fn to_char(&self) -> char {
        if self.outside {
            'O'
        } else if self.start {
            'S'
        } else if *self == Self::from_char('.') {
            '.'
        } else if *self == Self::from_char('|') {
            '|'
        } else if *self == Self::from_char('-') {
            '-'
        } else if *self == Self::from_char('L') {
            'L'
        } else if *self == Self::from_char('J') {
            'J'
        } else if *self == Self::from_char('7') {
            '7'
        } else if *self == Self::from_char('F') {
            'F'
        } else {
            '?'
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
