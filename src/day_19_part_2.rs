// Strategy:
// 1. Construct a decision tree of 2 children (T, F) each node. Expand the tree until all the leaf
//    nodes are either A or R.
//    ![illustration](docs/day-19-part-2.png)
// 2. For each A leaf node, work upwards through all the parents to collect (predicates, T/F).
//    Combine and deduce the minimal set of predicates i.e. the ranges (counts) that x, m, a, s can
//    take, and then multiply them together.
// 3. Sum the results from 2.
//
// We have to assume that the input is well-behaved and there are no cycles.

use std::{cmp::Ordering, collections::HashMap};

use tree_iterators_rs::prelude::*;

use crate::day_19_part_1::*;

pub fn run(input: &str) -> u64 {
    let (workflows, _) = parse_input(input);

    let tree = workflow_to_tree_node("in", &vec![], &workflows).unwrap();

    tree.bfs()
        .filter_map(|node| {
            if let NodeValue::Accept(path) = node {
                Some(path)
            } else {
                None
            }
        })
        .map(|predicates| reduce_predicates(&predicates))
        .sum()
}

/// Reduces a list of predicates to the number of possible combinations.
fn reduce_predicates(predicates: &[Predicate]) -> u64 {
    let x = sieve(
        &predicates
            .iter()
            .filter(|predicate| predicate.0.property == Some("x"))
            .map(|predicate| normalize_predicate(*predicate))
            .collect::<Vec<Rule>>(),
    );
    let m = sieve(
        &predicates
            .iter()
            .filter(|predicate| predicate.0.property == Some("m"))
            .map(|predicate| normalize_predicate(*predicate))
            .collect::<Vec<Rule>>(),
    );
    let a = sieve(
        &predicates
            .iter()
            .filter(|predicate| predicate.0.property == Some("a"))
            .map(|predicate| normalize_predicate(*predicate))
            .collect::<Vec<Rule>>(),
    );
    let s = sieve(
        &predicates
            .iter()
            .filter(|predicate| predicate.0.property == Some("s"))
            .map(|predicate| normalize_predicate(*predicate))
            .collect::<Vec<Rule>>(),
    );

    x * m * a * s
}

fn sieve(rules: &[Rule]) -> u64 {
    let mut min = 1;
    let mut max = 4000;

    rules.iter().for_each(|rule| {
        let value = rule.value.unwrap();

        match rule.cmp {
            Some(Ordering::Less) => {
                if value - 1 < max {
                    max = value - 1;
                }
            }
            Some(Ordering::Greater) => {
                if value + 1 > min {
                    min = value + 1;
                }
            }
            _ => unreachable!(),
        }
    });

    max - min + 1
}

/// Unwraps a Predicate into a Rule, taking into account whether it is true or false.
fn normalize_predicate(predicate: Predicate) -> Rule {
    match predicate.1 {
        true => predicate.0,
        false => flip_rule(&predicate.0),
    }
}

fn flip_rule<'a>(rule: &Rule<'a>) -> Rule<'a> {
    let value = rule.value.unwrap();

    match rule.cmp {
        Some(Ordering::Less) => Rule {
            cmp: Some(Ordering::Greater),
            property: rule.property,
            value: Some(value - 1),
            outcome: rule.outcome,
        },
        Some(Ordering::Greater) => Rule {
            cmp: Some(Ordering::Less),
            property: rule.property,
            value: Some(value + 1),
            outcome: rule.outcome,
        },
        _ => unreachable!(),
    }
}

/// Generates a decision tree from a starting workflow, recursively.
fn workflow_to_tree_node<'a>(
    name: &'a str,
    path: &Vec<Predicate<'a>>,
    workflows: &HashMap<&'a str, Workflow<'a>>,
) -> Option<Box<BinaryTreeNode<NodeValue<'a>>>> {
    // Terminate when we hit A or R.
    match name {
        "A" => {
            return Some(Box::new(BinaryTreeNode {
                value: NodeValue::Accept(path.clone()),
                left: None,
                right: None,
            }));
        }
        "R" => {
            return Some(Box::new(BinaryTreeNode {
                value: NodeValue::Reject,
                left: None,
                right: None,
            }));
        }
        _ => (),
    }

    rules_to_tree_node(&workflows.get(name).unwrap().rules, path, workflows)
}

/// Generates a decision tree from a list of rules, recursively.
fn rules_to_tree_node<'a>(
    rules: &[Rule<'a>],
    path: &Vec<Predicate<'a>>,
    workflows: &HashMap<&'a str, Workflow<'a>>,
) -> Option<Box<BinaryTreeNode<NodeValue<'a>>>> {
    // Terminate when we encounter a rule with no predicate i.e. the last rule of a workflow.
    if rules[0].cmp.is_none() {
        return Some(Box::new(BinaryTreeNode {
            value: NodeValue::Rule(path.clone()),
            left: workflow_to_tree_node(rules[0].outcome, path, workflows),
            right: None,
        }));
    }

    let mut true_path = path.clone();
    true_path.push((rules[0], true));
    let mut false_path = path.clone();
    false_path.push((rules[0], false));

    Some(Box::new(BinaryTreeNode {
        value: NodeValue::Rule(path.clone()),
        left: workflow_to_tree_node(rules[0].outcome, &true_path, workflows),
        right: rules_to_tree_node(&rules[1..], &false_path, workflows),
    }))
}

#[derive(PartialEq)]
enum NodeValue<'a> {
    Rule(Vec<(Rule<'a>, bool)>),
    Accept(Vec<(Rule<'a>, bool)>),
    Reject,
}

type Predicate<'a> = (Rule<'a>, bool);

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

        assert_eq!(run(input), 167409079868000);
    }
}
