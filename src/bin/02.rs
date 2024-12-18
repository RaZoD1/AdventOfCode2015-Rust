advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let gifts = input
        .lines()
        .map(|line|
            line.split('x').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>()
        );

    let total_packing_paper = gifts
        .map(|g| {
            let sides = vec![g[0] * g[1], g[0] * g[2], g[1] * g[2]];
            let extra_paper = sides.iter().min().unwrap();
            sides.iter().map(|s| s * 2).sum::<u64>() + extra_paper
        })
        .sum();

    Some(total_packing_paper)
}

pub fn part_two(input: &str) -> Option<u64> {
    let gifts = input
        .lines()
        .map(|line|
            line.split('x').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>()
        );

    let total_packing_paper = gifts
        .map(|mut g| {
            g.sort();
            (g[0] + g[1]) * 2 + (g[0] * g[1] * g[2])
        })
        .sum();

    Some(total_packing_paper)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(58 + 43));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
