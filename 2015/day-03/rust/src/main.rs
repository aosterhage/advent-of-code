/// Personalized puzzle input.
static INPUT: &str = include_str!("../../input.txt");

fn main() {
    // Print answers to both parts.
    println!("Part One: {}", part_one(INPUT));
    println!("Part Two: {}", part_two(INPUT));
}

/// Part One.
///
/// Santa's location can be represented as Cartesian coordinates in a
/// two-dimensional space (x, y) so we create a struct [Location] that stores
/// the x and y axes positions.
///
/// In order to track the houses Santa has visited, we will use a
/// [std::collections::HashSet] where the keys are [Location]s.
/// - We choose to use a Set because we don't have any data to associate with
///   the keys.
/// - We choose to use the Hash variant rather than the BTree variant because we
///   don't care that the keys are ordered: we only need to know how many keys
///   are stored.
///
/// If you are not sure what data structure to use, the Rust documentation has
/// [a great write-up on when to use which].
///
/// [a great write-up on when to use which]:
/// https://doc.rust-lang.org/std/collections/#when-should-you-use-which-collection
fn part_one(input: &str) -> String {
    // `scan` creates an iterator over Santa's location (by converting the input moves into new locations).
    // `fold` reduces those locations into a set of visited houses.
    input
        .chars()
        .scan(Location::STARTING_LOCATION, |loc, c| {
            loc.make_move(c);
            Some(*loc)
        })
        .fold(
            std::collections::HashSet::from([Location::STARTING_LOCATION]),
            |mut houses, loc| {
                houses.insert(loc);
                houses
            },
        )
        .len()
        .to_string()
}

#[test]
fn part_one_tests() {
    assert_eq!(part_one(">"), "2");
    assert_eq!(part_one("^>v<"), "4");
    assert_eq!(part_one("^v^v^v^v^v"), "2");
}

/// Part Two.
///
/// This solution is identical to part one but we need to separately iterate the
/// input, once for Santa, and once for Robo-Santa.
///
/// Each of these iterations should skip every other element in the input,
/// i.e. all even indexes are for Santa while all odd indexes are for Robo-Santa.
fn part_two(input: &str) -> String {
    let mut houses = std::collections::HashSet::from([Location::STARTING_LOCATION]);

    // Get Santa's visited houses.
    houses = input
        .chars()
        .step_by(2)
        .scan(Location::STARTING_LOCATION, |loc, c| {
            loc.make_move(c);
            Some(*loc)
        })
        .fold(houses, |mut houses, loc| {
            houses.insert(loc);
            houses
        });

    // Get Robo-Santa's visited houses.
    houses = input
        .chars()
        .skip(1)
        .step_by(2)
        .scan(Location::STARTING_LOCATION, |loc, c| {
            loc.make_move(c);
            Some(*loc)
        })
        .fold(houses, |mut houses, loc| {
            houses.insert(loc);
            houses
        });

    houses.len().to_string()
}

#[test]
fn part_two_tests() {
    assert_eq!(part_two("^v"), "3");
    assert_eq!(part_two("^>v<"), "3");
    assert_eq!(part_two("^v^v^v^v^v"), "11");
}

/// Represents a house location that Santa can visit.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    const STARTING_LOCATION: Location = Location { x: 0, y: 0 };

    /// Given a radio instruction, alter `self` to reflect the new location.
    fn make_move(&mut self, move_instruction: char) {
        match move_instruction {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _ => panic!(),
        }
    }
}
