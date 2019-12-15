pub mod day2 {
  fn str_to_vec(text: &str) -> Vec<u32> {
    text.split(',').map(|s| s.parse().unwrap()).collect()
  }

  pub fn exec(mut mem: Vec<u32>) -> u32 {
    let mut ip: usize = 0;
    loop {
      let opcode = mem[ip];
      match opcode {
        1 => {
          let addr1 = mem[ip + 1] as usize;
          let addr2 = mem[ip + 2] as usize;
          let dest_addr = mem[ip + 3] as usize;
          let sum = mem[addr1] + mem[addr2];
          mem[dest_addr] = sum;
          ip += 4;
        }
        2 => {
          let addr1 = mem[ip + 1] as usize;
          let addr2 = mem[ip + 2] as usize;
          let dest_addr = mem[ip + 3] as usize;
          let product = mem[addr1] * mem[addr2];
          mem[dest_addr] = product;
          ip += 4;
        }
        99 => break,
        _ => {
          println!("Unknown opcode: {}", opcode);
          break;
        }
      }
    }

    mem[0]
  }

  pub fn run(text: &str) -> u32 {
    let mem = str_to_vec(text);

    exec(mem)
  }

  pub fn run_input(text: &str, noun: u32, verb: u32) -> u32 {
    let mut mem = str_to_vec(text);
    mem[1] = noun;
    mem[2] = verb;
    exec(mem)
  }
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

  #[test]
  fn test_process() {
    assert_eq!(day2::run("1,9,10,3,2,3,11,0,99,30,40,50"), 3500);
    assert_eq!(day2::run("1,0,0,0,99"), 2);
    assert_eq!(day2::run("2,3,0,3,99"), 2);
    assert_eq!(day2::run("1,1,1,4,99,5,6,0,99"), 30);
  }
}
