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
  points: Vec<Point>,
}

impl Rope {
  fn new() -> Self {
    Self {
      points: (0..10).map(|_| Point { x: 0, y: 0 }).collect(),
    }  
  }

  fn traverse_knot(&mut self, idx: usize, dir: &Dir) {
    if idx > 9 {
      return;
    }

    let dist = self.cal_dist(idx-1, idx);
    let mut is_moved = false;
    if i32::abs(dist.x) > 1 {
      self.points[idx].x += if dist.x > 0 { 1 } else { -1 };
      if i32::abs(dist.y) != 0 {
        self.points[idx].y += if dist.y > 0 { 1 } else { -1 };
      }
      is_moved = true;
    } else if i32::abs(dist.y) > 1 {
      self.points[idx].y += if dist.y > 0 { 1 } else { -1 };
      if i32::abs(dist.x) != 0 {
        self.points[idx].x += if dist.x > 0 { 1 } else { -1 };
      }
      is_moved = true;
    }

    if is_moved {
      self.traverse_knot(idx+1, dir);
    }
  }

  fn move_rope(&mut self, dir: &Dir) -> (i32, i32) {
    match dir {
      Dir::U => {
        self.points[0].y += 1
      },
      Dir::D => {
        self.points[0].y -= 1;
      },
      Dir::L => {
        self.points[0].x -= 1;
      },
      Dir::R => {
        self.points[0].x += 1;
      },
    }
    self.traverse_knot(1, dir);
    return (self.points[9].x, self.points[9].y);
  }

  fn cal_dist(&self, idx1: usize, idx2: usize) -> Point {
    Point { 
      x: self.points[idx1].x - self.points[idx2].x, 
      y: self.points[idx1].y - self.points[idx2].y 
    }
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
  let mut rope = Rope::new();
  pos.insert(( 0, 0 ));

  for line in content.lines() {
    let (dir, cnt) = get_move(line);
    for _ in 0..cnt {
      pos.insert(rope.move_rope(&dir));
    }
  }

  // for i in 0..100 {
  //   for j in 0..100 {
  //     if i == 50 && j == 50 {
  //       print!("S");
  //     } else if pos.contains(&(i-50, j-50)) {
  //       print!("#");
  //     } else {
  //       print!(".");
  //     }
  //   }
  //   println!("");
  // }

  println!("Part2: {}", pos.len());
}