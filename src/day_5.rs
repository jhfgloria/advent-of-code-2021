use std::str::Lines;
use std::collections::HashMap;
use regex::Regex;
#[path="./problem.rs"] mod problem;

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Point {
  x: i16,
  y: i16,
}

#[derive(Debug, Clone, Copy)]
struct Direction {
  x: i16,
  y: i16,
}

#[derive(Debug, Clone, Copy)]
struct Line {
  p1: Point,
  p2: Point,
  direction: Direction,
}

pub fn run_problem_1() {
  match problem::get_problem(5) {
    Ok(input) => {
      let mut board = HashMap::<Point, i32>::new();

      let lines_of_vent = parse_lines(input.lines());
      lines_of_vent.iter().filter(|line| line.p1.x == line.p2.x || line.p1.y == line.p2.y )
                          .for_each(|line| { draw_board(&mut board, *line); });

      let overlaps = board.iter().fold(0, |count, next_point| {
        match *next_point.1 > 1 {
          true => count + 1,
          false => count,
        }
      });

      println!("{}", overlaps);
    },
    Err(_) => println!("Something went wrong"),
  }
}

pub fn run_problem_2() {
  match problem::get_problem(5) {
    Ok(input) => {
      let mut board = HashMap::<Point, i32>::new();

      let lines_of_vent = parse_lines(input.lines());
      lines_of_vent.iter().for_each(|line| { draw_board(&mut board, *line); });

      let overlaps = board.iter().fold(0, |count, next_point| {
        match *next_point.1 > 1 {
          true => count + 1,
          false => count,
        }
      });

      println!("{}", overlaps);
    },
    Err(_) => println!("Something went wrong"),
  }
}

fn draw_board(board: &mut HashMap::<Point, i32>, line: Line) -> () {
  let mut next_point: Point = line.p1;

  loop {
    if let Some(x) = board.get_mut(&next_point) {
      *x +=1;
    } else {
      board.insert(next_point, 1);
    }

    if next_point == line.p2 { break; }

    next_point = build_point(next_point.x + line.direction.x, next_point.y + line.direction.y);
  };
}

fn parse_lines(lines: Lines) -> Vec<Line> {
  let reg = Regex::new(r"([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)").unwrap();

  lines.map(|line| {
    let capture = reg.captures(line).unwrap();
    let x1 = capture.get(1).map_or("", |m| m.as_str()).parse::<i16>().unwrap();
    let y1 = capture.get(2).map_or("", |m| m.as_str()).parse::<i16>().unwrap();
    let x2 = capture.get(3).map_or("", |m| m.as_str()).parse::<i16>().unwrap();
    let y2 = capture.get(4).map_or("", |m| m.as_str()).parse::<i16>().unwrap();

    build_line(build_point(x1, y1), build_point(x2, y2))
  }).collect()
}

fn build_line(point_1: Point, point_2: Point) -> Line {
  let direction_x = match point_2.x - point_1.x {
    delta if delta > 0 => 1,
    delta if delta < 0 => -1,
    _ => 0,
  };

  let direction_y = match point_2.y - point_1.y {
    delta if delta > 0 => 1,
    delta if delta < 0 => -1,
    _ => 0,
  };

  Line {
    p1: point_1,
    p2: point_2,
    direction: Direction { x: direction_x, y: direction_y},
  }
}

fn build_point(x: i16, y: i16) -> Point {
  Point { x, y }
}
