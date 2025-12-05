advent_of_code::solution!(2);

// I want this to return
// [(begin:str, end:str), ...,(begin:str, end:str)]
fn parse(input: &str) -> Vec<(&str, &str)> {
    input
        .split(',')
        .map(|v| v.split_once('-').unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let res: u64 = parse(input.trim())
        .into_iter()
        .map(|(l, r)| {
            let min: u64 = l.parse().unwrap();
            let max: u64 = r.parse().unwrap();
            let min_len = 1 + min.checked_ilog10().unwrap() as usize;
            if (min_len / 2)
            for i in min..max {
                let i_len = 1 + i.checked_ilog10().unwrap() as usize;
                if !i_len.is_multiple_of(2) {
                    continue;
                }
            }
            0
        })
        .sum();
    Some(0)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
