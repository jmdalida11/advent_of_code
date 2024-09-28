use std::fs;

enum Cmd {
  Noop,
  Addx(i64),
}

struct Cpu {
  x: i64,
  cycle: usize,
  signal_strength: i64,
  screen: [char; 240],
}

impl Cpu {
  fn new() -> Self {
    Self {
      x: 1,
      cycle: 0,
      signal_strength: 0,
      screen: ['.'; 240],
    }
  }

  fn addx(&mut self, v: i64) {
    self.x += v;
  }

  fn one_cycle(&mut self) {
    self.draw();
    self.cycle += 1;
    self.add_signal_strength();
  }

  fn draw(&mut self) {
    let diff = i64::abs(self.x - (self.cycle % 40) as i64);
    if diff == 0 || diff == 1 {
      self.screen[self.cycle] = '#';
    }
  }

  fn add_signal_strength(&mut self) {
    match self.cycle {
      20 | 60 | 100 | 140 | 180 | 220 => {
        self.signal_strength += self.x * self.cycle as i64;
      },
      _ => {},
    }
  }

  fn fetch_cmd(&mut self, cmd: Cmd) {
    match cmd {
      Cmd::Noop => {
        self.one_cycle();
      },
      Cmd::Addx(v) => {
        self.one_cycle();
        self.one_cycle();
        self.addx(v);
      }
    }
  }
}

fn parse_line(line: &str) -> Cmd {
  let cmd = line.split(" ").collect::<Vec<&str>>();
  match cmd[0] {
    "noop" => Cmd::Noop,
    "addx" => Cmd::Addx(cmd[1].parse::<i64>().unwrap()),
    _ => panic!("Command is not supported!"),
  }
}

fn main() {
  let content = fs::read_to_string("input").expect("File 'input' not found!");
  
  let mut cpu = Cpu::new();
  for line in content.lines() {
    let cmd = parse_line(line);
    cpu.fetch_cmd(cmd);
  }

  println!("Part1: {}", cpu.signal_strength);
  println!("Part2:");

  for i in 0..240 {
    if i % 40 == 0 && i != 0 {
      println!("");
    }
    print!("{}", cpu.screen[i]);
  }
}
