use std::{cmp::Ordering, collections::HashMap};

pub fn run(input: &str) -> u64 {
    let (workflows, parts) = parse_input(input);

    let mut accepted = vec![];
    parts.iter().for_each(|part| {
        let mut outcome = "in";
        while outcome != "A" && outcome != "R" {
            outcome = workflows.get(&outcome).unwrap().apply(*part);
        }

        if outcome == "A" {
            accepted.push(part);
        }
    });

    accepted.into_iter().map(Part::sum).sum()
}

pub(crate) fn parse_input(input: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let (workflows_part, parts_part) = input.trim().split_once("\n\n").unwrap();

    let mut workflows = HashMap::new();
    workflows_part
        .lines()
        .map(str::trim)
        .map(Workflow::from)
        .for_each(|workflow| {
            workflows.insert(workflow.name, workflow);
        });

    let parts = parts_part.lines().map(str::trim).map(Part::from).collect();

    (workflows, parts)
}

pub(crate) struct Workflow<'a> {
    name: &'a str,
    pub(crate) rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn apply(&self, part: Part) -> &str {
        for rule in &self.rules {
            if let Some(output) = rule.apply(part) {
                return output;
            }
        }

        unreachable!()
    }
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(value: &'a str) -> Self {
        let (name, rules_part) = value.split_once('{').unwrap();

        let rules = rules_part[..rules_part.len() - 1]
            .split(',')
            .map(Rule::from)
            .collect::<Vec<Rule>>();

        Self { name, rules }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) struct Rule<'a> {
    pub(crate) cmp: Option<Ordering>,
    pub(crate) property: Option<&'a str>,
    pub(crate) value: Option<u64>,
    pub(crate) outcome: &'a str,
}

impl<'a> Rule<'a> {
    fn apply(&self, part: Part) -> Option<&str> {
        match (self.cmp, self.property, self.value) {
            (None, _, _) => return Some(self.outcome),
            (Some(cmp), Some("x"), Some(value)) => {
                if part.x.cmp(&value) == cmp {
                    return Some(self.outcome);
                }
            }
            (Some(cmp), Some("m"), Some(value)) => {
                if part.m.cmp(&value) == cmp {
                    return Some(self.outcome);
                }
            }
            (Some(cmp), Some("a"), Some(value)) => {
                if part.a.cmp(&value) == cmp {
                    return Some(self.outcome);
                }
            }
            (Some(cmp), Some("s"), Some(value)) => {
                if part.s.cmp(&value) == cmp {
                    return Some(self.outcome);
                }
            }
            _ => unreachable!(),
        }

        None
    }
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(value: &'a str) -> Self {
        if !value.contains(':') {
            return Self {
                cmp: None,
                property: None,
                value: None,
                outcome: value,
            };
        };

        let (condition_part, outcome) = value.split_once(':').unwrap();
        let (property, value_part) = condition_part.split_once(['<', '>']).unwrap();
        let value = value_part.parse::<u64>().unwrap();
        let cmp = if condition_part.contains('<') {
            Ordering::Less
        } else {
            Ordering::Greater
        };

        Self {
            cmp: Some(cmp),
            property: Some(property),
            value: Some(value),
            outcome,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let mut new = Self::default();

        value[1..value.len() - 1].split(',').for_each(|property| {
            let (name_part, value_part) = property.split_once('=').unwrap();
            let value = value_part.parse::<u64>().unwrap();

            match name_part {
                "x" => new.x = value,
                "m" => new.m = value,
                "a" => new.a = value,
                "s" => new.s = value,
                _ => unreachable!(),
            }
        });

        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_example() {
        let input = r"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        ";

        assert_eq!(run(input), 19114);
    }
}
