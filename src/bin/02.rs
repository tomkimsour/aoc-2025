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
            let mut local_res = 0;
            let min: u64 = l.parse().unwrap();
            let max: u64 = r.parse().unwrap();
            for i in min..max + 1 {
                let i_len = 1 + i.checked_ilog10().unwrap() as usize;
                if !i_len.is_multiple_of(2) {
                    continue;
                }
                let i_str = i.to_string();
                let (l_nb, r_nb) = i_str.split_at(i_len / 2);
                if l_nb == r_nb {
                    println!("{}", i);
                    local_res += i;
                }
            }
            local_res
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res: u64 = parse(input.trim())
        .into_iter()
        .map(|(l, r)| {
            let mut local_res = 0;
            let min: u64 = l.parse().unwrap();
            let max: u64 = r.parse().unwrap();
            for i in min..max + 1 {
                let i_len = 1 + i.checked_ilog10().unwrap() as usize;
                let i_str = i.to_string();
                let mut amount_of_split = 2;
                while amount_of_split <= i_len {
                    let (l_nb, r_nb) = i_str.split_at(i_len / amount_of_split);
                    let mut l_concat: String = "".to_string();
                    for _ in 0..amount_of_split - 1 {
                        l_concat += l_nb;
                    }
                    if l_concat == r_nb {
                        println!("{}", i);
                        local_res += i;
                        break;
                    }
                    amount_of_split += 1;
                }
            }
            local_res
        })
        .sum();
    Some(res)
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
        assert_eq!(result, Some(4174379265));
    }
}
