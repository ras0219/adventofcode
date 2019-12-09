#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};

fn main() {
  main2("day7/test1.txt");
  main2("day7/day7.txt");
  main3("day7/test2.txt");
  main3("day7/day7.txt");
}

fn load(filename: &str) -> Vec<i32> {
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);

  let mut total = 0;
  let init_data: Vec<i32> = reader
    .lines()
    .map(|line| {
      let line = line.unwrap(); // Ignore errors.
      line
        .split(",")
        .map(|entry| i32::from_str(entry).unwrap())
        .collect::<Vec<i32>>()
    })
    .flatten()
    .collect();
  return init_data;
}

fn foreach_permute<F>(mut f: F)
where
  F: FnMut(i32, i32, i32, i32, i32),
{
  for a in 5..10 {
    for b in 5..10 {
      if a == b {
        continue;
      }
      for c in 5..10 {
        if [a, b].contains(&c) {
          continue;
        }
        for d in 5..10 {
          if [a, b, c].contains(&d) {
            continue;
          }
          for e in 5..10 {
            if [a, b, c, d].contains(&e) {
              continue;
            }
            f(a, b, c, d, e);
          }
        }
      }
    }
  }
}

fn main3(filename: &str) {
  let init_mem = load(filename);

  let mut max_t = 0;
  let mut max_msg = String::from_str("none\n").unwrap();

  foreach_permute(|a, b, c, d, e| {
    let mut ap = Program::new(init_mem.clone());
    let mut bp = Program::new(init_mem.clone());
    let mut cp = Program::new(init_mem.clone());
    let mut dp = Program::new(init_mem.clone());
    let mut ep = Program::new(init_mem.clone());
    let mut ps =
      [&mut ap, &mut bp, &mut cp, &mut dp, &mut ep];
    for (p, s) in ps.iter_mut().zip(&[a, b, c, d, e]) {
      p.run();
      p.run_in(*s);
    }

    let mut t = 0;
    let mut q = PState::Input;
    loop {
      for p in ps.iter_mut() {
        let (newstate, out) = p.run_in(t);
        t = out[0];
        q = newstate;
      }
      if q == PState::Halt {
        break;
      }
    }
    if t > max_t {
      max_msg = format!(
        "main3({},{}{}{}{}{})={})\n",
        filename,
        a,
        b,
        c,
        d,
        e,
        t
      );
      max_t = t;
    }
  });
  print!("{}", max_msg);
}

struct Program {
  mem: Vec<i32>,
  ip: usize,
}

#[derive(PartialEq)]
enum PState {
  Input,
  Halt,
}

impl Program {
  fn new(mem: Vec<i32>) -> Program {
    Program { mem, ip: 0 }
  }

  fn param(&self, i: usize) -> i32 {
    let v = self.mem[self.ip];
    let mode = match i {
      1 => v / 100 % 10,
      2 => v / 1000 % 10,
      3 => v / 10000 % 10,
      _ => panic!("unexpected mode"),
    };
    if mode == 0 {
      self.mem[self.mem[self.ip + i] as usize]
    } else {
      self.mem[self.ip + i]
    }
  }
  fn write(&mut self, i: usize, val: i32) {
    let dst = self.mem[self.ip + i] as usize;
    self.mem[dst] = val;
  }

  fn run(&mut self) -> (PState, Vec<i32>) {
    let mut output: Vec<i32> = Vec::new();
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
        _ => {
          break;
        }
      }
    }
    return (PState::Halt, output);
  }
  fn run_in(
    &mut self,
    input: i32,
  ) -> (PState, Vec<i32>) {
    self.write(1, input);
    self.ip += 2;
    self.run()

  }
}

fn run(mut data: Vec<i32>, args_: &[i32]) -> Vec<i32> {
  let mut ip: i32 = 0;
  let mut args = args_;
  let mut output: Vec<i32> = Vec::new();
  loop {
    let uip = ip as usize;
    let v = data[uip];
    let param = |data: &Vec<i32>, i| -> i32 {
      let mode = match i {
        1 => v / 100 % 10,
        2 => v / 1000 % 10,
        3 => v / 10000 % 10,
        _ => panic!("unexpected mode"),
      };
      if mode == 0 {
        data[data[uip + i] as usize]
      } else {
        data[uip + i]
      }
    };
    let write = |data: &mut Vec<i32>, i: usize, val| {
      let dst = data[uip + i] as usize;
      data[dst] = val;
    };
    match v % 100 {
      1 => {
        let val = param(&data, 1) + param(&data, 2);
        write(&mut data, 3, val);
        ip += 4;
      }
      2 => {
        let val = param(&data, 1) * param(&data, 2);
        write(&mut data, 3, val);
        ip += 4;
      }
      3 => {
        let n = args.split_first().unwrap();
        args = n.1;
        write(&mut data, 1, *n.0);
        ip += 2;
      }
      4 => {
        output.push(param(&data, 1));
        ip += 2;
      }
      5 => {
        // jnz
        if param(&data, 1) != 0 {
          ip = param(&data, 2);
        } else {
          ip += 3;
        }
      }
      6 => {
        // jz
        if param(&data, 1) == 0 {
          ip = param(&data, 2);
        } else {
          ip += 3;
        }
      }
      7 => {
        // lt
        let val =
          if param(&data, 1) < param(&data, 2) {
            1
          } else {
            0
          };
        write(&mut data, 3, val);
        ip += 4;
      }
      8 => {
        // eq
        let val =
          if param(&data, 1) == param(&data, 2) {
            1
          } else {
            0
          };
        write(&mut data, 3, val);
        ip += 4;
      }
      _ => {
        break;
      }
    }
  }
  return output;
}

fn main2(filename: &'static str) {
  let init_data = load(filename);
  let mut max_thrust = 0;
  let mut max_setting = String::new();
  for a in 0..5 {
    let r_a = run(init_data.clone(), &[a, 0]);
    for b in 0..5 {
      if a == b {
        continue;
      }
      let r_b = run(init_data.clone(), &[b, r_a[0]]);
      for c in 0..5 {
        if [a, b].contains(&c) {
          continue;
        }
        let r_c = run(init_data.clone(), &[c, r_b[0]]);
        for d in 0..5 {
          if [a, b, c].contains(&d) {
            continue;
          }
          let r_d =
            run(init_data.clone(), &[d, r_c[0]]);
          for e in 0..5 {
            if [a, b, c, d].contains(&e) {
              continue;
            }
            let r_e =
              run(init_data.clone(), &[e, r_d[0]]);
            if r_e[0] > max_thrust {
              max_setting = format!(
                "{}: {}{}{}{}{} => {}",
                filename,
                a,
                b,
                c,
                d,
                e,
                r_e[0]
              );
              max_thrust = r_e[0];
            }
          }
        }
      }
    }
  }
  print!("{}\n", max_setting);
}
// vim: set tabstop=2 shiftwidth=2 expandtab
