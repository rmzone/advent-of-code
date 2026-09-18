pub mod part1;
pub mod part2;

pub fn calculate_priority(items: &Vec<&char>) -> i32 {
    items.iter().fold(0, |acc, &i| {
        let priority = match i {
            'a'..='z' => *i as i32 - 97 + 1,
            'A'..='Z' => *i as i32 - 65 + 27,
            _ => unreachable!(),
        };

        acc + priority
    })
}
