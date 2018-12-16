pub fn compute() {
  //  compute_solution(18);
  //  compute_solution(42);
  compute_solution(1723);
}

fn compute_solution(serial_id: i32) {
  let mut power_levels = Vec::with_capacity(300);
  for y in 0..300 {
    power_levels.push(vec![0i32; 300]);
    for x in 0..300 {
      power_levels[y][x] = power_level(x as i32, y as i32, serial_id);
    }
    //    println!("{:?}", &power_levels[y]);
  }
  let mut max_total = 0;
  let mut coord = (500, 500);
  for y in 0..300 - 2 {
    for x in 0..300 - 2 {
      let total = sum_nxn(x, y, 3, &power_levels);
      if total > max_total {
        max_total = total;
        coord = (x, y);
      }
    }
  }
  println!("Max is {} at ({},{})", max_total, coord.0, coord.1);

  let mut max_total = 0;
  let mut coord = (500, 500, 500);
  for n in 1..=300 {
    //  println!("n = {}", n);
    for y in 0..300 - n + 1 {
      for x in 0..300 - n + 1 {
        let total = sum_nxn(x, y, n, &power_levels);
        if total > max_total {
          max_total = total;
          coord = (x, y, n);
        }
      }
    }
    if coord.2 + 20 < n {
      // We're not going to do better at higher values
      break;
    }
  }
  println!("Max is {} at ({},{},{})", max_total, coord.0, coord.1, coord.2);
}

fn sum_nxn(x: usize, y: usize, n: usize, v: &Vec<Vec<i32>>) -> i32 {
  let mut total = 0;
  for y in y..y + n {
    for x in x..x + n {
      total += v[y][x];
    }
  }
  total
}

fn power_level(x: i32, y: i32, serial_id: i32) -> i32 {
  let rack_id = x + 10;
  let mut power_level = rack_id * y;
  power_level += serial_id;
  power_level *= rack_id;
  power_level /= 100;
  power_level %= 10;
  return power_level - 5;
}

#[test]
fn test_power_level() {
  assert_eq!(power_level(3, 5, 8), 4);
}
