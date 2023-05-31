use std::cmp::Ordering;
use std::fs;

#[derive(Debug)]
struct ElfData {
  elf_index: usize,
  total_calories: u32
}

fn get_elf_with_most_calorie(file_content: String) -> Option<ElfData> {
  let sum_of_calories_list: Vec<u32> = file_content
    .split("\n\n")
    .map(|e| {
      e.lines()
        .map(|c| c.parse::<u32>().unwrap_or(0))
        .sum::<u32>()
    })
    .collect();

  println!("{:#?}", sum_of_calories_list);

  let elf_with_most_calorie = sum_of_calories_list
    .iter()
    .enumerate()
    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
    .map(|(index, calorie)| {
      return ElfData {
        elf_index: index,
        total_calories: *calorie
      }
    });

  return elf_with_most_calorie;
}

pub fn main() {
  let file_contents = fs::read_to_string("./res/task1.txt");
  match file_contents {
    Ok(file_content) => {
      match get_elf_with_most_calorie(file_content) {
        None => {}
        Some(data) => {
          println!("Elf number {:?} is carrying most calories ({:?})", data.elf_index + 1, data.total_calories);
        }
      }
    }
    Err(_) => {
      panic!("cannot read file!");
    }
  }
}
