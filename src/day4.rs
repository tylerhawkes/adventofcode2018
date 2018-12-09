use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::collections::HashMap;

struct Observation {
  guard_id: u16,
  asleep_minutes: Vec<u64>,
}

impl Observation {
  fn new(guard_id: u16) -> Self {
    let asleep_minutes = vec![0u64; 60];
    Observation { guard_id, asleep_minutes }
  }

  fn state_change(&mut self, state: u64, minute: usize) {
    for i in minute..self.asleep_minutes.len() {
      self.asleep_minutes[i] = state;
    }
  }
}

impl Display for Observation {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    write!(f, "{}: ", self.guard_id)?;
    for i in &self.asleep_minutes {
      write!(f, "{}", i)?;
    }
    Ok(())
  }
}

pub fn compute(input: &[String]) {
  let mut records = Vec::from(input);
  records.sort();
  let mut obs: Vec<Observation> = vec![];
  let mut id: u16;
  let mut current_observation = Observation::new(0);
  for record in records {
    if record.ends_with("begins shift") {
      let mut s = record[26..].split(' ');
      id = s.next().unwrap().parse().unwrap();
      obs.push(current_observation);
      current_observation = Observation::new(id);
    } else {
      let minute = record[15..17].to_string();
      let minute = minute.parse().unwrap();
      if record.ends_with("falls asleep") {
        current_observation.state_change(1, minute);
      } else {
        // wakes up
        current_observation.state_change(0, minute);
      }
    }
  }
  obs.remove(0);
  obs.sort_by_key(|o| o.guard_id);

  let mut m = HashMap::<u16, Vec<Vec<u64>>>::new();

  for o in &obs {
    //println!("{}", o);
    let v = m.get_mut(&o.guard_id);
    match v {
      Some(vec) => {
        vec.push(o.asleep_minutes.clone());
      }
      None => {
        let mut v = vec![];
        v.push(o.asleep_minutes.clone());
        m.insert(o.guard_id, v);
      }
    }
  }

  let mut max_asleep = 0;
  let mut max_id = 0;
  for (k, v) in &m {
    let total_asleep: Vec<u64> = v.iter().map(|v| v.iter().sum()).collect();
    let total_asleep: u64 = total_asleep.iter().sum();
    if total_asleep > max_asleep {
      max_asleep = total_asleep;
      max_id = *k;
    }
  }
  //println!("{}, {}", max_id, max_asleep);

  let v = m.get(&max_id).unwrap();
  let mut max_asleep_on_minute = 0;
  let mut max_asleep_minute: u64 = 0;
  for minute in 0..60 {
    let total_asleep: u64 = v.iter().map(|v|v[minute]).sum();
    if total_asleep > max_asleep_on_minute {
      max_asleep_minute = minute as u64;
      max_asleep_on_minute = total_asleep;
    }
  }
  println!("Guard {} was asleep the most on minute {} for a hash of {}", max_id, max_asleep_minute, max_id as u64 * max_asleep_minute);

  //part2
  let mut max_guard = 0;
  let mut max_minute = 0;
  let mut max_asleep = 0;
  for minute in 0..60 {
    for (guard, days) in m.iter() {
      let asleep_freq: u64 = days.iter().map(|f|f[minute]).sum();
      if asleep_freq > max_asleep {
        max_guard = *guard;
        max_minute = minute;
        max_asleep = asleep_freq;
      }
    }
  }

  println!("Guard {} was most frequently asleep on minute {} ({} times) for a hash of {}", max_guard, max_minute, max_asleep, max_minute * max_guard as usize)
}
