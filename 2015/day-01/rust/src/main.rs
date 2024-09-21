/// Personalized puzzle input.
static INPUT: &str = include_str!("../../input.txt");

fn main() {
    // Print answers to both parts.
    println!("Part One: {}", part_one(INPUT));
    println!("Part Two: {}", part_two(INPUT));
}

/// Part One.
///
/// We know that we must iterate the entire string to get our answer. We also see
/// that each character can be converted to an integer:
/// - `(` = `1`
/// - `)` = `-1`
///
/// This can be solved by mapping the characters to their integer counterparts
/// and then reducing those integers to a single value using a summation. Rust
/// has both the [std::iter::Iterator::map] and [std::iter::Iterator::reduce]
/// methods, but Rust also provides the functionality of both in a single call:
/// the [std::iter::Iterator::fold] method. We will create an iterator over the
/// characters in the input string ([std::str::Chars]) and then fold
/// that iterator into a single integer value by accumulating them using the
/// [std::ops::Add] trait.
fn part_one(input: &str) -> String {
    // Fold all characters into a single u32.
    input
        .chars()
        .fold(0i32, |floor, c| {
            floor
                + match c {
                    '(' => 1,
                    ')' => -1,
                    // Panic on any unexpected input.
                    _ => panic!(),
                }
        })
        .to_string()
}

#[test]
fn part_one_tests() {
    assert_eq!(part_one("(())"), "0");
    assert_eq!(part_one("()()"), "0");
    assert_eq!(part_one("((("), "3");
    assert_eq!(part_one("(()(()("), "3");
    assert_eq!(part_one("))((((("), "3");
    assert_eq!(part_one("())"), "-1");
    assert_eq!(part_one("))("), "-1");
    assert_eq!(part_one(")))"), "-3");
    assert_eq!(part_one(")())())"), "-3");
}

/// Part Two.
///
/// We no longer need to iterate the entire string, we can stop as soon as the
/// current floor is less than `0`. While we could choose to iterate the
/// characters in the input string and `break` when some mutable floor
/// value is negative, I prefer to go a bit more functional for my solution.
///
/// I will use [std::iter::Iterator::scan] to maintain state while iterating the
/// characters from the input string. This state will be the current floor santa
/// is on. scan then returns an iterator whose values will be the current floor
/// santa is on after moving floors according to the associated
/// instruction. This can be fed into the [std::iter::Iterator::position] method
/// to find the first position where a predicate is true, in this case `floor <
/// 0`.
fn part_two(input: &str) -> String {
    // Use `scan` to accumulate the floor that santa is currently on, this
    // returns an iterator that we can chain to `position`.
    (input
        .chars()
        .scan(0i32, |floor, c| {
            *floor += match c {
                '(' => 1,
                ')' => -1,
                // Panic on any unexpected input.
                _ => panic!(),
            };
            Some(*floor)
        })
        .position(|floor| floor < 0)
        .unwrap()
        + 1) // + 1 because the prompt asks for a 1-based index, while `position` returns the 0-based index.
    .to_string()
}

#[test]
fn part_two_tests() {
    assert_eq!(part_two(")"), "1");
    assert_eq!(part_two("()())"), "5");
}
