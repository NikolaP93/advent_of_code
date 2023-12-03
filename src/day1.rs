use std::fs;
use std::io::{self, Read};

pub fn run(){
  match read_file() {
    Err(e) => println!("{:?}", e),
    _ => ()
  }
}

fn read_file() -> io::Result<()> {
  let filename = "src/day1.md";
  let mut file = fs::File::open(filename)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let mut final_num: i32 = 0;

  for line in contents.lines() {
    final_num += add_nums_from_string(line);
  }

  println!("{}",final_num);

  Ok(())
}

fn add_nums_from_string(s: &str) -> i32 {
  let first_digit = s.chars().find(|c| c.is_digit(10));
  let last_digit = s.chars().rev().find(|c| c.is_digit(10));

  match (first_digit, last_digit) {
      (Some(f), Some(l)) => format!("{}{}", f, l).parse::<i32>().unwrap_or(0),
      _ => 0,
  }
}