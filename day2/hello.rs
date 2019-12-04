#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn main() {
  let filename = "day2/day2.txt";
  // Open the file in read-only mode (ignoring errors).
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

  let target = 19690720;
  for arg1 in (0..100) {
    for arg2 in (0..100) {
      let mut data = init_data.clone();
      data[1] = arg1;
      data[2] = arg2;

      let mut ip: i32 = 0;
      loop {
        let uip = ip as usize;
        let v = data[uip];
        match v {
          1 => {
            let src1 = data[uip + 1] as usize;
            let src2 = data[uip + 2] as usize;
            let dst = data[uip + 3] as usize;
            data[dst] = data[src1] + data[src2];
          }
          2 => {
            let src1 = data[uip + 1] as usize;
            let src2 = data[uip + 2] as usize;
            let dst = data[uip + 3] as usize;
            data[dst] = data[src1] * data[src2];
          }
          _ => {
            break;
          }
        }
        ip += 4;
      }

      if data[0] == target {
        print!("{} {} = {}\n", arg1, arg2, data[0]);
        return;
      }
    }
  }
}

