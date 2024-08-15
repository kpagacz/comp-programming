type Layer = Vec<Vec<u8>>;
type Image = Vec<Layer>;

fn parse(width: usize, height: usize, input: &str) -> Image {
    let input = input.as_bytes();

    input
        .chunks(width * height)
        .map(|layer| layer.chunks(width).map(|chunk| chunk.to_owned()).collect())
        .collect()
}

fn count_digit_in_layer(digit: u8, layer: &[Vec<u8>]) -> usize {
    layer
        .iter()
        .map(|row| row.iter().filter(|&&c| c == digit).count())
        .sum()
}

fn part1(input: &str, width: usize, height: usize) -> usize {
    let mut image = parse(width, height, input);
    image.pop();

    let fewest_zeros_layer = image
        .iter()
        .enumerate()
        .map(|(pos, layer)| (count_digit_in_layer(b'0', layer), pos))
        .min()
        .unwrap()
        .1;

    count_digit_in_layer(b'1', &image[fewest_zeros_layer])
        * count_digit_in_layer(b'2', &image[fewest_zeros_layer])
}

const BLACK: u8 = b'0';
const WHITE: u8 = b'1';
const TRANSPARENT: u8 = b'2';
fn part2(input: &str, width: usize, height: usize) {
    let mut image = parse(width, height, input);
    image.pop();

    let layer = &image[0];
    let merged = image.iter().fold(
        vec![vec![TRANSPARENT; layer[0].len()]; layer.len()],
        |mut merged, layer| {
            for row in 0..layer.len() {
                for col in 0..layer[0].len() {
                    if merged[row][col] == TRANSPARENT {
                        merged[row][col] = match layer[row][col] {
                            BLACK => b'H',
                            WHITE => b' ',
                            _ => TRANSPARENT,
                        };
                    }
                }
            }
            merged
        },
    );

    for row in merged {
        println!("{}", String::from_utf8(row).unwrap());
    }
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");
    let test2 = include_str!("../test2");

    println!("Part 1 (test): {}", part1(test, 3, 2));
    println!("Part 1: {}", part1(input, 25, 6));

    println!("Part 2 (test2):");
    part2(test2, 2, 2);

    println!("Part 2:");
    part2(input, 25, 6);
}
