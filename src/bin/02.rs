use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(2);

fn list_is_safe(list: &[usize]) -> bool {
    if list.len() < 2 {
        return true;
    }

    let mut iter = list.iter();
    let mut prev = *iter.next().unwrap();
    let curr = *iter.next().unwrap();

    let ordering = curr.cmp(&prev);

    if ordering == Ordering::Equal || curr.abs_diff(prev) > 3 {
        return false;
    }

    prev = curr;

    for &e in iter {
        let new_ordering = e.cmp(&prev);
        if new_ordering == Ordering::Equal || new_ordering != ordering || e.abs_diff(prev) > 3 {
            return false;
        }

        prev = e;
    }

    true
}

fn list_is_safe_with_dampener(list: &[usize]) -> bool {
    if list_is_safe(list) {
        return true;
    }

    for i in 0..list.len() {
        let mut new_list = Vec::with_capacity(list.len() - 1);
        new_list.extend_from_slice(&list[..i]);
        new_list.extend_from_slice(&list[i + 1..]);
        if list_is_safe(&new_list) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|num| num.parse::<usize>().ok())
                    .collect_vec()
            })
            .filter(|list| list_is_safe(list))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|num| num.parse::<usize>().ok())
                    .collect_vec()
            })
            .filter(|list| list_is_safe_with_dampener(list))
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
