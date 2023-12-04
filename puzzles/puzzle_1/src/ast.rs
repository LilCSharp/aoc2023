use std::collections::HashMap;

#[derive(Debug)]
pub struct ASTNode {
  state: char,
  value: u8,
  next: Vec<Box<ASTNode>>,
}

impl ASTNode {
  fn new(state: char, value: u8) -> Self {
    ASTNode {
      state,
      value,
      next: Vec::new(),
    }
  }
}

pub fn construct_ast() -> HashMap<char, Box<ASTNode>> {
  let numbers = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
  let mut ast_tree = HashMap::new();
  
  for (index, number) in numbers.iter().enumerate() {
    let mut current_node = ast_tree.entry(
      number.chars().next().unwrap()
    )
    .or_insert_with(
      || {
        Box::new(ASTNode::new(number.chars().next().unwrap(), 0))
      }
    );

    for c in number.chars().skip(1) {
      let child_exists = current_node.next.iter().any(|node| node.state == c);

      if !child_exists {
        let new_node = Box::new(ASTNode::new(c, 0));
        current_node.next.push(new_node);
      }

    current_node = current_node
      .next
      .iter_mut()
      .find(|node| node.state == c)
      .unwrap();
    }

    let end_state = Box::new(ASTNode::new('\0', index.try_into().unwrap()));
    current_node.next.push(end_state);
  }

  ast_tree
}

fn get_node<'a>(
  current_node: &mut Option<&'a ASTNode>, 
  c: char, first_num: &mut Option<u8>, 
  second_num: &mut Option<u8>, 
  ast_tree: &'a HashMap<char, Box<ASTNode>>
) {
  match current_node {
    None => {
      *current_node = ast_tree.get(&c).map(|node| &**node);
    },
    Some(node) => {
      if let Some(next_node) = node.next.iter().find(|n| n.state == '\0') {
        match first_num {
          None => {
            *first_num = Some(next_node.value);
            *second_num = *first_num;
          },
          Some(_) => {
            *second_num = Some(next_node.value);
          }
        }

        *current_node = ast_tree.get(&c).map(|node| &**node);
      } else if let Some(next_node) = node.next.iter().find(|n| n.state == c) {
        *current_node = Some(&**next_node);
      } else {
        *current_node = ast_tree.get(&c).map(|node| &**node);
      }
    }
  }
}

pub fn parse_string(input: &String, ast_tree: &HashMap<char, Box<ASTNode>>) -> u32 {
  let mut first_num: Option<u8> = None;
  let mut second_num: Option<u8> = None;
  let mut current_node: Option<&ASTNode> = None;

  for c in input.chars() {
    get_node(&mut current_node, c, &mut first_num, &mut second_num, &ast_tree);

    if c.is_numeric() {
      match first_num {
        None => {
          first_num = Some(c.to_digit(10).unwrap() as u8);
          second_num = first_num;
        },
        Some(_) => {
          second_num = Some(c.to_digit(10).unwrap() as u8);
        }
      }
    }
  }

  get_node(&mut current_node, '\0', &mut first_num, &mut second_num, &ast_tree);

  let value: u32 = (first_num.unwrap_or(0) * 10 + second_num.unwrap_or(0)).into();

  value
}