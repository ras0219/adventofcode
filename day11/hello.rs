#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};

fn main() {
  main3("day11/day11.txt");
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
  for starting_tile in &[2, 1] {
    let mut p = Program::new(init_mem.clone());
    let (mut k, mut os) = p.run();
    let mut dir = (0, -1);
    let mut pos: (i32, i32) = (60, 50);
    let mut tiles: Vec<Vec<u8>> = Vec::new();
    for x in 0..80 {
      tiles.push(vec![2; 120]);
    }
    tiles[pos.1 as usize][pos.0 as usize] =
      *starting_tile;
    let mut painted = 0;
    loop {
      if k == PState::Halt {
        break;
      }
      let ref mut tile = tiles[pos.1 as usize][pos.0 as
                                                 usize];
      let x =
        p.run_in(
          if *tile == 2 { 0 } else { *tile as i64 },
        );
      k = x.0;
      os = x.1;
      if *tile == 2 {
        painted += 1;
      }
      *tile = os[0] as u8;
      if os[1] == 0 {
        dir = (-dir.1, dir.0);
      } else {
        dir = (dir.1, -dir.0);
      }
      pos.0 += dir.0;
      pos.1 += dir.1;
    }
    print!("painted: {}\n", painted);
    if *starting_tile == 2 {
      continue;
    }
    /*for y in 20..tiles.len() / 2 {
      for x in 5..tiles[0].len() / 2 {
        let r1 =
          tiles[y * 2].as_slice().split_at(x * 2).1;
        let r2 =
          tiles[y * 2 + 1].as_slice().split_at(x * 2).1;
        let i = if r1[0] == 1 { 1 } else { 0 } +
          if r1[1] == 1 { 2 } else { 0 } +
          if r2[0] == 1 { 4 } else { 0 } +
          if r2[1] == 1 { 8 } else { 0 };
        let chars = [
          " ",
          "`",
          "'",
          "^",
          ",",
          "[",
          "/",
          "F",
          ".",
          "\\",
          "]",
          "\\",
          "_",
          "L",
          "J",
          "#",
        ];
        print!("{}", chars[i]);
      }
      println!();
    }*/
    for y in 48..58 {
      for x in 22..67 {
        print!(
          "{}",
          [' ', '#', '.'][tiles[y][83 - x] as usize]
        );
      }
      println!();
    }
  }
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
