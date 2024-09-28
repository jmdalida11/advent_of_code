use std::fs;

fn main() {
  let content = fs::read_to_string("input").expect("File 'input' not found!");

  for line in content.lines() {
  }
}