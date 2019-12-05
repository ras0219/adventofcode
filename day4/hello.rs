#![allow(unused)]
#![allow(unused_imports)]

use std::io::{self, Write};
use std::fs::File;
use std::str::FromStr;
use std::io::{BufRead, BufReader};

fn main() {
  let text_range = "183564-657474";
  let mut cur: [u8; 6] = [1, 8, 3, 5, 6, 4];

  let valid = |a, b, c, d, e, f| -> i32 {
    if a != b && b != c && c != d && d != e && e != f {
      return 0;
    }
    let n = a * 100000 + b * 10000 + c * 1000 +
      d * 100 + e * 10 + f;
    if n > 657474 || n < 183564 {
      return 0;
    }
    return 1;
  };

  let valid2 = |a, b, c, d, e, f| -> i32 {
    let arr: [u32; 6] = [a, b, c, d, e, f];
    let left = a == b && b != c;
    let right = d != e && e == f;
    let middle = (1..4).any(|i| {
      arr[i - 1] != arr[i] && arr[i] == arr[i + 1] &&
        arr[i + 1] != arr[i + 2]
    });
    if !left && !right && !middle {
      return 0;
    }
    let n = a * 100000 + b * 10000 + c * 1000 +
      d * 100 + e * 10 + f;
    if n > 657474 || n < 183564 {
      return 0;
    }
    return 1;
  };

  let mut count = 0;
  let mut count2 = 0;
  for a in 1..7 {
    for b in a..10 {
      for c in b..10 {
        for d in c..10 {
          for e in d..10 {
            for f in e..10 {
              count += valid(a, b, c, d, e, f);
              count2 += valid2(a, b, c, d, e, f);
            }
          }
        }
      }
    }
  }
  print!("valid={}\n", count);
  print!("valid2={}\n", count2);
}
// vim: set tabstop=2 shiftwidth=2 expandtab
