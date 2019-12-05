#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};

fn main() {
  let filename = "day5/day5.txt";
  main2(filename)
}

fn load(filename: &'static str) -> Vec<i32> {
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);

  let mut total = 0;
  let init_data: Vec<i32> = reader
    .lines()
    .map(|line| {
      let line = line.unwrap(); // Ignore errors.
      line
        .split(",")
        .map(|entry| {
          let n = i32::from_str(entry).unwrap();
          // println!("{}", -n);
          return n;
        })
        .collect::<Vec<i32>>()
    })
    .flatten()
    .collect();
  return init_data;
}

fn run(mut data: Vec<i32>, arg: i32) -> Vec<i32> {
  let mut ip: i32 = 0;
  let mut output: Vec<i32> = Vec::new();
  loop {
    let uip = ip as usize;
    let v = data[uip];
    let param = |data: &Vec<i32>, i| -> i32 {
      let mode = match i {
        1 => v / 100 % 10,
        2 => v / 1000 % 10,
        3 => v / 10000 % 10,
        _ => panic!(),
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
        write(&mut data, 1, arg);
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
  let result = run(init_data.clone(), 1);
  for x in result {
    print!("(1) OUT: {}\n", x);
  }
  let r2 = run(init_data.clone(), 5);
  for x in r2 {
    print!("(5) OUT: {}\n", x);
  }

}
// vim: set tabstop=2 shiftwidth=2 expandtab
