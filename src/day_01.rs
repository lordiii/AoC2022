pub fn main(puzzle_input: &str) {
    let elves: Vec<&str> = puzzle_input.split("\n\n").collect();

    let mut max_calories: [u32; 3] = [0; 3];
    let mut max_elf: [u16; 3] = [0; 3];

    let mut i: u16 = 1;

    for elf in elves {
        let calories: u32 = extract_elf_calories(elf);

        if calories > max_calories[2] {
            max_calories[0] = max_calories[1];
            max_calories[1] = max_calories[2];
            max_calories[2] = calories;

            max_elf[0] = max_elf[1];
            max_elf[1] = max_elf[2];
            max_elf[2] = i;
        } else if calories > max_calories[1] {
            max_calories[0] = max_calories[1];
            max_calories[1] = calories;

            max_elf[0] = max_elf[1];
            max_elf[1] = i;
        } else if calories > max_calories[0] {
            max_calories[0] = calories;

            max_elf[0] = i;
        }

        i += 1;
    }

    println!("Elf {}: {} calories", max_elf[2], max_calories[2]);
    println!("Elf {}: {} calories", max_elf[1], max_calories[1]);
    println!("Elf {}: {} calories", max_elf[0], max_calories[0]);

    println!("Total : {} calories", max_calories[2] + max_calories[1] + max_calories[0])
}

fn extract_elf_calories(elf: &str) -> u32 {
    let mut result: u32 = 0;

    let elves: Vec<&str> = elf.split_whitespace().collect();
    for calorie in elves {
        result += calorie.parse::<u32>().unwrap();
    }

    return result;
}