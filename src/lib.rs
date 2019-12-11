pub fn test() {
  println!("Hello world!");
}

pub mod day1 {
  pub fn compute_fuel(mass: u32) -> u32 {
    std::cmp::max(mass / 3, 2) - 2
  }

  pub fn compute_fuel2(mut mass: u32) -> u32 {
    let mut sum: u32 = 0;

    while mass > 0 {
      let fuel = compute_fuel(mass);
      sum += fuel;
      mass = fuel;
    }
    sum
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compute_fuel() {
    assert_eq!(day1::compute_fuel(12), 2);
    assert_eq!(day1::compute_fuel(14), 2);
    assert_eq!(day1::compute_fuel(1969), 654);
    assert_eq!(day1::compute_fuel(100756), 33583);
  }

  #[test]
  fn test_compute_fuel2() {
    assert_eq!(day1::compute_fuel2(14), 2);
    assert_eq!(day1::compute_fuel2(1969), 966);
    assert_eq!(day1::compute_fuel2(100756), 50346);
  }
}
