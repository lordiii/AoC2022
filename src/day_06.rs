pub fn main(puzzle_input: &str) {
    let bytes = puzzle_input.as_bytes();

    println!("Start of packet: {}", process_buffer(bytes, 4));
    println!("Start of message: {}", process_buffer(bytes, 14));
}

fn process_buffer(buffer: &[u8], size: usize) -> usize {
    let mut pos: usize = 0;
    while pos < buffer.len() {
        if !find_match(&buffer[pos..(pos + size)]) {
            break;
        }

        pos += 1;
    }

    return pos + size;
}

fn find_match(buffer: &[u8]) -> bool {
    for i in 0..buffer.len() {
        for j in (i + 1)..buffer.len() {
            if buffer[i] == buffer[j] {
                return true;
            }
        }
    }

    return false;
}
