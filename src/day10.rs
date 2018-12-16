use regex::Regex;

#[derive(Debug)]
struct Light {
  x: i32,
  y: i32,
  dx: i32,
  dy: i32
}

impl Light {
  fn mv(&mut self) {
    self.x += self.dx;
    self.y += self.dy;
  }
  fn mv_back(&mut self) {
    self.x -= self.dx;
    self.y -= self.dy;
  }
}

pub fn compute(input: &[String]) {
  let regex = Regex::new(r"^position=<([0-9 \-]+), ([0-9 \-]+)> velocity=<([0-9 \-]+), ([0-9 \-]+)>$").unwrap();
  let mut lights: Vec<Light> = input.iter().map(|s| {
    let loc = regex.captures(s).unwrap();
    let x: i32 = loc.get(1).unwrap().as_str().trim().parse().unwrap();
    let y: i32 = loc.get(2).unwrap().as_str().trim().parse().unwrap();
//    let y: i32 = -y;
    let dx: i32 = loc.get(3).unwrap().as_str().trim().parse().unwrap();
    let dy: i32 = loc.get(4).unwrap().as_str().trim().parse().unwrap();
//    let dy: i32 = -dy;
    Light {x, y, dx, dy}
  }).collect();
//  println!("{:?}", lights);
  let mut dx = i32::max_value();
  let mut dy = i32::max_value();
  let mut seconds = 0;
  loop {
    lights.iter_mut().for_each(Light::mv);
    seconds += 1;
//  println!("{:?}", lights);
    let xmin: i32 = lights.iter().map(|l| l.x).min().unwrap();
    let ymin: i32 = lights.iter().map(|l| l.y).min().unwrap();
    let xmax: i32 = lights.iter().map(|l| l.x).max().unwrap();
    let ymax: i32 = lights.iter().map(|l| l.y).max().unwrap();
    let new_dx = xmax - xmin;
    let new_dy = ymax - ymin;
    if new_dx > dx || new_dy > dy {
//      println!("{},{},{},{} {},{} {},{}", xmin, ymin, xmax, ymax, new_dx, dx, new_dy, dy);
      break;
    }
    dx = new_dx;
    dy = new_dy;
  }
  lights.iter_mut().for_each(Light::mv_back);
  seconds -= 1;
  let xmin: i32 = lights.iter().map(|l| l.x).min().unwrap();
  let ymin: i32 = lights.iter().map(|l| l.y).min().unwrap();
  let xmax: i32 = lights.iter().map(|l| l.x).max().unwrap();
  let ymax: i32 = lights.iter().map(|l| l.y).max().unwrap();
  for y in ymin..=ymax {
    for x in xmin..=xmax {
      if lights.iter().any(|l|l.x == x && l.y == y) {
        print!("#");
      } else {
        print!(" ");
      }
    }
    println!()
  }
  println!("Seconds took: {}", seconds);
}