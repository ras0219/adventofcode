#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::collections::{HashSet, HashMap};

fn main() {
  main2("day6/test.txt");
  main2("day6/test2.txt");
  main2("day6/day6.txt");
}

fn main2(filename: &str) {
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);

  let mut total = 0;
  let init_data: Vec<String> =
    reader.lines().map(|line| line.unwrap()).collect();
  let edges: Vec<(&str, &str)> = init_data
    .iter()
    .map(|l| {
      let e: Vec<_> = l.split(")").collect();
      (e[0], e[1])
    })
    .collect();

  print!("{} edges.\n", edges.len());
  let mut total = 0;
  let mut map = HashMap::new();
  for e in edges.iter() {
    map.insert(e.1, e.0);
  }
  for p in map.iter() {
    let mut x = map.get(p.1);
    loop {
      total += 1;
      x = match x {
        Some(y) => map.get(*y),
        None => break,
      };
    }
  }
  print!("{} orbits.\n", total);

  let path = |v: &str| -> Vec<&str> {

    let mut p = Vec::new();
    let mut x = map.get(v);
    loop {
      x = match x {
        Some(y) => {
          p.push(*y);
          map.get(*y)
        }
        None => break,
      };
    }
    return p;
  };
  let pyou = path("YOU");
  let psan = path("SAN");
  let sharecnt = pyou
    .iter()
    .rev()
    .zip(psan.iter().rev())
    .take_while(|(a, b)| a == b)
    .count();

  let dist = pyou.len() + psan.len() - 2 * sharecnt;
  print!("{} distance\n", dist);
}
