use crate::input;

pub fn day11() -> input::Result<()> {
    println!("Part 1:{}",part_1());
    println!("Part 2:{}",part_2());

    Ok(())
}

pub fn part_1() -> usize {
    let mut monkeys = parse().unwrap();
    let mut inspections = vec![0;monkeys.len()];

    for _ in 0..20 {
        for index in 0..monkeys.len() {
            let items: Vec<u64> = monkeys[index].items.drain(..).collect();
            let monkey = monkeys[index].clone();
            for old in items {
                // inspect
                inspections[index] += 1;
                // apply operation
                let new = monkey.operation.apply(old);
                // relieved
                let new = new / 3;
                // test
                let index = if new % monkey.test.divisible == 0 {
                    monkey.test.if_true
                } else {
                    monkey.test.if_false
                };
                let receiver = &mut monkeys[index];
                // throw
                receiver.items.push(new);
            }
        }
    }

    inspections.sort_unstable();
    inspections.iter().rev().take(2).product()
}

pub fn part_2() -> usize {
    let mut monkeys = parse().unwrap();
    let mut inspections = vec![0;monkeys.len()];
    let common_multiple: u64 = monkeys.iter().map(|x| x.test.divisible).product();

    for _ in 0..10000 {
        for index in 0..monkeys.len() {
            let items: Vec<u64> = monkeys[index].items.drain(..).collect();
            let monkey = monkeys[index].clone();
            for old in items {
                // inspect
                inspections[index] += 1;
                // apply operation
                let new = monkey.operation.apply(old);
                // relieved
                let new = new % common_multiple;
                // test
                let index = if new % monkey.test.divisible == 0 {
                    monkey.test.if_true
                } else {
                    monkey.test.if_false
                };
                let receiver = &mut monkeys[index];
                // throw
                receiver.items.push(new);
            }
        }
    }

    inspections.sort_unstable();
    inspections.iter().rev().take(2).product()
}

fn parse() -> Option<Vec<Monkey>> {
    let contents = input::load_day_file("day11.txt");
    // create monkeys, and put them in a vector
    let mut monkeys= Vec::new();
    for block in contents.split("\r\n\r\n") {

        let mut lines = block.lines().skip(1);
        
        let iter = lines.next().unwrap();
        let (_, items) =iter.split_once(": ")?;
        let items = items
        .split_terminator(", ")
        .filter_map(|x| x.parse().ok())
        .collect();
        
        let (_, operation) = lines.next()?.split_once("= old ")?;
        let (operator, value) = operation.split_once(" ")?;
        let value = match value {
            "old" => Value::Old,
            _ => {
                let n = value.parse().ok()?;
                Value::Num(n)
            },
        };
        
        let operation = match operator {
            "*" => Operation::Mul(value),
            "+" => Operation::Add(value),
            _ => panic!("Unknown operator"),
        };

        let (_, divisible) = lines.next()?.rsplit_once(" ")?;
        let divisible = divisible.parse().ok()?;
        let (_, if_true) = lines.next()?.rsplit_once(" ")?;
        let if_true = if_true.parse().ok()?;
        let (_, if_false) = lines.next()?.rsplit_once(" ")?;
        let if_false = if_false.parse().ok()?;

        let test = Test {
            divisible,
            if_true,
            if_false,
        };

        let monkey = Monkey {
            items,
            operation,
            test,
        };

        monkeys.push(monkey);
    }
    Some(monkeys)
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

#[derive(Clone)]
enum Operation {
    Mul(Value),
    Add(Value),
}

impl Operation {
    fn apply(&self, old: u64) -> u64 {
        match self {
            Operation::Mul(v) => old * v.get(old),
            Operation::Add(v) => old + v.get(old),
        }
    }
}

#[derive(Clone)]
enum Value {
    Old,
    Num(u64),
}

impl Value {
    fn get(&self, old: u64) -> u64 {
        match self {
            Value::Old => old,
            Value::Num(n) => *n,
        }
    }
}

#[derive(Clone)]
struct Test {
    divisible: u64,
    if_true: usize,
    if_false: usize,
}

