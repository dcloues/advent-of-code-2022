use std::error::Error;

fn parse_map(input: &str) -> Result<Vec<Vec<i8>>,Box<dyn Error>> {
    input.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).map(|d| d as i8)).collect())
        .collect::<Option<Vec<Vec<i8>>>>()
        .ok_or_else(|| "bad input".into())
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let map = parse_map(input)?;
    let mut count = 0;
    let mut recorded: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    for (rownum, row) in map.iter().enumerate() {
        let mut tallest = -1i8;
        for (colnum, height) in row.iter().enumerate() {
            if *height > tallest {
                tallest = *height;
                if !recorded[rownum][colnum] {
                    recorded[rownum][colnum] = true;
                    count += 1;
                }
            }
        }

        tallest = -1;

        for (colnum, height) in row.iter().enumerate().rev() {
            if *height > tallest {
                tallest = *height;
                if !recorded[rownum][colnum] {
                    recorded[rownum][colnum] = true;
                    count += 1;
                }
            }
        }
    }

    for colnum in 0..map[0].len() {
        let mut tallest = -1i8;
        for (rownum, row) in map.iter().enumerate() {
            let height = row[colnum];
            if height > tallest {
                tallest = height;
                if !recorded[rownum][colnum] {
                    recorded[rownum][colnum] = true;
                    count += 1;
                }
            }
        }

        tallest  = -1i8;
        for (rownum, row) in map.iter().enumerate().rev() {
            let height = row[colnum];
            if height > tallest {
                tallest = height;
                if !recorded[rownum][colnum] {
                    recorded[rownum][colnum] = true;
                    count += 1;
                }
            }
        }
    }

    Ok(count.to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day8test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "21")
    }

    fn test_part2() {
        todo!("unimplemented");
        assert_eq!(part2(INPUT).unwrap(), "")
    }
}