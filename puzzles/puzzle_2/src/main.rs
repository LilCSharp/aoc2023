use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn retrieve_id(s: &String) -> u32 {
  let regex_str = r"Game ([\d]+)";
  let regex = Regex::new(regex_str).expect("Failed to create regex");
  let id = regex.captures(s).unwrap().get(1).unwrap().as_str();

  id.parse().unwrap()
}

fn parse_line(input: &String) -> (u32, u32) {
  let id = retrieve_id(&input);

  let regex_str = r"(?:(?: ([\d]+) (blue|green|red)),?)(?: ([\d]+) (blue|green|red),?)?(?: ([\d]+) (blue|green|red),?)?(?:\;|$)";
  let regex = Regex::new(regex_str).expect("Failed to create regex");

  let mut num: u32 = 0;
  let mut valid = true;
  let mut color_maxes: [u32; 3] = [0, 0, 0]; 

  for capture in regex.captures_iter(input) {
    for group in 1..capture.len() {
      match capture.get(group) {
        None => {},
        Some(matched) => {
          let value = matched.as_str();

          if group % 2 == 1 {
            num = value.parse::<u32>().unwrap();
          } else {
            let result = match value {
              "red" => {
                if num > color_maxes[0] {
                  color_maxes[0] = num;
                }

                num <= 12
              },
              "green" => {
                if num > color_maxes[1] {
                  color_maxes[1] = num;
                }

                num <= 13
              },
              "blue" => {
                if num > color_maxes[2] {
                  color_maxes[2] = num;
                }

                num <= 14
              },
              _ => { true },
            };

            valid = valid && result;
          }
        }
      }
    }
  }

  let power = color_maxes[0] * color_maxes[1] * color_maxes[2];

  if valid {
    (id, power)
  } else {
    (0, power)
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn main() {
  let mut sum: u32 = 0;
  let mut net_power: u32 = 0;
  
  if let Ok(lines) = read_lines("src/data/data.txt") {
    for line in lines {
      if let Ok(input) = line {
        let (value, power) = parse_line(&input);
        sum += value;
        net_power += power;
      }
    }
  }

  println!("ID Sum: {}", sum);
  println!("Net Power: {}", net_power);
}
