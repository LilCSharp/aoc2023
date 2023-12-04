mod ast;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use ast::{construct_ast, parse_string};

fn main() {
  let mut sum_question_1 = 0;
  let mut sum_question_2 = 0;
  let ast_tree = construct_ast();

  if let Ok(lines) = read_lines("src/data/data.txt") {
    for line in lines {
      if let Ok(msg) = line {
        sum_question_1 += construct_code(&msg);
        sum_question_2 += parse_string(&msg, &ast_tree);
      }
    }
  }

  println!("Question 1: {}", sum_question_1);
  println!("Question 2: {}", sum_question_2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn construct_code(s: &String) -> u32 {
  let mut first: u32 = 0;
  let mut second: u32 = 0;
  let mut unhit: bool = true;

  for c in s.chars() {
    if c.is_numeric() {
      second = c.to_digit(10).unwrap();

      if unhit {
        first = c.to_digit(10).unwrap();
        unhit = !unhit;
      }
    }
  }

  10 * first + second
}