use std::ptr::copy_nonoverlapping;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let floor: i32 = input.chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }).sum();

    Some(floor)
}

pub fn part_two(input: &str) -> Option<u64> {
    let deltas = input.chars().map(|c| match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    });
    let mut acc = 0;
    for (index, d) in deltas.enumerate() {
        acc += d;
        if(acc < 0) {
            return Some((index + 1) as u64);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
