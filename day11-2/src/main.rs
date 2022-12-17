use std::{collections::HashMap, sync::Mutex};

use anyhow::{bail, Result};
use once_cell::sync::OnceCell;
use util::Input;

static NEXT_ID: OnceCell<Mutex<usize>> = OnceCell::with_value(Mutex::new(0));

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Item {
    // id is just for debugging
    id: usize,
    worry_level: usize,
}

impl Item {
    fn new(worry_level: usize) -> Self {
        let id = *NEXT_ID.get().unwrap().lock().unwrap();
        *NEXT_ID.get().unwrap().lock().unwrap() += 1;
        Self { id, worry_level }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Old,
    Int(usize),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<Item>,
    operation: Operation,
    operand: Operand,
    test_divisor: usize,
    if_true: usize,
    if_false: usize,
    inspect_count: usize,
}

impl Monkey {
    fn handle_items(&mut self, group_mod: usize) -> Vec<(usize, Item)> {
        let mut output = vec![];
        for mut item in std::mem::take(&mut self.items) {
            self.inspect_count += 1;
            let operand = match self.operand {
                Operand::Old => item.worry_level,
                Operand::Int(v) => v,
            };
            let new_worry = match self.operation {
                Operation::Add => item.worry_level + operand,
                Operation::Mul => item.worry_level * operand,
            };

            let new_monkey = if new_worry % self.test_divisor == 0 {
                self.if_true
            } else {
                self.if_false
            };

            item.worry_level = new_worry % group_mod;

            output.push((new_monkey, item));
        }

        output
    }
}

fn get_prefix(line: &str, prefix: &str) -> Result<String> {
    let line = line.trim();
    if !line.starts_with(prefix) {
        bail!("{} does not have prefix {}", line, prefix);
    }

    Ok(line.chars().skip(prefix.len()).collect())
}

fn parse_monkey(lines: &[String]) -> Result<Monkey> {
    let first = get_prefix(&lines[0], "Monkey ")?;
    let id: usize = first
        .chars()
        .take_while(|c| *c != ':')
        .collect::<String>()
        .parse()?;

    let items = get_prefix(&lines[1], "Starting items: ")?
        .split(", ")
        .map(|s| s.parse())
        .collect::<Result<Vec<usize>, _>>()?
        .into_iter()
        .map(Item::new)
        .collect::<Vec<_>>();

    let operation_str = get_prefix(&lines[2], "Operation: new = old ")?;

    let operation = match operation_str.chars().next().unwrap() {
        '*' => Operation::Mul,
        '+' => Operation::Add,
        _ => bail!("Not a monkey"),
    };

    let operand = operation_str.chars().skip(2).collect::<String>();

    let operand = match operand.as_str() {
        "old" => Operand::Old,
        s => Operand::Int(s.parse()?),
    };

    let test = get_prefix(&lines[3], "Test: divisible by ")?;

    let test_divisor: usize = test.parse()?;

    let if_true: usize = get_prefix(&lines[4], "If true: throw to monkey ")?.parse()?;
    let if_false: usize = get_prefix(&lines[5], "If false: throw to monkey ")?.parse()?;

    Ok(Monkey {
        id,
        items,
        operation,
        operand,
        test_divisor,
        if_true,
        if_false,
        inspect_count: 0,
    })
}

fn print_inspections(monkeys: &HashMap<usize, Monkey>, round: usize) {
    if round == 1 || round == 20 || (round % 1000 == 0 && round != 0) {
        println!("== After round {round} ==");
        let indicies = monkeys.keys().copied().collect::<Vec<_>>();
        for i in indicies {
            println!(
                "Monkey {} inspected items {} times.",
                i, monkeys[&i].inspect_count
            );
        }
        println!();
    }
}

fn main() -> Result<()> {
    let input = Input::new()?.into_lines()?;

    let mut monkeys = HashMap::new();
    for lines in input.chunks(7) {
        let monkey = parse_monkey(lines)?;
        println!("Monkey {}: {:?}", monkey.id, monkey);
        monkeys.insert(monkey.id, monkey);
    }

    let mut monkey_ids = monkeys.keys().copied().collect::<Vec<_>>();
    monkey_ids.sort();

    let group_mod = monkeys.values().map(|m| m.test_divisor).product();
    println!("Group mod: {group_mod}");

    for r in 0..10_000 {
        print_inspections(&monkeys, r);
        for monkey in &monkey_ids {
            let output = monkeys.get_mut(monkey).unwrap().handle_items(group_mod);
            for (m, item) in output {
                monkeys.get_mut(&m).unwrap().items.push(item);
            }
        }
    }

    println!("{monkeys:#?}");
    println!();

    let mut inspection_counts = monkeys
        .values()
        .map(|m| m.inspect_count)
        .collect::<Vec<_>>();
    inspection_counts.sort();
    let monkey_business = inspection_counts[inspection_counts.len() - 1]
        * inspection_counts[inspection_counts.len() - 2];
    println!("Monkey business: {monkey_business}");

    Ok(())
}
