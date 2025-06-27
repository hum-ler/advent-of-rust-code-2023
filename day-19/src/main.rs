use std::{
    collections::HashMap,
    ops::{Deref, RangeInclusive},
    str::FromStr,
};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-19.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u64> {
    let (workflows, ratings) = parse_input_into_workflows_and_ratings(input)?;

    Ok(ratings
        .into_iter()
        .filter(|rating| accept_part(rating, &workflows))
        .map(|rating| rating.sum())
        .sum())
}

fn part_2(input: &str) -> Result<u64> {
    let (workflows, _) = parse_input_into_workflows_and_ratings(input)?;

    Ok(count_combinations(
        std::array::from_fn(|_| vec![1..=4000]), // defaults to all possible values
        &workflows["in"].rules,
        &workflows,
    ))
}

#[derive(Clone, Copy)]
struct Rating {
    categories: [u64; 4],
}

impl Deref for Rating {
    type Target = [u64];

    fn deref(&self) -> &Self::Target {
        &self.categories
    }
}

impl FromStr for Rating {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut categories = [0; 4];

        for token in s
            .strip_prefix("{")
            .and_then(|s| s.strip_suffix("}"))
            .map(|s| s.split_terminator(",").collect::<Vec<_>>())
            .ok_or(anyhow!("Cannot split input into value: {}", s))?
        {
            let Some((category, value)) = token.split_once("=") else {
                return Err(anyhow!(
                    "Cannot split input into category and value: {}",
                    token
                ));
            };
            let value = value.parse()?;

            match category {
                "x" => categories[0] = value,
                "m" => categories[1] = value,
                "a" => categories[2] = value,
                "s" => categories[3] = value,
                _ => return Err(anyhow!("Invalid category: {}", category)),
            }
        }

        Ok(Self { categories })
    }
}

impl Rating {
    fn sum(&self) -> u64 {
        self.categories.iter().sum()
    }
}

#[derive(Clone, Copy)]
enum Rule<'a> {
    Comparison {
        category: usize,
        operator: &'a str,
        operand: u64,
        outcome: &'a str,
    },
    Immediate(&'a str),
}

impl<'a> Rule<'a> {
    fn from_str(s: &'a str) -> Result<Self> {
        if !s.contains(":") {
            return Ok(Self::Immediate(s));
        }

        let Some((comparison, outcome)) = s.split_once(":") else {
            return Err(anyhow!(
                "Cannot split input into comparison and outcome: {}",
                s
            ));
        };

        let (category, comparison) = comparison.split_at(1);
        let category = match category {
            "x" => 0,
            "m" => 1,
            "a" => 2,
            "s" => 3,
            _ => return Err(anyhow!("Invalid category: {}", category)),
        };

        let (operator, operand) = comparison.split_at(1);
        if !"<>".contains(operator) {
            return Err(anyhow!("Invalid operator: {}", operator));
        }
        let operand = operand.parse()?;

        Ok(Self::Comparison {
            category,
            operator,
            operand,
            outcome,
        })
    }

    fn evaluate(&self, rating: &Rating) -> Option<&'a str> {
        match self {
            Rule::Comparison {
                category,
                operator,
                operand,
                outcome,
            } => match *operator {
                "<" => {
                    if &rating[*category] < operand {
                        Some(outcome)
                    } else {
                        None
                    }
                }
                ">" => {
                    if &rating[*category] > operand {
                        Some(outcome)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Rule::Immediate(outcome) => Some(outcome),
        }
    }
}

struct Workflow<'a> {
    id: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn from_str(s: &'a str) -> Result<Self> {
        let Some((id, rules)) = s.strip_suffix("}").and_then(|s| s.split_once("{")) else {
            return Err(anyhow!("Cannot split input into id and rules: {}", s));
        };

        let rules = rules
            .split_terminator(",")
            .map(Rule::from_str)
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { id, rules })
    }

    fn evaluate(&self, rating: &Rating) -> Result<&'a str> {
        self.rules
            .iter()
            .fold(None, |acc, rule| acc.or_else(|| rule.evaluate(rating)))
            .ok_or(anyhow!("Cannot get evaluation outcome"))
    }
}

