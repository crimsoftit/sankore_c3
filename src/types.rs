/*
primitive data types in rust
----------------------------
  * integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
  * float data types: f32, f64
  * booleans: (bool)
  * characters (char)
  * tuples
  * arrays
*/

pub fn run () {
    let age: u32 = 45;
    let salary = 3120.55;
    let karakta = '\u{1F600}';

    println!("age {} >= {} {}", age, salary, karakta);
}