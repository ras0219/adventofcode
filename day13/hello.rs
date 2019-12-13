#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};

fn main() {
  main3("day13/day13.txt");
}

fn load(filename: &str) -> Vec<i64> {
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);

  let mut total = 0;
  let init_data: Vec<i64> = reader
    .lines()
    .map(|line| {
      let line = line.unwrap(); // Ignore errors.
      line
        .split(",")
        .map(|entry| i64::from_str(entry).unwrap())
        .collect::<Vec<i64>>()
    })
    .flatten()
    .collect();
  return init_data;
}

fn main3(filename: &str) {
  let init_mem = load(filename);
  print!("file: {}\n", filename);
  {
    let mut p = Program::new(init_mem.clone());
    let (mut k, mut os) = p.run();
    let mut tiles: Vec<Vec<u8>> = Vec::new();
    for x in 0..80 {
      tiles.push(vec![0; 120]);
    }
    if k != PState::Halt {
      panic!("expected halt");
    }
    for x in 0..os.len() / 3 {
      let inst = os.split_at(x * 3).1;
      let tx = inst[0] as usize;
      let ty = inst[1] as usize;
      tiles[ty][tx] = inst[2] as u8;
    }

    let mut total = 0;
    for row in tiles {
      for cell in row {
        if cell == 2 {
          total += 1;
        }
      }
    }
    print!("total blocks: {}\n", total);
  }
  // part 2
  let mut p = Program::new(init_mem.clone());
  p.mem[0] = 2;
  let mut tiles: Vec<Vec<u8>> = Vec::new();
  for x in 0..80 {
    tiles.push(vec![0; 120]);
  }
  let mut ball = (0, 0, 1, 1);
  let mut paddle = (0, 0);
  let mut score = 0;
  let (mut k, mut os) = p.run();
  loop {
    for x in 0..os.len() / 3 {
      let inst = os.split_at(x * 3).1;
      if inst[0] == -1 {
        score = inst[2];
        continue;
      }
      if inst[2] == 4 {
        ball.0 = inst[0];
        ball.1 = inst[1];
      }
      if inst[2] == 3 {
        paddle.0 = inst[0];
        paddle.1 = inst[1];
      }
      let tx = inst[0] as usize;
      let ty = inst[1] as usize;
      tiles[ty][tx] = inst[2] as u8;
    }
    if k == PState::Halt {
      break;
    }
    // compute input
    let input = if paddle.0 < ball.0 {
      1
    } else if paddle.0 > ball.0 {
      -1
    } else {
      0
    };
    let r = p.run_in(input);
    k = r.0;
    os = r.1;
  }
  println!("Score: {}", score);
}

struct Program {
  mem: Vec<i64>,
  ip: usize,
  rbase: i64,
}

#[derive(PartialEq)]
enum PState {
  Input,
  Halt,
}
fn param_mode(instr: i64, i: usize) -> i64 {
  match i {
    1 => instr / 100 % 10,
    2 => instr / 1000 % 10,
    3 => instr / 10000 % 10,
    _ => panic!("unexpected param index"),
  }
}


impl Program {
  fn new(mut mem: Vec<i64>) -> Program {
    mem.resize(10000, 0);
    Program {
      mem,
      ip: 0,
      rbase: 0,
    }
  }

  fn param(&self, i: usize) -> i64 {
    let v = self.mem[self.ip];
    let mode = param_mode(v, i);
    let p = self.mem[self.ip + i];
    if mode == 0 {
      self.mem[p as usize]
    } else if mode == 1 {
      p
    } else if mode == 2 {
      self.mem[(self.rbase + p) as usize]
    } else {
      panic!()
    }
  }
  fn write(&mut self, i: usize, val: i64) {
    let mode = param_mode(self.mem[self.ip], i);
    let dst = self.mem[self.ip + i];
    if mode == 0 {
      self.mem[dst as usize] = val;
    } else if mode == 2 {
      self.mem[(dst + self.rbase) as usize] = val;
    } else {
      panic!();
    }
  }

  fn run(&mut self) -> (PState, Vec<i64>) {
    let mut output: Vec<i64> = Vec::new();
    loop {
      let v = self.mem[self.ip];
      match v % 100 {
        1 => {
          let val = self.param(1) + self.param(2);
          self.write(3, val);
          self.ip += 4;
        }
        2 => {
          let val = self.param(1) * self.param(2);
          self.write(3, val);
          self.ip += 4;
        }
        3 => {
          return (PState::Input, output);
        }
        4 => {
          output.push(self.param(1));
          self.ip += 2;
        }
        5 => {
          // jnz
          if self.param(1) != 0 {
            self.ip = self.param(2) as usize;
          } else {
            self.ip += 3;
          }
        }
        6 => {
          // jz
          if self.param(1) == 0 {
            self.ip = self.param(2) as usize;
          } else {
            self.ip += 3;
          }
        }
        7 => {
          // lt
          let val = if self.param(1) < self.param(2) {
            1
          } else {
            0
          };
          self.write(3, val);
          self.ip += 4;
        }
        8 => {
          // eq
          let val = if self.param(1) == self.param(2) {
            1
          } else {
            0
          };
          self.write(3, val);
          self.ip += 4;
        }
        9 => {
          let p = self.param(1);
          self.rbase += p;
          self.ip += 2;
        }
        _ => {
          break;
        }
      }
    }
    return (PState::Halt, output);
  }
  fn run_in(
    &mut self,
    input: i64,
  ) -> (PState, Vec<i64>) {
    self.write(1, input);
    self.ip += 2;
    self.run()
  }
}
// vim: set tabstop=2 shiftwidth=2 expandtab
