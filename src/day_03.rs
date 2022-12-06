pub fn main(puzzle_input: &str) {
    let rucksacks: Vec<&str> = puzzle_input.split("\n").collect();
    let length = rucksacks.len();

    let mut misplaced_item_sum: u16 = 0;
    for rucksack in &rucksacks {
        if rucksack.len() % 2 == 0 {
            let compartment_1: &[u8] = (&rucksack[..rucksack.len() / 2]).as_bytes();
            let compartment_2: &[u8] = (&rucksack[rucksack.len() / 2..]).as_bytes();

            misplaced_item_sum += compare_items(compartment_1, compartment_2);
        } else {
            panic!("Unexpected rucksack contents")
        }
    }

    let mut group_sum: u16 = 0;
    let mut i: usize = 0;
    while i < (length) {
        let a: &[u8] = rucksacks[i].as_bytes();
        let b: &[u8] = rucksacks[i + 1].as_bytes();
        let c: &[u8] = rucksacks[i + 2].as_bytes();

        group_sum += compare_group(&[a, b, c]);

        i += 3;
    }

    println!("Sum of misplaced items: {}", misplaced_item_sum);
    println!("Sum of group: {}", group_sum);
}

fn compare_items(compartment_1: &[u8], compartment_2: &[u8]) -> u16 {
    for i in 0..compartment_1.len() {
        for j in 0..compartment_2.len() {
            if compartment_1[i] == compartment_2[j] {
                return get_char_value(&(compartment_1[i] as char));
            }
        }
    }

    return 0;
}

fn compare_group(elfs: &[&[u8]; 3]) -> u16 {
    for i in 0..elfs[0].len() {
        for j in 0..elfs[1].len() {
            for k in 0..elfs[2].len() {
                if elfs[0][i] == elfs[1][j] && elfs[1][j] == elfs[2][k] {
                    return get_char_value(&(elfs[0][i] as char));
                }
            }
        }
    }

    return 0;
}

fn get_char_value(chr: &char) -> u16 {
    let mut res: u16 = 0;
    if chr.is_uppercase() {
        res += 26;
    }

    let lower_chr: char = chr.to_ascii_lowercase();
    res += lower_chr as u16;
    res -= 96;

    return res;
}