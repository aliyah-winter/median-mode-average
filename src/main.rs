use rand::Rng;
use std::collections::HashMap;

fn main() {
  let mut data: Vec<u32> = Vec::new();
  for _num in 1..=10 {
    data.push(rand::thread_rng().gen_range(0..=10))
  };

  data.sort();

  println!("Random data is: {:?}", data);

  println!("The median is: {}", median(&mut data));

  mode(&data);

  let average: u32 = data.iter().fold(0, |acc, num| acc + num) / data.len() as u32;

  println!("The average is: {average}");
}

fn median(data: &mut Vec<u32>) -> u32 {
  match data.len() % 2 == 0 {
    true => (data[data.len() / 2] + data[data.len() / 2 - 1]) / 2,
    false => data[data.len() / 2],
  }
  
}

fn mode(data: &Vec<u32>) {
  let mut mode_map = HashMap::new();
  for num in data {
    let count = mode_map.entry(num).or_insert(0);
    *count += 1;
  }
  let mut sorted: Vec<_> = mode_map.iter().collect();
  sorted.sort_by_key(|a| a.1);
  match sorted.pop() {
    Some(num) => println!("The mode is: {}", num.0),
    None => println!("Could not find the mode."),
  }
}



