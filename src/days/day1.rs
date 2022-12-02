use std::error::Error;

fn sum_batches(input: &str) -> Result<Vec<i64>, Box<dyn Error>> {
    let mut batch_totals: Vec<i64> = input
        .lines()
        .map(str::trim)
        .try_fold(
            vec![vec![]],
            |mut acc: Vec<Vec<i64>>, l: &str| -> Result<_, Box<dyn Error>> {
                if l.is_empty() {
                    acc.push(vec![]);
                } else {
                    acc.last_mut().unwrap().push(l.parse()?);
                }
                Ok(acc)
            },
        )?
        .iter()
        .map(|b| b.iter().sum())
        .collect();

    batch_totals.sort();
    Ok(batch_totals)
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    sum_batches(input).map(|bs| format!("{}", bs.last().unwrap()))
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    sum_batches(input).map(|bs| format!("{}", bs.iter().rev().take(3).sum::<i64>()))
}

#[cfg(test)]
mod test {
    use super::part1;
    use super::part2;

    const TEST_INPUT: &str = "1000
    2000
    3000
    
    4000
   
    5000
    6000
   
    7000
    8000
    9000
   
    10000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT).unwrap(), "24000")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT).unwrap(), "45000")
    }
}
