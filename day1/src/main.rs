use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let elves_vector = read_input();
    /*
    # Figure out which elf brought the most calories
    for i, meals in enumerate(calories):
        for meal in meals:
            total_calories += meal
        if max_calories < total_calories:
            max_calories = total_calories
            max_elf = i
        elf_dict[i + 1] = [total_calories]
        total_calories = 0

     */
    let mut max_calories = 0;
    let mut max_elf = 0;
    let mut total_calories = 0;
    let mut elf_map:HashMap<i32, i32> = HashMap::new();
    for (i, meals) in elves_vector.iter().enumerate() {
        for meal in meals {
            total_calories += meal;
        }
        if max_calories < total_calories {
            max_calories = total_calories;
            max_elf = i;
        }
        elf_map.insert(i as i32, total_calories);
        total_calories = 0;
    }
    println!("Elf {} has the most caloric snacks with total calories {}", max_elf, max_calories);

    // Sort the map by converting to a vector
    let mut hash_vec: Vec<(&i32, &i32)> = elf_map.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));
    let mut top_3_calory_total = 0;
    for (i, elf) in hash_vec.iter().enumerate() {
        if i < 3 {
            top_3_calory_total += elf.1;
        }
    }
    println!("Top 3 elves have a total of {} calories", top_3_calory_total);
}

fn read_input() -> Vec<Vec<i32>> {
    let mut elves_vector = vec![];
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("../input1") {
        // Consumes the iterator, returns an (Optional) String
        let mut meal_vector = vec![];
        for line in lines {
            if let Ok(meal) = line {
                if meal.chars().count()  == 0 {
                    elves_vector.push(meal_vector.clone());
                    meal_vector.clear();
                    continue;
                }
                meal_vector.push(meal.parse::<i32>().unwrap());
            }
        }
        elves_vector.push(meal_vector.clone());
    } else {
        println!("ERROR", );
    }
    elves_vector
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
