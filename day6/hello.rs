#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn main() {
  let filename = "day6/day6.txt";
  // Open the file in read-only mode (ignoring errors).
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
}
