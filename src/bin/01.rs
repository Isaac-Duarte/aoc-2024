use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let mut values = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok());

        if let (Some(val1), Some(val2)) = (values.next(), values.next()) {
            list1.push(val1);
            list2.push(val2);
        } else {
            return None;
        }
    }

    if list1.is_empty() || list1.len() != list2.len() {
        return None;
    }

    list1.sort_unstable();
    list2.sort_unstable();

    let total_distance: u32 = list1
        .iter()
        .zip(&list2)
        .map(|(&a, &b)| (a - b).unsigned_abs())
        .sum();

    Some(total_distance)
}
pub fn part_two(input: &str) -> Option<u32> {
    let mut numbers = Vec::new();
    let mut counts = HashMap::new();

    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let mut values = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok());

        if let (Some(val1), Some(val2)) = (values.next(), values.next()) {
            numbers.push(val1);
            *counts.entry(val2).or_insert(0) += 1;
        } else {
            return None;
        }
    }

    let total_similarity_score: u32 = numbers
        .iter()
        .filter_map(|&num| counts.get(&num).map(|&count| count * num))
        .sum();

    Some(total_similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
