#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::ops::Rem;
use std::collections::{HashSet, HashMap};

fn main() {
  main2("day16/test1.txt");
  main2("day16/test2.txt");
  main2("day16/day16.txt");
}

fn main2(filename: &str) {
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  println!("file: {}", filename);

  let data: Vec<String> =
    reader.lines().map(|l| l.unwrap()).collect();
  let signal: Vec<u8> = data[0]
    .as_bytes()
    .iter()
    .map(|x| x - '0' as u8)
    .collect();

  let fft = |xs: &Vec<u8>| -> Vec<u8> {
    let mut ys = vec![0; xs.len()];
    for iy in 0..ys.len() {
      let mut acc: i64 = 0;
      for (ix, x) in xs.iter().enumerate() {
        let i = ((ix + 1) / (iy + 1)) % 4;
        if i == 1 {
          acc += *x as i64;
        }
        if i == 3 {
          acc -= *x as i64;
        }
      }
      ys[iy] = (acc.abs() % 10) as u8;
    }
    return ys;
  };
  let prn = |x: &[u8]| {
    for s in x {
      print!("{}", s);
    }
    println!();
  };

  print!("{}=", signal.len());
  prn(&signal);
  let s2 = fft(&signal);
  print!("iter 1 = ");
  prn(&s2.as_slice().split_at(8).0);
  let s2 =
    (0..100).fold(signal.clone(), |s, _| fft(&s));
  print!("iter 100 = ");
  prn(&s2.as_slice().split_at(8).0);

  let offset = (0..7).fold(0, |a, i| {
    a * 10 + (signal[i] as usize)
  });
  if offset > signal.len() * 10000 {
    return;
  }
  let mut big = vec![0; signal.len() * 10000];
  for i in 0..big.len() {
    big[i] = signal[i % signal.len()];
  }
  if offset < big.len() / 2 {
    panic!();
  }
  for _ in 0..100 {
    for i in (offset..big.len() - 1).rev() {
      big[i] += big[i + 1];
      big[i] %= 10;
    }
  }
  for i in 0..8 {
    print!("{}", big[offset + i]);
  }
  println!();
}
