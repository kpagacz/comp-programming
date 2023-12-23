use std::collections::BTreeMap;
type Grid = Vec<Vec<char>>;

fn parse_input(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let mut answer = vec![vec!['#'; grid[0].len()]; grid.len() + 2];
    for i in 1..answer.len() - 1 {
        for j in 0..answer[0].len() {
            answer[i][j] = grid[i - 1][j];
        }
    }
    answer
}

fn find_start(grid: &Grid) -> (usize, usize) {
    let starting_row = 1;
    (
        starting_row,
        grid[starting_row]
            .iter()
            .enumerate()
            .find(|(_, &c)| c == '.')
            .unwrap()
            .0,
    )
}

const NEIGHBOURS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
const WALL: char = '#';
const UP: char = '^';
const RIGHT: char = '>';
const LEFT: char = '<';
const DOWN: char = 'v';
use std::collections::BTreeSet;
fn find_longest_hike(grid: &Grid) -> usize {
    let start = find_start(grid);

    fn longest_hike(
        pos: (usize, usize),
        walked_so_far: usize,
        grid: &Grid,
        seen: &mut BTreeSet<(usize, usize)>,
    ) -> usize {
        // End condition - arrived at the last row
        let ending_row = grid.len() - 2;
        if pos.0 == ending_row {
            return walked_so_far;
        }

        seen.insert(pos);
        let max_hike = if [UP, RIGHT, LEFT, DOWN].contains(&grid[pos.0][pos.1]) {
            let next_pos = match grid[pos.0][pos.1] {
                UP => (pos.0 - 1, pos.1),
                DOWN => (pos.0 + 1, pos.1),
                LEFT => (pos.0, pos.1 - 1),
                RIGHT => (pos.0, pos.1 + 1),
                _ => unreachable!(),
            };
            if grid[next_pos.0][next_pos.1] != WALL && !seen.contains(&next_pos) {
                longest_hike(next_pos, walked_so_far + 1, grid, seen)
            } else {
                0
            }
        } else {
            let possible_destinations = NEIGHBOURS
                .iter()
                .map(|(delta_x, delta_y)| (pos.0 as isize + delta_x, pos.1 as isize + delta_y))
                .map(|(x, y)| (x as usize, y as usize))
                .filter(|&(x, y)| grid[x][y] != WALL && !seen.contains(&(x, y)))
                .collect::<Vec<_>>();

            possible_destinations
                .into_iter()
                .map(|next_pos| longest_hike(next_pos, walked_so_far + 1, grid, seen))
                .max()
                .unwrap_or(0)
        };

        seen.remove(&pos);
        max_hike
    }

    let mut seen = BTreeSet::new();
    longest_hike(start, 0, grid, &mut seen)
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    find_longest_hike(&grid)
}

