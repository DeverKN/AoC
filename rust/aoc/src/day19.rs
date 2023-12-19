use regex::Regex;
use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

type WorkflowName = String;

#[derive(Debug)]
enum WorkFlowResult {
    Accepted,
    Rejected,
}

#[derive(Debug)]
enum Action {
    Accept,
    Reject,
    Switch(WorkflowName),
}

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Comparison {
    LessThan,
    GreaterThan,
}

#[derive(Debug)]
struct Toy {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug, Default, Clone)]
struct BoundedToy {
    x: Bounded,
    m: Bounded,
    a: Bounded,
    s: Bounded,
}

#[derive(Debug)]
struct Rule {
    category: Category,
    comparison: Comparison,
    val: u64,
}

#[derive(Debug)]
struct WorkFlow {
    rules: Vec<(Rule, Action)>,
    default: Action,
}

#[derive(Debug, Clone)]
struct Bounded {
    min: u64,
    max: u64,
}

impl Default for Bounded {
    fn default() -> Self {
        Bounded { min: 1, max: 4000 }
    }
}

fn is_feasible(val: &Bounded, constraint: &Comparison, constraint_val: u64) -> bool {
    match (constraint, val.min, val.max) {
        (Comparison::GreaterThan, _, max_val) => constraint_val <= max_val,
        (Comparison::LessThan, min_val, _) => constraint_val >= min_val,
    }
}

fn assert_bounded_invariant(val: Bounded) -> Option<Bounded> {
    // val.min.map(|min| val.max.map(|max| assert!(min <= max)));
    if val.min <= val.max {
        Some(val)
    } else {
        None
    }
}

fn constrain_inverse(
    val: &Bounded,
    constraint: &Comparison,
    constraint_val: u64,
) -> Option<Bounded> {
    match constraint {
        Comparison::GreaterThan => constrain(val, &Comparison::LessThan, constraint_val + 1),
        Comparison::LessThan => constrain(val, &Comparison::GreaterThan, constraint_val - 1),
    }
}

fn constrain(val: &Bounded, constraint: &Comparison, constraint_val: u64) -> Option<Bounded> {
    if !is_feasible(&val, &constraint, constraint_val) {
        return None;
    } else {
        return match constraint {
            Comparison::GreaterThan => {
                let new_min = max(val.min, constraint_val + 1);
                let new_val = Bounded {
                    min: new_min,
                    max: val.max,
                };
                assert_bounded_invariant(new_val)
            }
            Comparison::LessThan => {
                let new_max = min(val.max, constraint_val - 1);
                let new_val = Bounded {
                    min: val.min,
                    max: new_max,
                };
                assert_bounded_invariant(new_val)
            }
        };
    }
    /*match constraint {
        Comparison::GreaterThan => {
            if constraint_val > val.max {
                None
            } else {
                Some(Bounded {
                    min: max(val.min, constraint_val),
                    max: val.max,
                })
            }
        }
        Comparison::LessThan => {
          if constraint_val < val.min {
              None
          } else {
              Some(Bounded {
                  min: max(val.min, constraint_val),
                  max: val.max,
              })
          }
      }
    }*/
}

fn get(toy: &Toy, category: &Category) -> u64 {
    match category {
        Category::X => toy.x,
        Category::M => toy.m,
        Category::A => toy.a,
        Category::S => toy.s,
    }
}
fn match_rule(rule: &Rule, toy: &Toy) -> bool {
    let Rule {
        category,
        comparison,
        val,
    } = rule;
    match comparison {
        Comparison::LessThan => get(toy, category) < *val,
        Comparison::GreaterThan => get(toy, category) > *val,
    }
}

fn handle_workflow(
    workflows: &HashMap<WorkflowName, WorkFlow>,
    workflow: &WorkFlow,
    toy: &Toy,
) -> WorkFlowResult {
    let WorkFlow { rules, default } = workflow;
    let rule = rules
        .iter()
        .find(|(rule, _)| match_rule(rule, &toy))
        .map(|(_, action)| action);
    let action = rule.unwrap_or(&default);
    match action {
        Action::Accept => WorkFlowResult::Accepted,
        Action::Reject => WorkFlowResult::Rejected,
        Action::Switch(workflow_name) => {
            let new_workflow = workflows.get(workflow_name).expect("unknown workflow name");
            handle_workflow(workflows, new_workflow, toy)
        }
    }
}

fn sum(toy: &Toy) -> u64 {
    toy.x + toy.m + toy.a + toy.s
}

fn handle_toy(workflows: &HashMap<WorkflowName, WorkFlow>, toy: Toy) -> u64 {
    let in_wf = workflows.get("in").expect("missing in workflow");
    let res = handle_workflow(workflows, in_wf, &toy);
    // println!("Result for {:?}, {:?}", toy, res);
    match res {
        WorkFlowResult::Accepted => sum(&toy),
        WorkFlowResult::Rejected => 0,
    }
}

