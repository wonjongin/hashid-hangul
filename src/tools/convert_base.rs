/// Convert 10 base number to 2350 base number
/// Return a Vec<usize> value which contains each digit data
///
/// # Arguments
///
/// * `integer` - usize value 10 base number
pub fn convert_10_to_2350(integer: usize) -> Vec<usize> {
  let mut mid_int = integer;
  let mut i = 1;
  let max_digit = loop {
    let d = 2350_usize.pow(i as u32);
    if (integer / d) == 0 {
      break i;
    } else {
      i = i + 1;
    }
  };
  // let mut res = vec![0; (max_digit + 1) as usize];
  let mut res = Vec::new();
  for i in (1..(max_digit)).rev() {
    let p = 2350_usize.pow(i);
    let a = mid_int / p;
    mid_int = mid_int - a * p;
    res.push(a);
  }
  res.push(mid_int);
  res
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn convert_10_to_2350_test() {
    assert_eq!(convert_10_to_2350(5_529_549), vec![1, 2, 2349]);
  }
}
