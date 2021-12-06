use std::str::Lines;
#[path = "./problem.rs"] mod problem;

#[derive(Debug)]
enum BingoSlot {
  Visited(i32),
  Unvisited(i32),
}

pub fn run_problem_1() -> () {
  match problem::get_problem(4) {
    Ok(input) => {
      let (sequence, mut cards) = parse_bingo(input.lines());

      for value in sequence {
        play(value, &mut cards);
        let winning_card = check_bingo(&mut cards);

        match winning_card {
          Some(card) => {
            let unmarked_score = sum_card_unmarked_slots(&card);
            println!("{}", unmarked_score * value);
            return
          },
          None => (),
        }
      }
    },
    Err(_) => println!("Something went wrong"),
  }
}

pub fn run_problem_2() -> () {
  match problem::get_problem(4) {
    Ok(input) => {
      let (sequence, mut cards) = parse_bingo(input.lines());

      for i in 0..sequence.len() {
        let value = sequence[i];
        play(value, &mut cards);
        let winning_cards = check_multiple_bingo(&mut cards);

        if cards.len() == 0 {
          match winning_cards {
            Some(last_winning_cards) => {
              let unmarked_score = sum_card_unmarked_slots(last_winning_cards.first().unwrap());
              println!("{}", unmarked_score * value);
              return;
            }
            _ => println!("Dammit"), 
          }
        }
      }
    },
    Err(_) => println!("Something went wrong"),
  }
}

fn play(value: i32, cards: &mut Vec<Vec<Vec<BingoSlot>>>) {
  cards.iter_mut().for_each(|card| {
    card.iter_mut().for_each(|line| {
      for card_value in line.iter_mut() {
        match *card_value {
          BingoSlot::Unvisited(slot_value) => {
            if slot_value == value {
              *card_value = BingoSlot::Visited(value);
            }
          },
          _ => (),
        }
      }
    });
  })
}

fn check_bingo(cards: &mut Vec<Vec<Vec<BingoSlot>>>) -> Option<Vec<Vec<BingoSlot>>> {
  for (position, card) in cards.iter_mut().enumerate() {
    if check_rows(card) || check_lines(card) {
      return Some(cards.remove(position));
    }
  }
  None
}

fn check_multiple_bingo(cards: &mut Vec<Vec<Vec<BingoSlot>>>) -> Option<Vec<Vec<Vec<BingoSlot>>>> {
  let mut removals = cards.iter().enumerate().fold(Vec::new(), |mut acc, (index, card)| {
    if check_rows(card) || check_lines(card) {
      acc.push(index);
      return acc;
    }
    acc
  });

  if removals.is_empty() { return None; }

  removals.reverse();
  Some(removals.iter().fold(Vec::new(), |mut winning_cards, index| {
    winning_cards.push(cards.remove(*index));
    winning_cards
  }))
}

fn check_rows(card: &Vec<Vec<BingoSlot>>) -> bool {
  for line in 0..5 {
    let mut is_win = true;

    for row in 0..5 {
      match card[row][line] {
        BingoSlot::Unvisited(_) => is_win = false,
        _ => (),
      }
    }

    if is_win == true { return is_win }
  }

  false
}

fn check_lines(card: &Vec<Vec<BingoSlot>>) -> bool {
  for line in card {
    let is_win = line.iter().all(|value| {
      match value {
        BingoSlot::Visited(_) => true,
        BingoSlot::Unvisited(_) => false,
      }
    });

    if is_win == true { return is_win }
  };

  false
}

fn sum_card_unmarked_slots(card: &Vec<Vec<BingoSlot>>) -> i32 {
  card.iter().fold(0, |mut score, line| {
    for slot in line {
      match slot {
        BingoSlot::Unvisited(slot_value) => score += slot_value,
        BingoSlot::Visited(_) => (),
      }
    }
    score
  })
}

fn parse_bingo(mut lines: Lines) -> (Vec<i32>, Vec<Vec<Vec<BingoSlot>>>) {
  let sequence: Vec<i32>  = lines.next().unwrap().split(',').map(|value| value.parse::<i32>().unwrap() ).collect();
  let mut cards: Vec<Vec<Vec<BingoSlot>>> = Vec::new();
  let mut card: Vec<Vec<BingoSlot>> = Vec::new();
  
  lines.next(); // ignore white space
  
  for line in lines {
    match line {
      "" => {
        cards.push(card);
        card = Vec::new();
      },
      _ => {
        let bingo_line_values = line.split_whitespace();
        let mut bingo_line: Vec<BingoSlot> = Vec::new();

        for value in bingo_line_values { bingo_line.push(BingoSlot::Unvisited((*value).parse::<i32>().unwrap())) };
        card.push(bingo_line);
      }
    }
  }
  cards.push(card);
  (sequence, cards)
}