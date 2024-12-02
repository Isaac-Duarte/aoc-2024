use std::collections::HashMap;

use anyhow::{bail, Context};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut iter = line.split_whitespace();

        let val1_str = iter.next()?;
        let val2_str = iter.next()?;

        let val1 = val1_str.parse::<i32>().ok()?;
        let val2 = val2_str.parse::<i32>().ok()?;

        vec1.push(val1);
        vec2.push(val2);
    }

    if vec1.is_empty() || vec1.len() != vec2.len() {
        return None;
    }

    vec1.sort();
    vec2.sort();

    let total_distance: u32 = vec1
        .iter()
        .zip(&vec2)
        .map(|(a, b)| (a - b).abs() as u32)
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec1 = Vec::new();
    let mut counts = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut iter = line.split_whitespace();

        let val1_str = iter.next()?;
        let val2_str = iter.next()?;

        let val1 = val1_str.parse::<u32>().ok()?;
        let val2 = val2_str.parse::<u32>().ok()?;

        vec1.push(val1);
        *counts.entry(val2).or_insert(0) += 1;
    }

    let mut total_similarity_score = 0;

    for item in vec1 {
        if let Some(&count) = counts.get(&item) {
            total_similarity_score += count * item;
        }
    }

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
