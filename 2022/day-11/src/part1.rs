use common::custom_error::Result;
use tracing::info;
use crate::{parse_input, Operation};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> Result<String> {
    let (input, mut monkeys) = parse_input(input)?;

    info!("monkeys: {:?}", &monkeys.len());

    for _ in 0..20 {
        // loop through each monkey and each item
        for i in 0..monkeys.len() as i32 {
            let mut queue: Vec<(i32, i32)> = vec![];
            let monkey = monkeys.get_mut(&i).unwrap();
            while let Some(item) = monkey.items.pop() {
                monkey.inspected_items_count += 1;

                let mut worry_level = match monkey.operation {
                    Operation::Add(x) => {
                        if x == -1 { item + x } else { item + item }
                    }
                    Operation::Multiply(x) => {
                        if x == -1 { item * x } else { item * item }
                    }
                };

                worry_level = worry_level / 3;

                let next_monkey_id = if worry_level % monkey.test == 0 {
                    monkey.test_true
                } else {
                    monkey.test_false
                };

                // fling object
                queue.push((next_monkey_id, worry_level));
            }

            for (next_monkey_id, worry_level) in queue {
                let next_monkey = monkeys.get_mut(&next_monkey_id).unwrap();
                next_monkey.items.push(worry_level);
            }
        }
    }

    // find top 2 inspected_items_count and multiply them
    //let result = monkeys.iter()

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_process() -> Result<()> {
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
        assert_eq!("10605", process(input)?);
        Ok(())
    }
}
