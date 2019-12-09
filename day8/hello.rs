#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};

fn main() {
  let filename = "day8/day8.txt";
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);

  let width = 25;
  let height = 6;

  for line in reader.lines() {
    let line = line.unwrap(); // Ignore errors.
    part1(line.as_str());
    part2(line.as_str());
    break;
  }
}

fn part2(line: &str) {
  let width = 25;
  let height = 6;

  let layersize = width * height;
  let layers = line.len() / layersize;

  let mut img = Vec::<u8>::new();
  img.resize(width * height, '2' as u8);

  let mut iter = line.chars();
  for l in 0..layers {
    for y in 0..height {
      for x in 0..width {
        let mut ch = iter.next().unwrap();
        if ch == '0' {
          ch = ' ';
        }
        let cur = img[y * width + x];
        if cur == '2' as u8 {
          img[y * width + x] = ch as u8;
        }
      }
    }
  }

  for l in img.chunks(width) {
    println!("{}", std::str::from_utf8(&l).unwrap());
  }
}

fn part1(line: &str) {
  let width = 25;
  let height = 6;

  let layersize = width * height;
  println!("{}=={}", line.len(), layersize);
  let layers = line.len() / layersize;

  let mut iter = line.chars();
  let mut min_zeros = layersize;
  let mut min_result = 0;
  for l in 0..layers {
    let mut zeros = 0;
    let mut ones = 0;
    let mut twos = 0;
    for y in 0..(width * height) {
      let ch = iter.next().unwrap();
      if ch == '0' {
        zeros += 1;
      }
      if ch == '1' {
        ones += 1;
      }
      if ch == '2' {
        twos += 1;
      }
    }
    if min_zeros > zeros {
      min_zeros = zeros;
      min_result = ones * twos;
    }
  }
  println!("{}", min_result);
}