fn parse_category(cat: &str) -> Category {
    match cat {
        "x" => Category::X,
        "m" => Category::M,
        "a" => Category::A,
        "s" => Category::S,
        _ => panic!("unknown category"),
    }
}

// px{a<2006:qkq,m>2090:A,rfg}
fn parse_rule(rule: &str) -> (Rule, Action) {
    let re =
        Regex::new(r"(?<category>[xmas]+)(?<comparison>[<>])(?<val>[0-9]+):(?<action>[a-z]+|[AR])")
            .unwrap();
    let caps = re.captures(rule).unwrap();
    let category = parse_category(&caps["category"]);
    let val = caps["val"].parse::<u64>().expect("invalid value");
    let comparison = if caps["comparison"].to_string() == ">" {
        Comparison::GreaterThan
    } else {
        Comparison::LessThan
    };
    let action = parse_action(&caps["action"]);
    return (
        Rule {
            category,
            val,
            comparison,
        },
        action,
    );
}

fn parse_action(action: &str) -> Action {
    match action {
        "A" => Action::Accept,
        "R" => Action::Reject,
        workflow_name => Action::Switch(workflow_name.to_string()),
    }
}

fn parse_workflow(workflow: &str) -> (WorkflowName, WorkFlow) {
    let re = Regex::new(
        r"(?<name>[a-z]+)\{(?<rules>(?<rule>([xmas][><][0-9]+):([RA]|[a-z]+),)+([RA]|[a-z]+))\}",
    )
    .unwrap();
    let caps = re.captures(&workflow).unwrap();
    let name = caps["name"].to_string();
    let split_rules: Vec<&str> = caps["rules"].split(',').collect();
    let rules: Vec<(Rule, Action)> = split_rules[0..split_rules.len() - 1]
        .iter()
        .map(|rule| parse_rule(rule))
        .collect();
    let default = parse_action(split_rules.last().expect("no default rule"));
    let workflow = WorkFlow { rules, default };
    return (name, workflow);
}

fn parse_toy(toy: &str) -> Toy {
    let re =
        Regex::new(r"(\{x=(?<x>[0-9]+),m=(?<m>[0-9]+),a=(?<a>[0-9]+),s=(?<s>[0-9]+)\})").unwrap();
    let caps = re.captures(toy).unwrap();
    let x = caps["x"].parse::<u64>().expect("invalid x value");
    let m = caps["m"].parse::<u64>().expect("invalid m value");
    let a = caps["a"].parse::<u64>().expect("invalid a value");
    let s = caps["s"].parse::<u64>().expect("invalid s value");
    return Toy { x, m, a, s };
}

fn get_feasible_options(val: &Bounded) -> u64 {
    return (val.max - val.min) + 1;
    // match (val.min, val.max) {
    //   (Some(min), Some(max)) => {
    //     Some((max - min) + 1)
    //   }
    //   _ => None
    // }
}

fn get_toy_options(val: &BoundedToy) -> u64 {
    let x = get_feasible_options(&val.x);
    let m = get_feasible_options(&val.m);
    let a = get_feasible_options(&val.a);
    let s = get_feasible_options(&val.s);
    return x * m * a * s;
}

fn constrain_toy(
    toy: &BoundedToy,
    category: &Category,
    constraint: &Comparison,
    constraint_val: u64,
) -> Option<BoundedToy> {
    match category {
        Category::X => {
            let BoundedToy { x, m, a, s } = toy.clone();
            constrain(&x, constraint, constraint_val).map(|x| BoundedToy { x, m, a, s })
        }
        Category::M => {
            let BoundedToy { x, m, a, s } = toy.clone();
            constrain(&m, constraint, constraint_val).map(|m| BoundedToy { x, m, a, s })
        }
        Category::A => {
            let BoundedToy { x, m, a, s } = toy.clone();
            let new_a = constrain(&a, constraint, constraint_val);
            new_a.map(|a| BoundedToy { x, m, a, s })
        }
        Category::S => {
            let BoundedToy { x, m, a, s } = toy.clone();
            constrain(&s, constraint, constraint_val).map(|s| BoundedToy { x, m, a, s })
        }
    }
}

