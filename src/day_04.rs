pub fn main(puzzle_input: &String) {
    let pairs: Vec<&str> = puzzle_input.split_whitespace().collect();

    let mut overlapping: u16 = 0;
    let mut touching: u16 = 0;
    for pair in pairs {
        let elves: Vec<&str> = pair.split(",").collect();

        let compare_results = &compare_pair(&[elves[0], elves[1]]);
        if compare_results[0] {
            overlapping += 1;
        }

        if compare_results[1] {
            touching += 1;
        }
    }

    println!("Overlapping: {}", overlapping);
    println!("Touching: {}", touching);
}

fn compare_pair(elves: &[&str; 2]) -> [bool; 2] {
    let elf_1: [&str; 2] = vec_to_array(elves[0].split("-").collect());
    let elf_2: [&str; 2] = vec_to_array(elves[1].split("-").collect());

    let elf_1_num: [u16; 2] = [elf_1[0].parse::<u16>().unwrap(), elf_1[1].parse::<u16>().unwrap()];
    let elf_2_num: [u16; 2] = [elf_2[0].parse::<u16>().unwrap(), elf_2[1].parse::<u16>().unwrap()];


    return [
        (elf_1_num[0] >= elf_2_num[0] && elf_1_num[1] <= elf_2_num[1]) || (elf_2_num[0] >= elf_1_num[0] && elf_2_num[1] <= elf_1_num[1]),
        (elf_1_num[1] >= elf_2_num[0] && elf_1_num[0] <= elf_2_num[1])
    ];
}

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into().unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}