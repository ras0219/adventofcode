#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::cmp::{max, min};

enum Direct {
  U(i32),
  R(i32),
  D(i32),
  L(i32),
}

type Point = (i32, i32);

#[derive(Copy, Clone)]
struct Extents {
  max: Point,
  min: Point,
}

fn pointmax(a: Point, b: Point) -> Point {
  (max(a.0, b.0), max(a.1, b.1))
}

fn pointmin(a: Point, b: Point) -> Point {
  (min(a.0, b.0), min(a.1, b.1))
}

fn minmax(e: Extents, p: Point) -> Extents {
  Extents {
    max: pointmax(e.max, p),
    min: pointmin(e.min, p),
  }
}

fn pointadd(a: Point, b: Point) -> Point {
  return (a.0 + b.0, a.1 + b.1);
}

fn pointneg(a: Point) -> Point {
  return (-a.0, -a.1);
}

fn extents(wire: &Vec<Direct>) -> Extents {
  wire
    .iter()
    .fold(
      (
        (0, 0),
        Extents {
          max: (0, 0),
          min: (0, 0),
        },
      ),
      |(pos, ext), dir| {
        let newpos = match dir {
          Direct::U(i) => pointadd(pos, (0, -*i)),
          Direct::R(i) => pointadd(pos, (*i, 0)),
          Direct::D(i) => pointadd(pos, (0, *i)),
          Direct::L(i) => pointadd(pos, (-*i, 0)),
        };
        (newpos, minmax(ext, newpos))
      },
    )
    .1
}

fn maxextents(a: Extents, b: Extents) -> Extents {
  return Extents {
    max: pointmax(a.max, b.max),
    min: pointmin(a.min, b.min),
  };
}

fn foreach_point<F>(wire: &Vec<Direct>, mut f: F)
where
  F: FnMut(Point),
{
  wire.iter().fold((0, 0), |p, d| match d {
    Direct::U(i) => {
      for j in 1..*i + 1 {
        f((p.0, p.1 - j));
      }
      (p.0, p.1 - *i)
    }
    Direct::R(i) => {
      for j in 1..*i + 1 {
        f((p.0 + j, p.1));
      }
      (p.0 + *i, p.1)
    }
    Direct::D(i) => {
      for j in 1..*i + 1 {
        f((p.0, p.1 + j));
      }
      (p.0, p.1 + *i)
    }
    Direct::L(i) => {
      for j in 1..*i + 1 {
        f((p.0 - j, p.1));
      }
      (p.0 - *i, p.1)
    }
  });
}

fn extents_size(a: Extents) -> usize {
  return ((a.max.0 - a.min.0 + 1) *
            (a.max.1 - a.min.1 + 1)) as usize;
}

fn idx(e: Extents, p: Point) -> usize {
  (p.0 - e.min.0 +
     (p.1 - e.min.1) * (e.max.0 - e.min.0)) as usize
}

fn nearest_intersect(
  w1: &Vec<Direct>,
  w2: &Vec<Direct>,
) {
  let exts = maxextents(extents(w1), extents(w2));
  let mut data: Vec<u32> = Vec::new();
  let mut i: u32 = 1;
  data.resize(extents_size(exts), 0);
  foreach_point(w1, |p| {
    let mut d = &mut data[idx(exts, p)];
    if *d == 0 {
      *d = i;
    }
    i += 1;
  });
  let mut dist = extents_size(exts);
  i = 1;
  let mut steps = 999999999;
  foreach_point(w2, |p| {
    let d = data[idx(exts, p)];
    if d > 0 {
      let newdist = (p.0.abs() + p.1.abs()) as usize;
      dist = min(dist, newdist);
      steps = min(steps, i + d);
    }
    i += 1;
  });
  print!("Nearest dist: {}\n", dist);
  print!("Nearest steps: {}\n", steps);
}

fn main() {
  main2("day3/test1.txt");
  main2("day3/test2.txt");
  main2("day3/day3.txt");
}

fn main2(filename: &'static str) {
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);

  let mut total = 0;
  let init_data: Vec<Vec<Direct>> = reader
    .lines()
    .map(|line| {
      let line = line.unwrap();
      if line.is_empty() {
        Vec::new()
      } else {
        line
          .split(",")
          .map(|entry| {
            let (code, val) = entry.split_at(1);
            let ival = i32::from_str(val).unwrap();
            match code {
              "U" => Direct::U(ival),
              "R" => Direct::R(ival),
              "D" => Direct::D(ival),
              "L" => Direct::L(ival),
              _ => panic!(),
            }
          })
          .collect::<Vec<Direct>>()
      }
    })
    .collect();

  nearest_intersect(&init_data[0], &init_data[1])
}
