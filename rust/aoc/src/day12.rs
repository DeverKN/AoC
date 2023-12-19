use core::num;
use std::fs;

struct Config {
  row: String,
  groups: Vec<usize>
}

pub(crate) fn main() -> () {
  let input = fs::read_to_string("/Users/deverkemmenash/Desktop/2023/AoC/rust/aoc/inputs/day_12.txt").unwrap();
  let res: usize = input.lines().map(|row| count_valid(row_to_config(row.to_string()))).sum();
  println!("res: {}", res)
}

fn row_to_config(row: String) -> Config {
  let mut iter = row.split(" ");
  let template = iter.next().expect("empty row");
  let groups = iter.next().expect("incomplete row");
  Config {
    row: template.to_string(),
    groups: groups.split(",").map(|c| c.parse::<usize>().expect("invalid groups")).collect()
  }
}

fn count_valid(config: Config) -> usize {
  let options = gen_row(&config);
  let valid: Vec<&String> = options.iter().filter(|cfg| check(cfg.to_string(), &config.groups)).collect();
  let num_valid = valid.iter().count();
  // println!("# Valid: {}", num_valid);
  num_valid
}

fn count_broken(row: &String) -> usize {
  row.chars().filter(|c| c == &'#').count().try_into().unwrap()
}

fn count_unknown(row: &String) -> usize {
  row.chars().filter(|c| c == &'?').count().try_into().unwrap()
}

fn get_missing(config: &Config) -> usize {
  let num_broken = count_broken(&config.row);
  let expected = config.groups.iter().sum::<usize>();
  // println!("broken: {}, expected: {}", num_broken, expected);
  return expected - num_broken;
}

fn substitute_template(row: &String, insertions: &[char]) -> String {
    if insertions.is_empty() {
      return row.to_owned();
    }
    let mut iter = row.chars();
    let first = iter.next().unwrap();
    let rest: String = iter.collect();
    match first {
      '?' => {
        insertions.get(0).unwrap().to_string() + &substitute_template(&rest, &insertions[1..])
      }
      _ => {
        first.to_string() + &substitute_template(&rest, insertions)
      }
    }
}

fn gen_row(config: &Config) -> Vec<String> {
  let subs = gen(count_unknown(&config.row), get_missing(&config));
  subs.iter().map(|insertions| substitute_template(&config.row, insertions)).collect()
}

fn gen(num_unknown: usize, num_missing: usize) -> Vec<Vec<char>> {
  if (num_unknown == 0 && num_missing > 0) {
    return vec![]
  } else if num_missing == 0 {
    return vec![vec!['.'; num_unknown]]
  } else if num_unknown == num_missing {
    return vec![vec!['#'; num_unknown]]
  } else {
    let mut broke = gen(num_unknown - 1, num_missing - 1);
    broke.iter_mut().for_each(|v| v.insert(0, '#'));
    let mut woke = gen(num_unknown - 1, num_missing);
    woke.iter_mut().for_each(|v| v.insert(0, '.'));
    broke.append(&mut woke);
    return broke;
  }
}

fn check(config: String,  groups: &Vec<usize>) -> bool {
  return &get_groups(config, 0) == groups
}

fn get_groups(config: String, running: usize) -> Vec<usize> {
  if config.is_empty() {
    if running > 0 {
      return Vec::from([running]);
    } else {
      return Vec::new();
    }
  }
  let mut iter = config.chars();
  let first = iter.next().unwrap();
  let rest: String = iter.collect();
  match first {
    '#' => {
      get_groups(rest, running + 1)
    }
    '.' => {
      let mut rest_vec = get_groups(rest, 0);
      if running > 0 {
        rest_vec.insert(0, running);
        rest_vec
      } else{
        rest_vec
      }
    }
    _ => {
      panic!();
    }
  }
  // todo!()
}