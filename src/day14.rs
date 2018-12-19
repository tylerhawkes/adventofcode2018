
pub fn compute() {

  let mut elf_1 = 0;
  let mut elf_2 = 1;
  let mut recipes = vec![3, 7];

  while recipes.len() < 640441 + 10 {
    let elf_1_recipe = recipes[elf_1];
    let elf_2_recipe = recipes[elf_2];
    let sum = elf_1_recipe + elf_2_recipe;
    let right = sum % 10;
    let left = sum / 10;
    if left > 0 {
      recipes.push(left);
    }
    recipes.push(right);
    elf_1 = step(elf_1, elf_1_recipe + 1, recipes.len());
    elf_2 = step(elf_2, elf_2_recipe + 1, recipes.len());
//    println!("elves: {}, {}, recipes: {:?}", elf_1, elf_2, recipes);
  }
  println!("10 recipes after 640441: {:?}", &recipes[640441..]);

  //part 2
  let mut elf_1 = 0;
  let mut elf_2 = 1;
  let mut recipes :Vec<u8> = vec![3, 7];

  let mut iter = 0;
  let seq: [u8; 6] = [6,4,0,4,4,1];
  let mut slice_start = 0;

  loop {
    let mut push_twice = false;
//    if iter % 100000 == 0 {
//      println!("On iter {}", iter);
//    }
    let elf_1_recipe = recipes[elf_1];
    let elf_2_recipe = recipes[elf_2];
    let sum = elf_1_recipe + elf_2_recipe;
    let right = sum % 10;
    let left = sum / 10;
    if left > 0 {
      recipes.push(left);
      push_twice = true;
    }
    recipes.push(right);
    elf_1 = step(elf_1, elf_1_recipe as isize + 1, recipes.len());
    elf_2 = step(elf_2, elf_2_recipe as isize + 1, recipes.len());
//    println!("elves: {}, {}, recipes: {:?}", elf_1, elf_2, recipes);
    iter += 1;
    slice_start = (recipes.len()-6.min(recipes.len()));
    if &recipes[slice_start..] == &seq {
      println!("recipes before [6,4,0,4,4,1]: {:?}", recipes.len() - 6);
      break;
    }
    if push_twice && slice_start > 0 && &recipes[slice_start-1..slice_start + 5] == &seq {
      println!("recipes before [6,4,0,4,4,1]: {:?}", recipes.len() - 7);
      break;
    }
  }

  println!("recipes ({}) last 10: {:?}", recipes.len(), &recipes[recipes.len() - 10..]);
} // 20174746

fn step(start: usize, step: isize, len: usize) -> usize {
  let i = start as isize + step;
  let len = len as isize;
  let v = (((i as isize % len) + len) % len) as usize;
  //  println!("{}, {}, {}, {}", start, step, len, v);
  v
}