fn parse_input_into_workflows_and_ratings(
    input: &str,
) -> Result<(HashMap<&str, Workflow>, Vec<Rating>)> {
    let Some((workflows, ratings)) = input.split_once("\n\n") else {
        return Err(anyhow!(
            "Cannot split input into workflows and ratings: {}",
            input
        ));
    };

    let workflows = workflows
        .lines()
        .map(|line| {
            let workflow = Workflow::from_str(line)?;

            Ok((workflow.id, workflow))
        })
        .collect::<Result<HashMap<_, _>>>()?;
    let ratings = ratings
        .lines()
        .map(Rating::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok((workflows, ratings))
}

fn accept_part(rating: &Rating, workflows: &HashMap<&str, Workflow>) -> bool {
    let mut workflow_id = "in";
    loop {
        match workflows[workflow_id].evaluate(rating) {
            Ok(outcome) => match outcome {
                "A" => return true,
                "R" => return false,
                _ => workflow_id = outcome,
            },
            Err(_) => return false,
        }
    }
}

/// [x_ranges, m_ranges, a_ranges, s_ranges]
type CategoryRanges = [Vec<RangeInclusive<u64>>; 4];

/// Counts the total number of combinations of category values that results in an accepted part.
fn count_combinations(
    mut category_ranges: CategoryRanges,
    rules: &[Rule],
    workflows: &HashMap<&str, Workflow>,
) -> u64 {
    // The recursion is always terminated by Rule::Immediate("A"), Rule::Immediate("R"), or a
    // Rule::Comparison with an outcome of "A" or "R".
    if rules.is_empty() {
        unreachable!();
    }

    match rules[0] {
        Rule::Immediate("R") => 0,
        Rule::Immediate("A") => calculate_combinations(category_ranges),
        Rule::Immediate(workflow_id) => {
            count_combinations(category_ranges, &workflows[workflow_id].rules, workflows)
        }
        Rule::Comparison {
            category,
            outcome: "R",
            ..
        } => {
            category_ranges[category].push(get_range_from_rule(&rules[0], false));
            count_combinations(category_ranges, &rules[1..], workflows)
        }
        Rule::Comparison {
            category,
            outcome: "A",
            ..
        } => {
            let mut success_category_ranges = category_ranges.clone();
            success_category_ranges[category].push(get_range_from_rule(&rules[0], true));
            let success_combinations = calculate_combinations(success_category_ranges);

            category_ranges[category].push(get_range_from_rule(&rules[0], false));
            success_combinations + count_combinations(category_ranges, &rules[1..], workflows)
        }
        Rule::Comparison {
            category, outcome, ..
        } => {
            let mut success_rating_ranges = category_ranges.clone();
            success_rating_ranges[category].push(get_range_from_rule(&rules[0], true));
            let success_combinations = count_combinations(
                success_rating_ranges.clone(),
                &workflows[outcome].rules,
                workflows,
            );

            category_ranges[category].push(get_range_from_rule(&rules[0], false));
            success_combinations + count_combinations(category_ranges, &rules[1..], workflows)
        }
    }
}

/// Derives the input range so that the given [Rule] evaluates to evaluates_to.
///
/// The category of the input is assumed to be the same as the category in [Rule::Comparison].
fn get_range_from_rule(rule: &Rule, evaluates_to: bool) -> RangeInclusive<u64> {
    match rule {
        Rule::Comparison {
            operator, operand, ..
        } => match (*operator, evaluates_to) {
            ("<", true) => 1..=*operand - 1,
            ("<", false) => *operand..=4000,
            (">", true) => *operand + 1..=4000,
            (">", false) => 1..=*operand,
            _ => 1..=4000,
        },
        Rule::Immediate(_) => 1..=4000,
    }
}

/// Calculates the number of combinations that is represented by the given [CategoryRanges].
fn calculate_combinations(mut rating_ranges: CategoryRanges) -> u64 {
    rating_ranges
        .iter_mut()
        .for_each(|category| *category = tighten_ranges(category));

    rating_ranges
        .into_iter()
        .map(|category| count_values_in_ranges(&category))
        .product()
}

/// Reduces (by intersecting) a set of ranges together into as few as possible.
fn tighten_ranges(ranges: &[RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    let mut ranges = Vec::from(ranges);
    ranges.sort_by_key(|range| *range.start());

    let mut snapshot = Vec::new();
    while snapshot != ranges && ranges.len() > 1 {
        snapshot = ranges.clone();

        ranges = ranges
            .windows(2)
            .flat_map(|window| intersection(&window[0], &window[1]))
            .collect();
    }

    ranges
}

/// Reduces 2 ranges into 1 if they intersect. If they do not intersect, both ranges are returned
/// intact and separate, in this order: range, other.
fn intersection(
    range: &RangeInclusive<u64>,
    other: &RangeInclusive<u64>,
) -> Vec<RangeInclusive<u64>> {
    match (range, other) {
        // <-- other -->
        //   <-- range -->
        (range, other) if other.contains(range.start()) && range.contains(other.end()) => {
            vec![(*range.start()..=*other.end())]
        }

        //   <-- other -->
        // <-- range -->
        (range, other) if range.contains(other.start()) && other.contains(range.end()) => {
            vec![(*other.start()..=*range.end())]
        }

        // <---- other ---->
        //   <-- range -->
        (range, other) if other.contains(range.start()) && other.contains(range.end()) => {
            vec![range.clone()]
        }

        //   <-- other -->
        // <---- range ---->
        (range, other) if range.contains(other.start()) && range.contains(other.end()) => {
            vec![other.clone()]
        }

        // <-- range --> <-- other -->  or  <-- other --> <-- range -->
        _ => vec![range.clone(), other.clone()],
    }
}

fn count_values_in_ranges(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
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

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 19114);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 167409079868000);

        Ok(())
    }
}
