use std::{collections::HashMap, ops::Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Compare {
    GreaterThan,
    LessThan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Decision {
    Accept,
    Reject,
    Workflow(&'static str),
}

impl Decision {
    fn parse(decision: &'static str) -> Self {
        match decision {
            "A" => Decision::Accept,
            "R" => Decision::Reject,
            workflow => Decision::Workflow(workflow),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    key: u8,
    compare: Compare,
    value: u32,
    decision: Decision,
}

type Part = HashMap<u8, u32>;

type PartInterval = HashMap<u8, Range<u32>>;

impl Rule {
    fn process(&self, part: &Part) -> Option<Decision> {
        let part_value = part[&self.key];
        match self.compare {
            Compare::GreaterThan => {
                if part_value > self.value {
                    Some(self.decision)
                } else {
                    None
                }
            }
            Compare::LessThan => {
                if part_value < self.value {
                    Some(self.decision)
                } else {
                    None
                }
            }
        }
    }

    fn process_interval(
        &self,
        mut part: PartInterval,
    ) -> (Option<(PartInterval, Decision)>, Option<PartInterval>) {
        let part_interval = part[&self.key].clone();
        match self.compare {
            Compare::GreaterThan => {
                if part_interval.start > self.value {
                    (Some((part, self.decision)), None)
                } else if part_interval.end - 1 <= self.value {
                    (None, Some(part))
                } else {
                    let mut decided = part.clone();
                    decided.insert(self.key, self.value + 1..part_interval.end);
                    part.insert(self.key, part_interval.start..self.value + 1);
                    (Some((decided, self.decision)), Some(part))
                }
            }
            Compare::LessThan => {
                if part_interval.end - 1 < self.value {
                    (Some((part, self.decision)), None)
                } else if part_interval.start >= self.value {
                    (None, Some(part))
                } else {
                    let mut decided = part.clone();
                    decided.insert(self.key, part_interval.start..self.value);
                    part.insert(self.key, self.value..part_interval.end);
                    (Some((decided, self.decision)), Some(part))
                }
            }
        }
    }

    fn parse(rule: &'static str) -> Self {
        let mut parts = rule.split(':');
        let condition = parts.next().unwrap().as_bytes();
        let key = condition[0];
        let compare = match condition[1] {
            b'>' => Compare::GreaterThan,
            b'<' => Compare::LessThan,
            _ => panic!("Invalid rule"),
        };
        let value = std::str::from_utf8(&condition[2..])
            .unwrap()
            .parse()
            .unwrap();
        let decision = Decision::parse(parts.next().unwrap());
        Self {
            key,
            compare,
            value,
            decision,
        }
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    fallback_decision: Decision,
}

impl Workflow {
    fn process(&self, part: &Part) -> Decision {
        for rule in &self.rules {
            if let Some(decision) = rule.process(part) {
                return decision;
            }
        }
        self.fallback_decision
    }

    fn process_interval(&self, part: PartInterval) -> Vec<(PartInterval, Decision)> {
        let mut result = vec![];
        let mut maybe_undecided = Some(part);
        for rule in &self.rules {
            let Some(undecided) = maybe_undecided else {
                break;
            };
            let (maybe_rule_decided, maybe_rule_undecided) = rule.process_interval(undecided);
            if let Some(decided) = maybe_rule_decided {
                result.push(decided);
            }
            maybe_undecided = maybe_rule_undecided;
        }
        if let Some(undecided) = maybe_undecided {
            result.push((undecided, self.fallback_decision));
        }
        result
    }

    fn parse(workflow: &'static str) -> Self {
        let rules = workflow.split(',').collect::<Vec<_>>();
        let (rules, fallback_decision) = rules.split_at(rules.len() - 1);
        let rules = rules.iter().map(|rule| Rule::parse(rule)).collect();
        let fallback_decision = Decision::parse(fallback_decision[0]);
        Self {
            rules,
            fallback_decision,
        }
    }
}

#[derive(Debug, Clone)]
struct System {
    workflows: HashMap<&'static str, Workflow>,
}

impl System {
    /// Returns if accepted
    fn process(&self, part: &Part) -> bool {
        let mut workflow = &self.workflows["in"];
        loop {
            match workflow.process(part) {
                Decision::Accept => return true,
                Decision::Reject => return false,
                Decision::Workflow(name) => workflow = &self.workflows[name],
            }
        }
    }

    fn find_accepted(&self, part: PartInterval) -> Vec<PartInterval> {
        let mut stack = vec![(part, &self.workflows["in"])];
        let mut result = vec![];
        while let Some((part, workflow)) = stack.pop() {
            for (part, decision) in workflow.process_interval(part) {
                match decision {
                    Decision::Accept => result.push(part),
                    Decision::Reject => {}
                    Decision::Workflow(name) => stack.push((part, &self.workflows[name])),
                }
            }
        }
        result
    }

    fn count_accepted(&self, part: PartInterval) -> usize {
        let accepted = self.find_accepted(part);
        accepted
            .iter()
            .map(|part| part.values().map(|range| range.len()).product::<usize>())
            .sum()
    }

    fn parse(system: &'static str) -> Self {
        let mut workflows = HashMap::new();
        for workflow in system.lines() {
            let mut parts = workflow.split('{');
            let name = parts.next().unwrap();
            let workflow = Workflow::parse(parts.next().unwrap().split('}').next().unwrap());
            workflows.insert(name, workflow);
        }
        Self { workflows }
    }
}

fn parse_part(part: &str) -> Part {
    let mut result = Part::new();
    let part = part.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
    for pair in part.split(',') {
        let mut pair = pair.split('=');
        let key = pair.next().unwrap().as_bytes()[0];
        let value = pair.next().unwrap().parse().unwrap();
        result.insert(key, value);
    }
    result
}

fn parse(input: &'static str) -> (System, Vec<Part>) {
    let mut parts = input.split("\n\n");
    let system = System::parse(parts.next().unwrap());
    let parts = parts
        .next()
        .unwrap()
        .lines()
        .map(|part| parse_part(part))
        .collect();
    (system, parts)
}

fn run(system: &System, parts: &[Part]) -> u32 {
    parts
        .iter()
        .filter(|part| system.process(part))
        .map(|part| part.values().sum::<u32>())
        .sum()
}

const INPUT: &str = include_str!("input.txt");

fn main() {
    let (system, parts) = parse(INPUT);
    println!("Part 1: {}", run(&system, &parts));
    println!(
        "Part 2: {}",
        system.count_accepted(
            [
                (b'x', 1..4001),
                (b'm', 1..4001),
                (b'a', 1..4001),
                (b's', 1..4001),
            ]
            .into_iter()
            .collect()
        )
    );
}
