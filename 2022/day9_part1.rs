use std::fs;
use std::collections::HashSet;

enum Dir {
  U,
  D,
  L,
  R,
}

struct Point {
  x: i32,
  y: i32,
}

struct Rope {
  head: Point,
  tail: Point,
}

impl Rope {
  fn new(x: i32, y: i32) -> Self {
    Self {
      head: Point{x, y},
      tail: Point{x, y}
    }  
  }

  fn move_rope(&mut self, dir: &Dir) -> (i32, i32) {
    match dir {
      Dir::U => {
        self.head.y += 1;
        let dist = self.head_tail_distance();
        if dist.y > 1 {
          self.tail.y = self.head.y - 1;
          self.tail.x = self.head.x;
        }
      },
      Dir::D => {
        self.head.y -= 1;
        let dist = self.head_tail_distance();
        if dist.y > 1 {
          self.tail.y = self.head.y + 1;
          self.tail.x = self.head.x;
        }
      },
      Dir::L => {
        self.head.x -= 1;
        let dist = self.head_tail_distance();
        if dist.x > 1 {
          self.tail.x = self.head.x + 1;
          self.tail.y = self.head.y;
        }
      },
      Dir::R => {
        self.head.x += 1;
        let dist = self.head_tail_distance();
        if dist.x > 1 {
          self.tail.x = self.head.x - 1;
          self.tail.y = self.head.y;
        }
      },
    }
    (self.tail.x, self.tail.y)
  }

  fn head_tail_distance(&self) -> Point {
    Point { x: i32::abs(self.head.x - self.tail.x), y: i32::abs(self.head.y - self.tail.y) }
  }
}

fn get_move(line: &str) -> (Dir, i32) {
  let cmd: Vec<&str> = line.split(" ").collect();
  let dir = cmd[0];
  let cnt = cmd[1].parse::<i32>().unwrap();
  match dir {
    "D" => (Dir::D, cnt),
    "U" => (Dir::U, cnt),
    "R" => (Dir::R, cnt),
    "L" => (Dir::L, cnt),
    _ => panic!("Direction is not supported!"),
  }
}

fn main() {
  let content = fs::read_to_string("input").expect("File 'input' not found!");

  let mut pos = HashSet::new();
  let mut rope = Rope::new(0, 0);
  pos.insert(( 0, 0 ));

  for line in content.lines() {
    let (dir, cnt) = get_move(line);
    for _ in 0..cnt {
      pos.insert(rope.move_rope(&dir));
    }
  }

  println!("Part1: {}", pos.len());
}