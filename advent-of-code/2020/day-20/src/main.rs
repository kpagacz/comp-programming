use regex::Regex;

fn parse_input(path: &str) -> (Vec<Vec<Vec<char>>>, Vec<i64>) {
    let mut tiles = Vec::new();
    let mut tile_values = Vec::new();

    let input = std::fs::read_to_string(path).unwrap();
    let index_regex = Regex::new(r"Tile (\d+):").unwrap();
    input.split("\n\n").for_each(|tile| {
        let first_line = tile.lines().next().unwrap();
        let captures = index_regex.captures(&first_line).unwrap();
        tile_values.push(captures.get(1).unwrap().as_str().parse::<i64>().unwrap());

        tiles.push(
            tile.lines()
                .skip(1)
                .map(|line| line.chars().collect())
                .collect(),
        );
    });

    (tiles, tile_values)
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Normal0,
    Normal90,
    Normal180,
    Normal270,
    Mirror0,
    Mirror90,
    Mirror180,
    Mirror270,
}

#[derive(Debug, Clone)]
struct TilePlacing<'a> {
    tile: &'a Vec<Vec<char>>,
    orientation: Orientation,
    idx: usize,
}

impl<'a> TilePlacing<'a> {
    fn new(tile: &Vec<Vec<char>>, orientation: Orientation, idx: usize) -> TilePlacing {
        TilePlacing {
            tile,
            orientation,
            idx,
        }
    }

    fn right_side_fits(&self, other: &TilePlacing) -> bool {
        let orientated_self = translate_tile(&self.tile, self.orientation);
        let orientated_other = translate_tile(&other.tile, other.orientation);
        orientated_self
            .iter()
            .zip(orientated_other.iter())
            .all(|(first, second)| first.last().unwrap() == second.first().unwrap())
    }

    fn bottom_side_fits(&self, other: &TilePlacing) -> bool {
        let orientated_self = translate_tile(&self.tile, self.orientation);
        let orientated_other = translate_tile(&other.tile, other.orientation);
        orientated_self.last() == orientated_other.first()
    }
}

fn translate_tile(tile: &Vec<Vec<char>>, new_orientation: Orientation) -> Vec<Vec<char>> {
    let mut translated_tile = vec![vec!['0'; tile.len()]; tile.len()];
    let tile_size = -1 + tile.len() as i32;

    for x in 0..(tile.len() as i32) {
        for y in 0..(tile.len() as i32) {
            let (mut new_x, mut new_y) = (x, y);
            match new_orientation {
                Orientation::Normal0 => {}
                Orientation::Normal90 => (new_x, new_y) = (y, -x + tile_size),
                Orientation::Normal180 => (new_x, new_y) = (-x + tile_size, -y + tile_size),
                Orientation::Normal270 => (new_x, new_y) = (-y + tile_size, x),
                Orientation::Mirror0 => (new_x, new_y) = (x, -y + tile_size),
                Orientation::Mirror90 => {
                    return translate_tile(
                        &translate_tile(tile, Orientation::Mirror0),
                        Orientation::Normal90,
                    )
                }
                Orientation::Mirror180 => {
                    return translate_tile(
                        &translate_tile(tile, Orientation::Mirror0),
                        Orientation::Normal180,
                    )
                }
                Orientation::Mirror270 => {
                    return translate_tile(
                        &translate_tile(tile, Orientation::Mirror0),
                        Orientation::Normal270,
                    )
                }
            }
            translated_tile[new_x as usize][new_y as usize] = tile[x as usize][y as usize];
        }
    }

    translated_tile
}

fn find_tiling<'a>(
    tiles: &'a Vec<Vec<Vec<char>>>,
    current_tiling: &mut Vec<TilePlacing<'a>>,
    already_placed: &mut Vec<bool>,
    placed_recently: usize,
    picture_side_size: &i32,
) -> Vec<TilePlacing<'a>> {
    if current_tiling.len() == tiles.len() {
        return current_tiling.clone();
    }
    use Orientation::*;
    let all_orientations: Vec<Orientation> = vec![
        Normal0, Normal90, Normal180, Normal270, Mirror0, Mirror90, Mirror180, Mirror270,
    ];

    for (idx, tile) in tiles.iter().enumerate() {
        if already_placed[idx] {
            continue;
        }
        for orientation in &all_orientations {
            let new_placement = TilePlacing::new(tile, *orientation, idx);
            let mut fits = true;

            let currently_placing = current_tiling.len();
            if currently_placing as i32 / picture_side_size > 0 {
                fits = fits
                    && current_tiling
                        .get((currently_placing as i32 - picture_side_size) as usize)
                        .unwrap()
                        .bottom_side_fits(&new_placement);
            }
            if currently_placing as i32 % picture_side_size != 0 {
                fits = fits
                    && current_tiling
                        .get((currently_placing as i32 - 1) as usize)
                        .unwrap()
                        .right_side_fits(&new_placement);
            }

            if fits {
                current_tiling.push(new_placement);
                already_placed[idx] = true;
                let fitting_tiling = find_tiling(
                    tiles,
                    current_tiling,
                    already_placed,
                    idx,
                    picture_side_size,
                );
                if fitting_tiling.len() > 0 {
                    return fitting_tiling;
                }
            }
        }
    }

    if current_tiling.is_empty() == false {
        current_tiling.pop();
    }
    already_placed[placed_recently] = false;
    vec![]
}

