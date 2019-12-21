#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use std::cmp::{max, min};

fn main() {
  main3("day15/day15.txt");
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

type Robot = (i32, i32, i32, Program);

fn main3(filename: &str) {
  let init_mem = load(filename);
  print!("file: {}\n", filename);
  let mut p = Program::new(init_mem.clone());
  let (mut k, mut os) = p.run();
  let mut tiles: Vec<Vec<i32>> = Vec::new();
  for x in 0..46 {
    tiles.push(vec![0; 46]);
  }
  let display =
    |tiles: &Vec<Vec<i32>>| for row in tiles {
      for c in row {
        if *c == 2 {
          print!("#");
        } else if *c == 1 {
          print!(" ");
        } else {
          print!(".");
        }
      }
      println!();
    };

  let mut oxypos = (0, 0);
  {
    let mut stk: VecDeque<Robot> = VecDeque::new();
    stk.push_back((25, 25, 1, p));
    loop {
      if stk.is_empty() {
        break;
      }
      let mut r = stk.pop_front().unwrap();
      for (n, x, y) in [
        (1, 0, -1),
        (2, 0, 1),
        (3, -1, 0),
        (4, 1, 0),
      ].iter()
      {
        let mut r2 = r.clone();
        r2.0 += x;
        r2.1 += y;
        let t = &mut tiles[r2.1 as usize][r2.0 as
                                            usize];
        if *t != 0 {
          continue;
        }
        *t = 1;
        let (_, mut os) = r2.3.run_in(*n);
        if os[0] == 0 {
          *t = 2;
          continue;
        } else if os[0] == 2 {
          println!("Score: {}", r2.2);
          oxypos = (r2.0 as isize, r2.1 as isize);
        }
        r2.2 += 1;
        stk.push_back(r2);
      }
    }
  }
  display(&tiles);
  {
    let mut maxstep = 0;
    let mut stk: VecDeque<(isize, isize, i32)> =
      VecDeque::new();
    stk.push_back((oxypos.0, oxypos.1, 0));
    loop {
      if stk.is_empty() {
        break;
      }
      let mut r = stk.pop_front().unwrap();
      maxstep = max(maxstep, r.2);
      for (x, y) in [(0, -1), (0, 1), (-1, 0), (1, 0)]
        .iter()
      {
        let mut r2 = r.clone();
        r2.0 += x;
        r2.1 += y;
        let t = &mut tiles[r2.1 as usize][r2.0 as
                                            usize];
        if *t != 1 {
          continue;
        }
        *t = 0;
        r2.2 += 1;
        stk.push_back(r2);
      }
    }
    println!("minutes: {}", maxstep);
  }
}

#[derive(Clone)]
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
