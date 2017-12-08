use std::io::{self, Read};
use std::collections::HashMap;


struct Guard<'a> {
    target: &'a str,
    ammount: isize,
    op: &'a str,
}

impl<'a> Guard<'a> {
    fn eval(&self, state: &HashMap<&str, isize>) -> bool {
        match self.op {
            "<" => state[self.target] < self.ammount,
            "<=" => state[self.target] <= self.ammount,
            ">" => state[self.target] > self.ammount,
            ">=" => state[self.target] >= self.ammount,
            "==" => state[self.target] == self.ammount,
            "!=" => state[self.target] != self.ammount,
            _ => unreachable!(),
        }
    }
}

struct Action<'a> {
    ammount: isize,
    target: &'a str,
    guard: Guard<'a>,
}

impl<'a> Action<'a> {
    fn eval(&self, state: &mut HashMap<&str, isize>, max: &mut isize) {
        if self.guard.eval(state) {
            *state.get_mut(self.target).unwrap() += self.ammount;
            if *max < state[self.target] {
                *max = state[self.target];
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut input).unwrap();

    let mut registers = HashMap::new();
    let mut actions = Vec::new();

    let mut max = 0;

    for l in input.lines() {
        let mut l = l.split_whitespace();
        let reg = l.next().unwrap();
        let action = l.next().unwrap();
        let mut val = l.next().unwrap().parse::<isize>().unwrap();
        val = match action {
            "inc" => val,
            "dec" => -val,
            _ => unreachable!(),
        };
        l.next().unwrap(); // if
        let guard_target = l.next().unwrap();
        let op = l.next().unwrap();
        let num = l.next().unwrap().parse::<isize>().unwrap();
        registers.insert(reg, 0);
        actions.push(Action {
            ammount: val,
            target: reg,
            guard: Guard {
                target: guard_target,
                ammount: num,
                op: op,
            },
        });
    }

    for a in actions {
        a.eval(&mut registers, &mut max);
    }

    println!("Part1: {}", registers.values().max().unwrap());
    println!("Part2: {}", max);
}