fn constrain_toy_inverse(
    toy: &BoundedToy,
    category: &Category,
    constraint: &Comparison,
    constraint_val: u64,
) -> Option<BoundedToy> {
    match category {
        Category::X => {
            let BoundedToy { x, m, a, s } = toy.clone();
            constrain_inverse(&x, constraint, constraint_val).map(|x| BoundedToy { x, m, a, s })
        }
        Category::M => {
            let BoundedToy { x, m, a, s } = toy.clone();
            constrain_inverse(&m, constraint, constraint_val).map(|m| BoundedToy { x, m, a, s })
        }
        Category::A => {
            let BoundedToy { x, m, a, s } = toy.clone();
            let new_a = constrain_inverse(&a, constraint, constraint_val);
            new_a.map(|a| BoundedToy { x, m, a, s })
        }
        Category::S => {
            let BoundedToy { x, m, a, s } = toy.clone();
            constrain_inverse(&s, constraint, constraint_val).map(|s| BoundedToy { x, m, a, s })
        }
    }
}

fn solve_rule(
    old_toy: &BoundedToy,
    workflows: &HashMap<WorkflowName, WorkFlow>,
    rule: &Rule,
    action: &Action,
) -> u64 {
    if let Some(toy) = constrain_toy(old_toy, &rule.category, &rule.comparison, rule.val) {
        match action {
            Action::Accept => {
                println!("accepted {:?}", toy);
                get_toy_options(&toy)
            }
            Action::Reject => 0,
            Action::Switch(new_wf) => solve(&toy, workflows, workflows.get(new_wf).unwrap()),
        }
    } else {
        0
    }
}

fn solve(
    toy: &BoundedToy,
    workflows: &HashMap<WorkflowName, WorkFlow>,
    workflow: &WorkFlow,
) -> u64 {
    let (default_toy, rules_options) =
        workflow
            .rules
            .iter()
            .fold((Some(toy.clone()), 0), |(toy, acc), (rule, action)| {
                // solve_rule(&toy, &workflows, rule, action)
                if let Some(toy) = toy {
                    let new_acc = acc + solve_rule(&toy, &workflows, rule, action);
                    let new_toy =
                        constrain_toy_inverse(&toy, &rule.category, &rule.comparison, rule.val);
                    return (new_toy, new_acc);
                } else {
                    return (toy, acc);
                }
            });
    if let Some(toy) = default_toy {
      let default_options = match &workflow.default {
        Action::Accept => {
            println!("accepted (default) {:?}", toy);
            get_toy_options(&toy)
        }
        Action::Reject => 0,
        Action::Switch(new_wf) => solve(&toy, workflows, workflows.get(new_wf).unwrap()),
    };
    return default_options + rules_options;
    } else {
      return rules_options;
    }
    // let mut rules_options = 0;
    // let mut rules_toy = Some(toy.clone());
    // for (rule, action) in workflow.rules[0..] {
    //   rules_options += solve_rule(&toy, &workflows, &rule, &action);
    //   rules_toy = constrain_toy_inverse(&toy, &rule.category, &rule.comparison, rule.val)
    // }
    
    // todo!()
}

pub(crate) fn main() {
    let input =
        fs::read_to_string("/Users/deverkemmenash/Desktop/2023/AoC/rust/aoc/inputs/day_19.txt")
            .unwrap();
    let lines = input.lines();
    let mut workflows = HashMap::<WorkflowName, WorkFlow>::new();
    lines
        .take_while(|line| !line.is_empty())
        .map(|line| parse_workflow(line))
        .for_each(|(name, wf)| {
            // println!("Adding workflow {}: {:?}", name, wf);
            workflows.insert(name, wf);
        });
    let res = solve(
        &BoundedToy::default(),
        &workflows,
        workflows.get("in").unwrap(),
    );
    println!("Options {}", res)
    // let total: u64 = lines
    //     .skip_while(|line| !line.is_empty())
    //     .skip(1)
    //     .map(|toy| parse_toy(toy))
    //     .map(|toy| handle_toy(&workflows, toy))
    //     .sum();
    // println!("total: {}", total);
    // let val = Bounded::default();
    // let val2 = constrain(&val, &Comparison::GreaterThan, 10).unwrap();
    // let val3 = constrain(&val2, &Comparison::LessThan, 100).unwrap();
    // let val4 = constrain(&val3, &Comparison::GreaterThan, 100).unwrap();
    // let feasible_options = get_feasible_options(val3);
    // println!("possible options {:?}", feasible_options);
    // let feasible_options = get_feasible_options(val4);
    // println!("possible options {:?}", feasible_options);
    // let (wf_name, wf) = parse_workflow("px{a<2006:qkq,m>2090:A,rfg}".to_string());
    // let toy = parse_toy("{x=787,m=2655,a=1222,s=2876}");
    // println!("{:?}", wf);
    // println!("{:?}", toy);
    // let mut workflows = HashMap::<WorkflowName, WorkFlow>::new();
    // workflows.insert(wf_name, wf);
    // let res = handle_toy(&workflows, toy);
    // println!("{:?}", res);
}
