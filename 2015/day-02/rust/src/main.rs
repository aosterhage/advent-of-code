/// Personalized puzzle input.
static INPUT: &str = include_str!("../../input.txt");

fn main() {
    // Print answers to both parts.
    println!("Part One: {}", part_one(INPUT));
    println!("Part Two: {}", part_two(INPUT));
}

/// Part One.
///
/// For a given length, width, and height, calculating the paper is
/// straightforward (we are given the equation).
///
/// The other problem to solve is how to take the input and get the length,
/// width, and height for each line. Note that each is separated by an `x`. We
/// can split the line ([std::str::Split]) by this delimiter to get the 3
/// dimensions.
///
/// In order to calculate the total across all lines, we will utilize
/// [std::iter::Iterator::fold] again.
fn part_one(input: &str) -> String {
    /// Calculate wrapping paper given box dimensions.
    fn wrapping_paper(l: u32, w: u32, h: u32) -> u32 {
        (2 * l * w) + (2 * w * h) + (2 * l * h) + [l * w, w * h, l * h].iter().min().unwrap()
    }

    input
        .lines()
        .fold(0u32, |total_paper, e| {
            let mut iter = e.split("x").map(|e| e.parse::<u32>().unwrap());
            let (l, w, h) = (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            );
            total_paper + wrapping_paper(l, w, h)
        })
        .to_string()
}

#[test]
fn part_one_tests() {
    assert_eq!(part_one("2x3x4"), "58");
    assert_eq!(part_one("1x1x10"), "43");
}

/// Part Two.
///
/// This is identical to part one, but we use a different equation with the
/// dimensions.
fn part_two(input: &str) -> String {
    /// Calculate ribbon given box dimensions.
    fn ribbon(l: u32, w: u32, h: u32) -> u32 {
        2 * [l + w, w + h, l + h].iter().min().unwrap() + (l * w *h)
    }

    input
        .lines()
        .fold(0u32, |total_ribbon, e| {
            let mut iter = e.split("x").map(|e| e.parse::<u32>().unwrap());
            let (l, w, h) = (
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            );
            total_ribbon + ribbon(l, w, h)
        })
        .to_string()
}

#[test]
fn part_two_tests() {
    assert_eq!(part_two("2x3x4"), "34");
    assert_eq!(part_two("1x1x10"), "14");
}
