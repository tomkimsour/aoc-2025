advent_of_code::solution!(3);
use std::cmp;

// I want this to return
// [
// [1,2,3,4...5]
// [1,2,3,4...5]
// [1,2,3,4...5]
// [1,2,3,4...5]
// ]
fn parse(input: &str) -> Vec<Vec<String>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = parse(input)
        .into_iter()
        .map(|line| {
            let mut max = 0;
            let mut it = line.into_iter();
            while let Some(w1) = it.next() {
                let it2 = it.clone();
                for w2 in it2 {
                    max = cmp::max((w1.to_owned() + &w2).parse::<u64>().unwrap(), max);
                }
            }
            max
        })
        .sum::<u64>();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = parse(input)
        .into_iter()
        .map(|line| {
            let mut max_res = "".to_owned();
            // let s assume an array of size of line with indices of the current
            // [1,1,1,1,1,1,1,1,0,0,0,0,0,0]
            // [1,1,1,1,1,1,1,0,1,0,0,0,0,0]
            // [1,1,1,1,1,1,1,0,0,1,0,0,0,0]
            // ...
            // [0,0,0,0,0,0,1,1,1,1,1,1,1,1]
            let line_size = line.len();
            let mut beg = 0;
            for i in 0..12 {
                let window = &line[beg..(line_size - 11 + i)];
                // println!("{:?}", window);
                let max_window_value = window
                    .iter()
                    .enumerate()
                    .max_by(|(_idx2, val2), (_idx1, val1)| match val2.cmp(val1) {
                        cmp::Ordering::Equal => cmp::Ordering::Greater,
                        other => other,
                    })
                    .unwrap();
                // println!(
                //     "{:?} : ({}..{} - {} + {}) ",
                //     max_window_value, beg, line_size, 11, i
                // );
                beg += max_window_value.0 + 1;
                max_res += max_window_value.1;
            }
            // println!("---- {} ", max_res);
            max_res.parse::<u64>().unwrap()
        })
        .sum::<u64>();
    // println!("{:?}", res);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