fn part1<'a>(
    tiles: &'a Vec<Vec<Vec<char>>>,
    tile_values: &Vec<i64>,
) -> (i64, Vec<TilePlacing<'a>>) {
    let picture_side = (tiles.iter().count() as f32).sqrt() as i32;
    let tile_side = tiles.iter().next().unwrap().iter().count();
    println!(
        "Picture side consists of {} tiles. Tile has side of size {}",
        &picture_side, &tile_side
    );

    let mut current_tiling = vec![];
    let mut already_placed = vec![false; tiles.len()];
    let tiling = find_tiling(
        &tiles,
        &mut current_tiling,
        &mut already_placed,
        0,
        &picture_side,
    );

    (
        [
            0,
            picture_side - 1,
            picture_side * (picture_side - 1),
            picture_side * picture_side - 1,
        ]
        .iter()
        .map(|idx| return tile_values[tiling.get(*idx as usize).unwrap().idx])
        .fold(1, std::ops::Mul::mul),
        tiling,
    )
}

fn trim_sides(tile: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    tile.drain(1..tile.len() - 1 as usize)
        .map(|mut line| line.drain(1..line.len() - 1 as usize).collect())
        .collect()
}

fn count_dragon_tiles(image: &Vec<Vec<char>>) -> i32 {
    let dragon_coordinates = [
        (1, 0),
        (1, 5),
        (1, 6),
        (1, 11),
        (1, 12),
        (1, 17),
        (1, 18),
        (1, 19),
        (0, 18),
        (2, 1),
        (2, 4),
        (2, 7),
        (2, 10),
        (2, 13),
        (2, 16),
    ];
    let mut dragon_tiles = 0;
    for (x, row) in image.iter().enumerate() {
        if x == 0 || x == image.len() - 1 {
            continue;
        }
        for (y, _) in row.iter().enumerate() {
            if y + 19 >= row.len() {
                continue;
            }
            if dragon_coordinates
                .iter()
                .all(|(dragon_x, dragon_y)| image[x + dragon_x][y + dragon_y] == '#')
            {
                dragon_tiles += dragon_coordinates.len() as i32;
            }
        }
    }
    dragon_tiles
}

fn part2(tiling: &Vec<TilePlacing>) -> i32 {
    let picture_side = (tiling.len() as f32).sqrt() as usize;
    let mut trimmed_tiles: Vec<Vec<Vec<char>>> = tiling
        .iter()
        .map(|tile_placement| translate_tile(tile_placement.tile, tile_placement.orientation))
        .map(|mut rotated_tile| trim_sides(&mut rotated_tile))
        .collect();
    let mut whole_image = vec![vec![]; picture_side * 8];
    trimmed_tiles
        .chunks_exact_mut(picture_side)
        .enumerate()
        .for_each(|(idx, row_tiles)| {
            row_tiles.iter_mut().for_each(|tile| {
                tile.iter_mut()
                    .enumerate()
                    .for_each(|(tile_row, row)| whole_image[idx * 8 + tile_row].append(row))
            });
        });

    let sharps: i32 = whole_image
        .iter()
        .flat_map(|c| c.iter().map(|c| if *c == '#' { 1 } else { 0 }))
        .sum();

    use Orientation::*;
    let all_orientations: Vec<Orientation> = vec![
        Normal0, Normal90, Normal180, Normal270, Mirror0, Mirror90, Mirror180, Mirror270,
    ];

    for orientation in all_orientations {
        let rotated_image = translate_tile(&whole_image, orientation);
        let dragon_tiles = count_dragon_tiles(&rotated_image);
        if dragon_tiles != 0 {
            return sharps - dragon_tiles;
        }
    }
    -1
}

fn main() {
    let path = "input";
    let (tiles, tile_values) = parse_input(path);
    let (answer, tiling) = part1(&tiles, &tile_values);
    println!("Part 1: {}", &answer);
    println!("Part 2: {}", part2(&tiling));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_translating_tiles() {
        let tile = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        use Orientation::*;
        for orientation in vec![
            Normal0, Normal90, Normal180, Normal270, Mirror0, Mirror90, Mirror180, Mirror270,
        ] {
            println!("{:?}", &orientation);
            translate_tile(&tile, orientation)
                .iter()
                .for_each(|line| println!("{}", line.iter().collect::<String>()));
        }
    }

    #[test]
    fn test_tile_fitting() {
        let tile = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let tile2 = vec![
            vec!['3', '2', '3'],
            vec!['6', '5', '6'],
            vec!['9', '8', '9'],
        ];
        let tile3 = vec![
            vec!['9', '8', '9'],
            vec!['6', '5', '6'],
            vec!['9', '8', '9'],
        ];
        let tile_placement = TilePlacing::new(&tile, Orientation::Normal0);
        let tile_placement2 = TilePlacing::new(&tile2, Orientation::Normal0);
        let tile_placement3 = TilePlacing::new(&tile3, Orientation::Normal0);

        assert!(tile_placement.right_side_fits(&tile_placement2));
        assert!(tile_placement.right_side_fits(&tile_placement) == false);
        assert!(tile_placement2.bottom_side_fits(&tile_placement3));
    }
}
