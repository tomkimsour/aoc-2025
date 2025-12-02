advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let mut res = 0;
    let _ = input.lines().into_iter().fold(50, |acc, line| {
        let (direction, number_str) = line.split_at(1);
        let parsed_number: i64 = number_str.to_string().parse::<i64>().unwrap() % 100;
        let mut current_sum = match direction {
            "L" => acc - parsed_number,
            _ => acc + parsed_number,
        };
        if current_sum > 99 {
            current_sum -= 100;
        } else if current_sum < 0 {
            current_sum += 100;
        }

        if current_sum == 0 {
            res += 1;
        }
        println!("{:?}", current_sum);
        current_sum
    });
    Some(res)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut res = 0;
    let _ = input.lines().into_iter().fold(50, |acc, line| {
        let (direction, number_str) = line.split_at(1);
        let parsed_number: i64 = number_str.to_string().parse::<i64>().unwrap();
        res += parsed_number / 100;
        println!(
            "{} / 100 = {} - > res : {} ",
            parsed_number,
            parsed_number / 100,
            res
        );
        let parsed_number = parsed_number % 100;
        let mut current_sum = match direction {
            "L" => acc - parsed_number,
            _ => acc + parsed_number,
        };
        if current_sum > 99 {
            current_sum -= 100;
            res += 1;
            println!("Res : {:}", res);
        } else if current_sum < 0 {
            current_sum += 100;
            res += 1;
            println!("Res : {:}", res);
        }

        println!("{:?}", current_sum);
        current_sum
    });
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
