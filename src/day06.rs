use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

pub fn compute(input: &[String]) {
  let mut xmax = 0;
  let mut ymax = 0;
  let points: Vec<Point> = input
    .iter()
    .map(|s| {
      let mut split = s.split(", ");
      let x: i32 = split.next().unwrap().parse().unwrap();
      let y: i32 = split.next().unwrap().parse().unwrap();
      xmax = x.max(xmax);
      ymax = y.max(ymax);
      Point { x, y }
    })
    .collect();
  println!("{}, {}, {}", points.len(), xmax, ymax);
  let xsize = (xmax + 10) as usize;
  let ysize = (ymax + 10) as usize;
  let mut grid = vec![];
  for x in 0..xsize {
    grid.push(vec![-1i16; ysize]);
    for y in 0..ysize {
      let gp = Point { x: x as i32, y: y as i32 };
      let mut min_dist = u32::max_value();
      let mut min_point: isize = -1;
      for (i, p) in points.iter().enumerate() {
        let dist = man_dist(p, &gp);
        if dist < min_dist {
          min_dist = dist;
          min_point = i as isize;
        } else if dist == min_dist {
          min_point = -1;
        }
      }
      grid[x][y] = min_point as i16;
    }
  }
  let mut blacklist = HashSet::<i16>::new();
  //blacklist x's
  for y in 0..ysize {
    blacklist.insert(grid[0][y]);
    blacklist.insert(grid[xsize - 1][y]);
  }
  //blacklist y's
  for x in 0..xsize {
    blacklist.insert(grid[x][0]);
    blacklist.insert(grid[x][ysize - 1]);
  }
  //blacklist.iter().for_each(|f|println!("bl: {}", f));
  println!("-1, {}", count_instances(&grid, -1));
  let mut max_area = 0;
  let mut max_point: isize = -1;
  for (i, _p) in points.iter().enumerate() {
    if !blacklist.contains(&(i as i16)) {
      let area = count_instances(&grid, i as i16);
      //println!("{}, {}, {}, {}", i, p.x, p.y, area);
      if area > max_area {
        max_area = area;
        max_point = i as isize;
      }
    }
  }
  let p = points[max_point as usize];
  println!("Max area is {} for point {}, {} ({})", max_area, p.x, p.y, max_point);

  //part 2
  let mut grid = vec![];
  for x in 0..xsize {
    grid.push(vec![100_000u32; ysize]);
    for y in 0..ysize {
      let gp = Point { x: x as i32, y: y as i32 };
      let mut total_distance = 0;
      for p in &points {
        total_distance += man_dist(p, &gp);
      }
      grid[x][y] = total_distance;
    }
  }
  let y_counts: Vec<usize> = grid.iter().map(|f| f.iter().filter(|v| **v < 10000).count()).collect();
  let total_area: usize = y_counts.iter().sum();
  println!("Total area less than 10000 is {}", total_area);
}

fn man_dist(p1: &Point, p2: &Point) -> u32 {
  ((p1.x - p2.x).abs() + (p1.y - p2.y).abs()) as u32
}

fn count_instances(grid: &Vec<Vec<i16>>, val: i16) -> usize {
  let y_counts: Vec<usize> = grid.iter().map(|f| f.iter().filter(|v| **v == val).count()).collect();
  y_counts.iter().sum()
}
