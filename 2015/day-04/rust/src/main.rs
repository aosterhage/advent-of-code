use md5::{Digest, Md5};

/// Personalized puzzle input.
static INPUT: &str = include_str!("../../input.txt");

fn main() {
    // Print answers to both parts.
    println!("Part One: {}", part_one(INPUT));
    println!("Part Two: {}", part_two(INPUT));
}

/// Part One.
///
/// The prompt is fairly straightforward, we should:
/// - create a string of our secret key followed by a number
/// - hash that string using MD5
/// - find the lowest positive number whose hash starts with 5 zeroes
fn part_one(input: &str) -> String {
    find_number_for_md5(input, "00000").to_string()
}

#[test]
fn part_one_tests() {
    assert_eq!(part_one("abcdef"), "609043");
}

/// Part Two.
///
/// This solution is identical to part one with just a small change in the
/// "starts_with" string.
fn part_two(input: &str) -> String {
    find_number_for_md5(input, "000000").to_string()
}

/// Create an iterator over an unbound range of positive numbers, then find the
/// first position in that iterator where the MD5 hash starts with some input.
fn find_number_for_md5(secret_key: &str, starts_with: &str) -> usize {
    std::iter::successors(Some(1u32), |n| n.checked_add(1))
        .position(|n| {
            let mut hasher = Md5::new();
            hasher.update(format!("{secret_key}{n}"));
            format!("{:x}", hasher.finalize()).starts_with(starts_with)
        })
        .unwrap()
        + 1 // + 1 because `position` returns the 0-based index but the iterators start at n = 1
}