// PART 2
// Returns a tuple of 4 things:
// * vector of node coordinates
// * position of the start node in the above vector
// * position of the end node in the above vector
// * vector representing the graph, which i-th element contains a vector of tuples with (destination, distance)
fn create_graph(
    grid: &Grid,
    start_node: (usize, usize),
) -> (Vec<(usize, usize)>, usize, usize, Vec<Vec<(usize, usize)>>) {
    fn is_junction(
        point: (usize, usize),
        grid: &Grid,
        seen: &mut BTreeSet<(usize, usize)>,
    ) -> bool {
        NEIGHBOURS
            .iter()
            .map(|(delta_x, delta_y)| (point.0 as isize + delta_x, point.1 as isize + delta_y))
            .map(|(x, y)| (x as usize, y as usize))
            .filter(|&(x, y)| grid[x][y] != WALL && !seen.contains(&(x, y)))
            .count()
            > 1
    }

    type Point = (usize, usize);
    fn find_junctions(
        point: (usize, usize),
        last_junction: (usize, usize),
        walked_so_far: usize,
        graph: &mut BTreeMap<(Point, Point), usize>,
        grid: &Grid,
        seen: &mut BTreeSet<(usize, usize)>,
        junctions: &mut BTreeSet<(usize, usize)>,
    ) {
        if is_junction(point, grid, seen) || point.0 == grid.len() - 2 {
            graph.insert((last_junction, point), walked_so_far);
            if !junctions.contains(&point) {
                junctions.insert(point);
                NEIGHBOURS
                    .iter()
                    .map(|(delta_x, delta_y)| {
                        (point.0 as isize + delta_x, point.1 as isize + delta_y)
                    })
                    .map(|(x, y)| (x as usize, y as usize))
                    .filter(|&(x, y)| grid[x][y] != WALL && !seen.contains(&(x, y)))
                    .for_each(|new_point| {
                        let mut new_seen = BTreeSet::from([point]);
                        find_junctions(new_point, point, 1, graph, grid, &mut new_seen, junctions);
                    });
            }
        } else {
            seen.insert(point);
            let next_point = NEIGHBOURS
                .iter()
                .map(|(delta_x, delta_y)| (point.0 as isize + delta_x, point.1 as isize + delta_y))
                .map(|(x, y)| (x as usize, y as usize))
                .find(|&(x, y)| grid[x][y] != WALL && !seen.contains(&(x, y)))
                .unwrap_or_else(|| panic!("There is a dead end at: {point:?}"));
            find_junctions(
                next_point,
                last_junction,
                walked_so_far + 1,
                graph,
                grid,
                seen,
                junctions,
            );
        }
    }

    let mut seen = BTreeSet::from([start_node]);
    let mut junctions = BTreeSet::from([start_node]);
    let mut graph = BTreeMap::new();
    find_junctions(
        (start_node.0 + 1, start_node.1),
        start_node,
        1,
        &mut graph,
        grid,
        &mut seen,
        &mut junctions,
    );

    let mut mut_grid = grid.clone();
    for junction in &junctions {
        mut_grid[junction.0][junction.1] = '@';
    }

    let mut nodes = graph
        .keys()
        .flat_map(|&(first, second)| vec![first, second])
        .collect::<Vec<_>>();
    nodes.sort_unstable();
    nodes.dedup();

    let start_node_pos = nodes
        .iter()
        .enumerate()
        .find(|(_, &node)| node.0 == 1)
        .unwrap()
        .0;
    let end_node_pos = nodes
        .iter()
        .enumerate()
        .find(|(_, &node)| node.0 == grid.len() - 2)
        .unwrap()
        .0;

    let mut answer = vec![Vec::<(usize, usize)>::new(); nodes.len()];
    for i in 0..nodes.len() {
        for j in i..nodes.len() {
            if let Some(&distance) = graph
                .get(&(nodes[i], nodes[j]))
                .or_else(|| graph.get(&(nodes[j], nodes[i])))
            {
                answer[i].push((j, distance));
                answer[j].push((i, distance));
            }
        }
    }

    (nodes, start_node_pos, end_node_pos, answer)
}

fn find_longest(
    node: usize,
    end_node: usize,
    walked_so_far: usize,
    graph: &[Vec<(usize, usize)>],
    seen: &mut BTreeSet<usize>,
) -> usize {
    if node == end_node {
        return walked_so_far;
    }
    if seen.contains(&node) {
        return 0;
    }

    seen.insert(node);
    let possible_destinations = graph[node]
        .iter()
        .filter(|&(destination, _)| !seen.contains(destination))
        .collect::<Vec<_>>();
    let max_distance = possible_destinations
        .into_iter()
        .map(|&(destination, distance)| {
            find_longest(destination, end_node, walked_so_far + distance, graph, seen)
        })
        .max()
        .unwrap_or(0);

    seen.remove(&node);
    max_distance
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let start = find_start(&grid);
    let (_, start_node_pos, end_node_pos, graph) = create_graph(&grid, start);
    let mut seen = BTreeSet::new();
    find_longest(start_node_pos, end_node_pos, 0, &graph, &mut seen)
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test));
    println!("Part 2: {}", part2(test));

    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
