use std::str::Lines;
use regex::Regex;
use std::cmp::max;

#[path = "./problem.rs"] mod problem;

#[derive(Debug)]
enum Instruction {
  Forward(i32),
  Up(i32),
  Down(i32),
  Idle,
}

pub fn run_day_2_1() {
  match problem::get_problem(2) {
    Ok(input) => {
      let lines = input.lines();
      let instructions = parse_instructions(lines);

      let final_coordinates: (i32, i32) = instructions.iter().fold((0, 0), |coordinates, next_instruction| {
        process_instructions(coordinates, next_instruction, None)
      });

      println!("{:#?}", final_coordinates.0 * final_coordinates.1)
    },
    Err(_) => println!("Something went wrong"),
  }
}

pub fn run_day_2_2() {
  match problem::get_problem(2) {
    Ok(input) => {
      let lines = input.lines();
      let instructions = parse_instructions(lines);
      let mut aim = 0;

      let final_coordinates: (i32, i32) = instructions.iter().fold((0, 0), |coordinates, next_instruction| {
        aim = process_aim(aim, next_instruction);
        process_instructions(coordinates, next_instruction, Some(aim))
      });

      println!("{:#?}", final_coordinates.0 * final_coordinates.1)
    },
    Err(_) => println!("Something went wrong"),
  }
}

fn process_aim(aim: i32, next_instruction: &Instruction) -> i32 {
  match next_instruction {
    Instruction::Up(value) => max(0, aim - value),
    Instruction::Down(value) => aim + value,
    _ => aim,
  }
}

fn process_instructions(mut coordinates: (i32, i32), next_instruction: &Instruction, aim: Option<i32>) -> (i32, i32) {
  match next_instruction {
    Instruction::Forward(value) => {
      coordinates.0 = coordinates.0 + value;
      if aim.is_some() { coordinates.1 = coordinates.1 + (value * aim.unwrap()) };
    },
    Instruction::Up(value) if aim.is_none() => coordinates.1 = max(0, coordinates.1 - value),
    Instruction::Down(value) if aim.is_none() => coordinates.1 = coordinates.1 + value,
    _ => (),
  };

  return coordinates
}

fn parse_instructions(lines: Lines) -> Vec<Instruction> {
  let reg = Regex::new(r"([a-z]+) ([0-9]+)").unwrap();

  let parse = |line| {
    let capture = reg.captures(line).unwrap();
    let left_side = capture.get(1).map_or("", |m| m.as_str());
    let right_side = capture.get(2).map_or("", |m| m.as_str());

    match left_side {
      "forward" => Instruction::Forward(right_side.parse().unwrap()),
      "up" => Instruction::Up(right_side.parse().unwrap()),
      "down" => Instruction::Down(right_side.parse().unwrap()),
      _ => Instruction::Idle,
    }
  };

  lines.map(parse).collect()
}
