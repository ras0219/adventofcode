#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::ops::Rem;

fn main() {
  main2("day12/test1.txt");
  main2("day12/day12.txt");
}
fn main2(filename: &str) {
  let file = File::open(filename).unwrap();
  let reader = BufReader::new(file);
  println!("file: {}", filename);

  let points_init: Vec<(i64, i64, i64)> = reader
    .lines()
    .map(|line| {
      let line = line.unwrap();
      let line = line.split_at(1).1;
      let mut i = line
        .split(" ")
        .map(
          |s| s.split_at(2).1.split_at(s.len() - 3).0,
        )
        .map(|s| i64::from_str(s).unwrap());
      let x = i.next().unwrap();
      let y = i.next().unwrap();
      let z = i.next().unwrap();
      (x, y, z)
    })
    .collect();

  let mut points = points_init.clone();
  let mut vels: Vec<(i64, i64, i64)> =
    vec![(0, 0, 0); points.len()];
  for p in &points {
    println!("{},{},{}", p.0, p.1, p.2);
  }

  for step in 0..1000 {
    for (i, p) in points.iter().enumerate() {
      for (j, q) in points.iter().enumerate() {
        if j == i {
          continue;
        }
        if p.0 < q.0 {
          vels[i].0 += 1;
        }
        if p.0 > q.0 {
          vels[i].0 -= 1;
        }
        if p.1 < q.1 {
          vels[i].1 += 1;
        }
        if p.1 > q.1 {
          vels[i].1 -= 1;
        }
        if p.2 < q.2 {
          vels[i].2 += 1;
        }
        if p.2 > q.2 {
          vels[i].2 -= 1;
        }
      }
    }
    for (i, v) in vels.iter().enumerate() {
      points[i].0 += v.0;
      points[i].1 += v.1;
      points[i].2 += v.2;
    }
    if step == 9 || step == 999 {
      println!("After {} steps:", step + 1);
      for p in &points {
        println!("{},{},{}", p.0, p.1, p.2);
      }
      let total: i64 = points
        .iter()
        .enumerate()
        .map(|(i, p)| {
          (p.0.abs() + p.1.abs() + p.2.abs()) *
            (vels[i].0.abs() + vels[i].1.abs() +
               vels[i].2.abs())
        })
        .sum();
      println!("Total kinetic: {}", total);
    }
  }

  // Approach: simulate each axis separately,
  let pairs =
    [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
  let pi = &points_init;
  let mut repeats = [-1i64; 3];
  for x in 0..3 {
    let points_init = match x {
      0 => [pi[0].0, pi[1].0, pi[2].0, pi[3].0],
      1 => [pi[0].1, pi[1].1, pi[2].1, pi[3].1],
      2 => [pi[0].2, pi[1].2, pi[2].2, pi[3].2],
      _ => panic!(),
    };
    let mut points = points_init;
    let mut vels = [0; 4];

    for step in 0..1000000 {
      for (i, j) in &pairs {
        let (i, j) = (*i, *j);
        if points[i] > points[j] {
          vels[i] -= 1;
          vels[j] += 1;
        }
        if points[i] < points[j] {
          vels[i] += 1;
          vels[j] -= 1;
        }
      }
      for i in 0..4 {
        points[i] += vels[i];
      }
      if points == points_init && vels == [0; 4] {
        println!("Repeat {} after {}", x, step + 1);
        repeats[x] = step + 1;
        break;
      }
    }
  }
  // least common multiple
  let cycle = repeats[0] / gcd(repeats[0], repeats[1]) *
    repeats[1];
  let cycle = cycle / gcd(cycle, repeats[2]) *
    repeats[2];
  println!("Total cycle: {}", cycle);
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
/*
  // Approach: simulate each axis separately,
  // with periods of constant acceleration
  let mut vels: [[i64; 4]; 3] = [[0; 4]; 3];
  let mut points: [[i64; 4]; 3] = [[0; 4]; 3];
  for (i, p) in points_init.iter().enumerate() {
    points[0][i] = p.0;
    points[1][i] = p.1;
    points[2][i] = p.2;
  }
  let points_init = points;

  let mut time = 0;

  let pairs =
    [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
  for x in 0..3 {
    let mut points = &mut points[x];
    let mut vels = &mut vels[x];
    for step in 0..100 {
      let mut accels = [0; 4];
      for p in &pairs {
        if points[p.0] < points[p.1] {
          accels[p.0] += 1;
          accels[p.1] -= 1;
        }
        if points[p.0] > points[p.1] {
          accels[p.0] -= 1;
          accels[p.1] += 1;
        }
      }

      let delta_t: i64 = pairs
        .iter()
        .map(|(i, j)| -> i64 {
          let (i, j): (usize, usize) = (*i, *j);
          if points[i] == points[j] {
            if vels[i] == vels[j] {
              if accels[i] == accels[j] {
                return 100000;
              }
              return 2;
            }
            return 1;
          }
          let v = (vels[i] - vels[j]) as f64;
          let x = (points[i] - points[j]) as f64;
          if accels[i] == accels[j] {
            // 0 = x + vt
            if v * x >= 0.0 {
              return 100000;
            } else {
              return (-x / v).ceil() as i64;
            }
          }
          let a = (accels[i] - accels[j]) as f64;
          // 0 = x + (v + a/2)t + att
          let (a, b, c) = (a / 2.0, v + a / 2.0, x);
          let radical = b * b - 4.0 * a * c;
          if radical < 0.0 {
            return 100000;
          }
          let radical = radical.sqrt();
          let t0 = (-b + radical) / 2.0 / a;
          let t1 = (-b - radical) / 2.0 / a;
          if t0 > 0.0 {
            return t0.ceil() as i64;
          }
          if t1 > 0.0 {
            return t1.ceil() as i64;
          }
          return 100000;
        })
        .min()
        .unwrap();
      time += delta_t;
      for j in 0..4 {
        points[j] += vels[j] * delta_t +
          accels[j] * (delta_t * (delta_t + 1)) / 2;
        vels[j] += accels[j] * delta_t;
      }

      if *points == points_init[x] && *vels == [0; 4] {
        println!("FOUND {} REPEAT AT {}", x, time);
      }
    }
  }
}*/
