use std::str::Lines;
#[path = "./problem.rs"] mod problem;

pub fn run_day_1_1() {
  match problem::get_problem(1) {
    Ok(input) => {
      let measurements = parse_measuerements(input.lines());
      let increase_count = find_increase_count(measurements);

      println!("{}", increase_count)
    },
    Err(_) => println!("Something went wrong"),
  }
}

pub fn run_day_1_2() {
  match problem::get_problem(1) {
    Ok(input) => {
      let measurements = parse_measuerements(input.lines());
      let sliding_measurements = three_measuresment_sliding_window(measurements);
      let increase_count = find_increase_count(sliding_measurements);

      println!("{}", increase_count)
    },
    Err(_) => println!("Something went wrong"),
  }
}

fn parse_measuerements(lines: Lines) -> Vec<i32> {
  lines.map(|line| line.parse().unwrap()).collect()
}

fn three_measuresment_sliding_window(measurements: Vec<i32>) -> Vec<i32> {
  let mut new_measurements: Vec<i32> = Vec::new();

  for i in 0..measurements.len() - 2 {
    let sum: i32 = measurements[i] + measurements[i+1] + measurements[i+2];
    new_measurements.push(sum);
  }

  new_measurements
}

fn find_increase_count(measurements: Vec<i32>) -> i32 {
  let mut last_item: Option<i32> = None;
  let mut increase_count = 0;

  for measurement in measurements {
    match last_item {
      Some(last_measurement) if measurement > last_measurement => {
        increase_count = increase_count + 1;
      },
      _ => (),
    };
    last_item = Some(measurement);
  };

  increase_count
}
