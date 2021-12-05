use std::cmp::min;

#[path = "./problem.rs"] mod problem;

pub fn run_problem_1() {
  match problem::get_problem(3) {
    Ok(input) => {
      let lines = input.lines().collect::<Vec<&str>>();
      let problem_size = lines.len() as i32;
      let mut counter = [0; 12];

      lines.iter().for_each(|binary| { update_counter(binary, &mut counter); });
      
      let gamma_rate = counter_to_binary(&counter, problem_size, '1', '0');
      let epsilon_rate = counter_to_binary(&counter, problem_size, '0', '1');

      println!("{}", binary_to_i32(gamma_rate) * binary_to_i32(epsilon_rate))
    },
    Err(_) => println!("Something went wrong"),
  }
}

pub fn run_problem_2() {
  match problem::get_problem(3) {
    Ok(input) => {
      let lines = input.lines().collect::<Vec<&str>>();
      let oxygen_rate = find_generator_rating(&lines, 0, '1', '0');
      let co2_rate = find_generator_rating(&lines, 0, '0', '1');

      println!("{}", oxygen_rate * co2_rate)
    },
    Err(_) => println!("Something went wrong"),
  }
}

fn update_counter(binary: &str, counter: &mut[i32; 12]) -> () {
  let mut bits = binary.chars();

  for i in 0..binary.len() {
    let bit = bits.next().unwrap();
    if bit == '1' { counter[i] = counter[i] + 1; }
  }
}

fn counter_to_binary(counter: &[i32; 12], input_size: i32, up_bit: char, down_bit: char) -> String {
  counter.iter().fold("".to_string(), |mut binary, count| { 
    if (*count as f32) / (input_size as f32) >= 0.5 {
      binary.push(up_bit);
      binary
    }
    else {
      binary.push(down_bit);
      binary
    }
   })
}

fn binary_to_i32(binary: String) -> i32 { isize::from_str_radix(&binary, 2).unwrap() as i32 }

fn find_generator_rating(binaries: &Vec<&str>, bit_position: i32, up_bit: char, down_bit: char) -> i32 {
  if binaries.len() == 1 {
    return binary_to_i32(binaries.first().unwrap().to_string());
  }

  let mut counter = [0; 12];
  let _ = &binaries.iter().for_each(|binary| { update_counter(binary, &mut counter); });
  let relevant_bit = if counter[bit_position as usize] >= (binaries.len() as f32 / 2.0).ceil() as i32 { up_bit } else { down_bit };
  let filtered_binaries = &(binaries.iter()
                                  .filter(|binary| binary.chars().nth(bit_position as usize).unwrap() == relevant_bit )
                                  .map(|&binary| binary)).collect();
  
  find_generator_rating(filtered_binaries, min(bit_position + 1, 11), up_bit, down_bit)
}
