use aochelpers::Coordinate;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process( 
    input: &str,
) -> miette::Result<String, AocError> {
    let stars = generate_starfield(&input);
    Ok(star_distance(&stars, 2).to_string())
}


fn star_distance(stars: &[Coordinate<i64>], empty_cost: i64) -> i64 {
    let max_x = stars.iter().map(|c: &Coordinate<i64>| c.x).max().unwrap();
    let max_y = stars.iter().map(|c: &Coordinate<i64>| c.y).max().unwrap();

    let empty_columns = (0..max_x).filter(|x| stars.iter().all(|c| c.x != *x)).collect::<Vec<_>>();
    let empty_rows= (0..max_y).filter(|y| stars.iter().all(|c| c.y != *y)).collect::<Vec<_>>();
    let mut total = 0;
    for (idx, left_star) in stars.iter().enumerate() {
        for right_star in stars.iter().skip(idx+1){
            let empty_cols_count = empty_columns.iter().filter(|c| *c > &right_star.x.min(left_star.x) && *c < &right_star.x.max(left_star.x)).count() as i64;
            let empty_rows_count = empty_rows.iter().filter(|c| *c > &right_star.y.min(left_star.y) && *c < &right_star.y.max(left_star.y)).count() as i64;
            let distance = left_star.manhattan_distance(right_star) + ((empty_cols_count + empty_rows_count) * (empty_cost -1));
            total += distance
        }
    }
    total
}

fn generate_starfield(data: &str) -> Vec<Coordinate<i64>> {
    let mut stars = Vec::new();
    for (y, line) in data.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                stars.push(Coordinate {x: x as i64, y: y as i64})
            }
        }
    }
    stars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
