#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::ops::Rem;

fn main() {
  main3("day10/test1.txt");
  main3("day10/test4.txt");
  main3("day10/day10.txt");
}

fn main3(filename: &str) {
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  let init_data: Vec<String> =
    reader.lines().map(|line| line.unwrap()).collect();
  let byte_data: Vec<&[u8]> = init_data
    .iter()
    .map(|ref x| x.as_bytes())
    .collect();

  print!("File: {}\n", filename);
  print!("{}\n", byte_data.len() * byte_data[0].len());
  print!(
    "{},{}\n",
    byte_data.len(),
    byte_data[0].len()
  );

  let height = byte_data.len();
  let width = byte_data[0].len();

  let mut counts: Vec<Vec<i32>> = byte_data
    .iter()
    .map(|&x| {
      let mut v = Vec::new();
      v.resize(x.len(), 0);
      v
    })
    .collect();

  let mut offsets: Vec<(i32, i32)> = Vec::new();

  for a in 0..height as i32 {
    for b in 1..width as i32 {
      let g = gcd(a as usize, b as usize);
      if g != 1 {
        continue;
      }
      offsets.push((a, b));
    }
  }
  print!("Computing\n");
  let at = |(x, y): (i32, i32)| {
    byte_data[y as usize][x as usize]
  };
  let add = |(x, y): (i32, i32),
             (x2, y2): (i32, i32)| (x + x2, y + y2);
  let in_bounds = |(x, y): (i32, i32)| {
    x >= 0 && (x < width as i32) && y >= 0 &&
      (y < height as i32)
  };
  let empty = '.' as u8;
  for offset in &offsets {
    //print!("offset {} {}\n", offset.0, offset.1);
    for y in 0..height as i32 {
      if y + offset.1 >= height as i32 {
        break;
      }
      for x in 0..width as i32 {
        if x + offset.0 >= width as i32 {
          break;
        }
        if offset.0 <= x && offset.1 <= y {
          break;
        }
        let max_x = width as i32 - 1 - x;
        let max_y = height as i32 - 1 - y;
        let dirs =
          [
            ((x, y), *offset),
            ((max_y, x), (-offset.1, offset.0)),
            ((max_x, max_y), (-offset.0, -offset.1)),
            ((y, max_x), (offset.1, -offset.0)),
          ];
        for (start, off) in &dirs {
          /*print!(
            "pos({},{}) towards ({},{})\n",
            start.0,
            start.1,
            off.0,
            off.1
          );*/
          // find first asteroid
          let mut pos = *start;
          while in_bounds(pos) && at(pos) == empty {
            pos = add(pos, *off);
          }
          // skip asteroid
          pos = add(pos, *off);
          // increment all other locations
          while in_bounds(pos) {
            counts[pos.1 as usize][pos.0 as usize] += 1;
            pos = add(pos, *off);
          }
        }
      }
    }
  }

  let mut max_aster = 0;
  let mut max_pos = (0, 0);
  for y in 0..height {
    for x in 0..width {
      let c = counts[y][x];
      if byte_data[y][x] == empty {
        //print!(".");
        continue;
      }
      //print!("{}", c % 10);
      if c > max_aster {
        max_aster = c;
        max_pos = (x, y);
      }
    }
    //println!();
  }
  print!(
    "Max observ: {} at {},{}\n",
    max_aster,
    max_pos.0,
    max_pos.1
  );

  offsets.iter_mut().for_each(
    |mut x| { *x = (x.0, -x.1); },
  );
  offsets.sort_by_key(|x| -10000 * x.0 / x.1);

  let seq1 = offsets.iter().map(|o| (o.0, o.1));
  let seq2 = offsets.iter().map(|o| (-o.1, o.0));
  let seq3 = offsets.iter().map(|(a, b)| (-a, -b));
  let seq4 = offsets.iter().map(|o| (o.1, -o.0));

  let mut count = 200;
  'l: for o in seq1.chain(seq2).chain(seq3).chain(
    seq4,
  )
  {
    let mut start =
      (max_pos.0 as i32, max_pos.1 as i32);
    loop {
      start = add(start, o);
      if !in_bounds(start) {
        break;
      }
      if at(start) != empty {
        count -= 1;
        if count == 0 {
          print!(
            "200th destroyed: {},{}\n",
            start.0,
            start.1
          );
          break 'l;
        }
        break;
      }
    }
  }
}

fn gcd<T>(a: T, b: T) -> T
where
  T: Eq,
  T: Rem<T, Output = T>,
  T: From<u8>,
  T: Copy,
{
  if b == T::from(0) {
    a
  } else if a == T::from(0) {
    b
  } else {
    gcd(b, a % b)
  }
}
