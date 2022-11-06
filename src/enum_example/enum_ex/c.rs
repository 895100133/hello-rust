#![allow(dead_code)]

enum Number {
  Zero,
  One,
  Two,
}

enum Color {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff,
}

pub fn c_fn() {
  use Number::{Zero, One};
  println!("zero is {}", Zero as i32);
  println!("one is {}", One as i32);

  use Color::{ Red, Blue };
  println!("roses ars #{:06x}", Red as i32);
  println!("violets ars #{:06x}", Blue as i32);
}