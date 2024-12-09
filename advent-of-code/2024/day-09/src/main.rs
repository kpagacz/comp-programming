// https://adventofcode.com/2024/day/9
fn parse_input(input: &str) -> Vec<i32> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i32))
        .collect()
}

fn part1(input: &str) -> usize {
    let mut checksum = 0;
    let disk = parse_input(input);
    let mut memory_fragments: Vec<_> = disk.chunks(2).map(|chunk| chunk[0]).collect();
    let total_fragments = memory_fragments.iter().sum::<i32>() as usize;
    let mut current_pos = 0;
    let mut disk_it = 0;
    let mut rev_fragments_it = memory_fragments.len() - 1;

    while current_pos < total_fragments {
        let (mem_block_size, gap_size) = (disk[disk_it], disk[disk_it + 1]);
        let forward_id = disk_it / 2;
        disk_it += 2;

        for _ in 0..mem_block_size {
            if current_pos >= total_fragments {
                break;
            }
            checksum += forward_id * current_pos;
            current_pos += 1;
        }

        for _ in 0..gap_size {
            if memory_fragments[rev_fragments_it] == 0 {
                rev_fragments_it -= 1;
            }
            if current_pos >= total_fragments {
                break;
            }

            checksum += rev_fragments_it * current_pos;
            memory_fragments[rev_fragments_it] -= 1;
            current_pos += 1;
        }
    }

    checksum
}

fn part2(input: &str) -> usize {
    let disk = parse_input(input);
    let mut new_disk = Vec::with_capacity(disk.len()); // size, id
    let files: Vec<_> = disk.chunks(2).map(|chunk| chunk[0]).collect();
    let mut moved = vec![false; files.len()];

    for (pos, chunk) in disk.chunks(2).enumerate() {
        if !moved[pos] {
            new_disk.push((chunk[0], pos));
        } else {
            new_disk.push((chunk[0], 0));
        }
        if chunk.len() == 1 {
            continue;
        }

        let mut fitting_file = files
            .iter()
            .enumerate()
            .rev()
            .find(|(file_id, &el)| !moved[*file_id] && el <= chunk[1] && *file_id > pos);
        let mut gap_left = chunk[1];

        while let Some((id, &size)) = fitting_file {
            // println!("Found: {fitting_file:?}");
            moved[id] = true;
            new_disk.push((size, id));
            gap_left -= size;
            if gap_left <= 0 {
                break;
            }
            fitting_file = files
                .iter()
                .enumerate()
                .rev()
                .find(|(file_id, &el)| !moved[*file_id] && el <= gap_left && *file_id > pos);
        }
        if gap_left > 0 {
            new_disk.push((gap_left, 0));
        }
        // println!("new disk {new_disk:?}");
    }

    let mut check_sum = 0;
    let mut it = 0;
    for (size, id) in new_disk {
        for _ in 0..size {
            check_sum += it * id;
            it += 1;
        }
    }

    check_sum
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");

    println!("Part 1 (test): {}", part1(test));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (test): {}", part2(test));
    println!("Part 2: {}", part2(input));
}
