use std::{collections::HashMap, fmt::Display};

use anyhow::Result;
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use array2d::Array2D;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, one_of, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};
use shrinkwraprs::Shrinkwrap;
use std::hash::Hash;

const NESSI: &str = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

pub struct Input {
    tiles: HashMap<TileId, Vec<(usize, usize)>>,
}
fn parse_tile_header(input: &str) -> IResult<&str, TileId> {
    let (input, id) = terminated(
        preceded(
            tag("Tile"),
            preceded(space1, map_res(digit1, |s: &str| s.parse::<usize>())),
        ),
        pair(tag(":"), line_ending),
    )(input)?;
    Ok((input, TileId(id)))
}
fn parse_tile(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    let (input, rows) = separated_list1(line_ending, many1(one_of("#.")))(input)?;
    assert_eq!(rows.len(), 10);
    let content = rows.iter().enumerate().fold(vec![], |acc, (r, row)| {
        assert_eq!(row.len(), 10);
        row.iter().enumerate().fold(acc, |mut acc, (c, field)| {
            if field == &'#' {
                acc.push((c, r));
            }
            acc
        })
    });

    Ok((input, content))
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Input {
    println!("parse Input");
    let (_, tiles): (_, Vec<(TileId, Vec<(usize, usize)>)>) = separated_list1(
        tuple((line_ending, line_ending)),
        tuple((parse_tile_header, parse_tile)),
    )(input)
    .expect("parser error");
    println!("parsed Input");
    Input {
        tiles: tiles
            .iter()
            .cloned()
            .map(|(key, value)| (key, value))
            .collect::<HashMap<_, _>>(),
    }
}
#[derive(PartialEq, Copy, Clone, Eq, Debug)]
enum Neighbor {
    Yes,
    No,
    Flipped,
}
#[derive(Debug, PartialEq, Eq, Clone)]
struct Edges {
    top: u64,
    right: u64,
    left: u64,
    bottom: u64,
}
impl Display for Edges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "T: {:10b}, R: {:10b}, B: {:10b}, L: {:10b}",
            self.top, self.right, self.bottom, self.left
        )
    }
}
impl Edges {
    fn share_n(&self, rhs: &Edges) -> Neighbor {
        if self.top == rhs.bottom {
            return Neighbor::Yes;
        } else if self.top == flipped(rhs.bottom, 10) {
            return Neighbor::Flipped;
        }
        Neighbor::No
    }
    fn share_w(&self, rhs: &Edges) -> Neighbor {
        if self.left == rhs.right {
            return Neighbor::Yes;
        } else if self.right == rhs.right {
            return Neighbor::Flipped;
        }
        Neighbor::No
    }
}

#[derive(Shrinkwrap, Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub struct TileId(usize);
impl TileId {
    fn flip(&self, map: &mut HashMap<TileId, (Edges, Vec<(usize, usize)>)>) {
        if let Some((edges, vec)) = map.get_mut(self) {
            let copy = edges.clone();
            edges.left = copy.right;
            edges.right = copy.left;
            edges.top = flipped(edges.top, 10);
            edges.bottom = flipped(edges.bottom, 10);
            for s in vec.iter_mut() {
                *s = (9 - s.0, s.1);
            }
        }
    }

    fn rotate(&self, map: &mut HashMap<TileId, (Edges, Vec<(usize, usize)>)>) {
        if let Some((edges, vec)) = map.get_mut(self) {
            let copy = edges.clone();
            edges.right = copy.top;
            edges.top = flipped(copy.left, 10);
            edges.left = copy.bottom;
            edges.bottom = flipped(copy.right, 10);

            for s in vec.iter_mut() {
                *s = (9 - s.1, s.0)
            }
        }
    }
    fn get_neighbours(
        &self,
        dir: Direction,
        tiles: &HashMap<TileId, (Edges, Vec<(usize, usize)>)>,
    ) -> usize {
        let left = &tiles[self];
        tiles
            .iter()
            .filter(|(k, _)| *k != self)
            .filter(|(_, v)| left.0.is_neighbor_with_dir(&v.0, dir, 10))
            .count()
    }
    fn match_tile(
        &self,
        rhs: &TileId,
        dir: Direction,
        tiles: &mut HashMap<TileId, (Edges, Vec<(usize, usize)>)>,
    ) -> Self {
        match dir {
            Direction::Left => {
                for _ in 0..4 {
                    if tiles[self].0.share_w(&tiles[rhs].0) == Neighbor::Yes {
                        return *self;
                    } else if tiles[self].0.share_w(&tiles[rhs].0) == Neighbor::Flipped {
                        self.flip(tiles);
                        if tiles[self].0.share_w(&tiles[rhs].0) == Neighbor::Yes {
                            return *self;
                        }
                    }
                    self.rotate(tiles);
                }
                panic!()
            }
            Direction::Top => {
                for _ in 0..4 {
                    if tiles[self].0.share_n(&tiles[rhs].0) == Neighbor::Yes {
                        return *self;
                    } else if tiles[self].0.share_n(&tiles[rhs].0) == Neighbor::Flipped {
                        self.flip(tiles);
                        if tiles[self].0.share_n(&tiles[rhs].0) == Neighbor::Yes {
                            return *self;
                        }
                    }
                    self.rotate(tiles);
                }
                panic!()
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

impl Edges {
    fn is_neighbor(&self, rhs: &Edges, size: usize) -> bool {
        self.right == rhs.left
            || self.right == rhs.right
            || self.right == rhs.top
            || self.right == rhs.bottom
            || self.bottom == rhs.left
            || self.bottom == rhs.right
            || self.bottom == rhs.top
            || self.bottom == rhs.bottom
            || self.left == rhs.left
            || self.left == rhs.right
            || self.left == rhs.top
            || self.left == rhs.bottom
            || self.top == rhs.left
            || self.top == rhs.right
            || self.top == rhs.top
            || self.top == rhs.bottom
            || self.right == flipped(rhs.left, size)
            || self.right == flipped(rhs.right, size)
            || self.right == flipped(rhs.top, size)
            || self.right == flipped(rhs.bottom, size)
            || self.bottom == flipped(rhs.left, size)
            || self.bottom == flipped(rhs.right, size)
            || self.bottom == flipped(rhs.top, size)
            || self.bottom == flipped(rhs.bottom, size)
            || self.left == flipped(rhs.left, size)
            || self.left == flipped(rhs.right, size)
            || self.left == flipped(rhs.top, size)
            || self.left == flipped(rhs.bottom, size)
            || self.top == flipped(rhs.left, size)
            || self.top == flipped(rhs.right, size)
            || self.top == flipped(rhs.top, size)
            || self.top == flipped(rhs.bottom, size)
    }
    fn is_neighbor_with_dir(&self, rhs: &Edges, dir: Direction, size: usize) -> bool {
        match dir {
            Direction::Right => {
                self.right == rhs.left
                    || self.right == rhs.right
                    || self.right == rhs.top
                    || self.right == rhs.bottom
                    || self.right == flipped(rhs.left, size)
                    || self.right == flipped(rhs.right, size)
                    || self.right == flipped(rhs.top, size)
                    || self.right == flipped(rhs.bottom, size)
            }
            Direction::Bottom => {
                self.bottom == rhs.left
                    || self.bottom == rhs.right
                    || self.bottom == rhs.top
                    || self.bottom == rhs.bottom
                    || self.bottom == flipped(rhs.left, size)
                    || self.bottom == flipped(rhs.right, size)
                    || self.bottom == flipped(rhs.top, size)
                    || self.bottom == flipped(rhs.bottom, size)
            }
            Direction::Left => {
                self.left == rhs.left
                    || self.left == rhs.right
                    || self.left == rhs.top
                    || self.left == rhs.bottom
                    || self.left == flipped(rhs.left, size)
                    || self.left == flipped(rhs.right, size)
                    || self.left == flipped(rhs.top, size)
                    || self.left == flipped(rhs.bottom, size)
            }
            Direction::Top => {
                self.top == rhs.left
                    || self.top == rhs.right
                    || self.top == rhs.top
                    || self.top == rhs.bottom
                    || self.top == flipped(rhs.left, size)
                    || self.top == flipped(rhs.right, size)
                    || self.top == flipped(rhs.top, size)
                    || self.top == flipped(rhs.bottom, size)
            }
        }
    }
}

#[inline]
fn flipped(i: u64, size: usize) -> u64 {
    let i = i.reverse_bits();
    i.rotate_left(size as u32)
}

#[aoc(day20, part1)]
pub fn part1(input: &Input) -> usize {
    let size = 10;
    let tiles = input
        .tiles
        .iter()
        .map(|(s, vec)| {
            let mut top = vec
                .iter()
                .filter(|s| s.1 == 0)
                .map(|s| s.0)
                .collect::<Vec<_>>();
            top.sort();
            let top = (0..size)
                .into_iter()
                .fold(0u64, |acc, v| (acc << 1) | top.contains(&v) as u64);
            let mut right = vec
                .iter()
                .filter(|s| s.0 == size - 1)
                .map(|s| s.1)
                .collect::<Vec<_>>();
            right.sort();
            let right = (0..size)
                .into_iter()
                .fold(0u64, |acc, v| (acc << 1) | right.contains(&v) as u64);
            let mut bottom = vec
                .iter()
                .filter(|s| s.1 == size - 1)
                .map(|s| s.0)
                .collect::<Vec<_>>();
            bottom.sort();
            let bottom = (0..size)
                .into_iter()
                .fold(0u64, |acc, v| (acc << 1) | bottom.contains(&v) as u64);
            let mut left = vec
                .iter()
                .filter(|s| s.0 == 0)
                .map(|s| s.1)
                .collect::<Vec<_>>();
            left.sort();
            let left = (0..size)
                .into_iter()
                .fold(0u64, |acc, v| (acc << 1) | left.contains(&v) as u64);

            (
                s,
                Edges {
                    top,
                    right,
                    left,
                    bottom,
                },
            )
        })
        .collect::<HashMap<_, _>>();
    let tiles_ids = input.tiles.keys().collect::<Vec<_>>();
    tiles_ids
        .iter()
        .filter(|cur| {
            let left = &tiles[*cur];
            tiles_ids
                .iter()
                .filter(|s| *s != *cur)
                .filter(|s| left.is_neighbor(&tiles[*s], size))
                .count()
                == 2
        })
        .map(|s| ***s)
        .product()
}

#[aoc(day20, part2)]
pub fn part2(input: &Input) -> usize {
    let size = 10;
    println!("Contruct tile map");
    let mut tiles = input
        .tiles
        .iter()
        .map(|(s, vec)| {
            let mut top = vec
                .iter()
                .filter(|s| s.1 == 0)
                .map(|s| s.0)
                .collect::<Vec<_>>();
            top.sort();
            let top = (0..size)
                .into_iter()
                .fold(0u64, |acc, v| (acc << 1) | top.contains(&v) as u64);
            let mut right = vec
                .iter()
                .filter(|s| s.0 == size - 1)
                .map(|s| s.1)
                .collect::<Vec<_>>();
            right.sort();
            let right = (0..size)
                .into_iter()
                .fold(0u64, |acc, v| (acc << 1) | right.contains(&v) as u64);
            let mut bottom = vec
                .iter()
                .filter(|s| s.1 == size - 1)
                .map(|s| s.0)
                .collect::<Vec<_>>();
            bottom.sort();
            let bottom = (0..size)
                .into_iter()
                .fold(0u64, |acc, v| (acc << 1) | bottom.contains(&v) as u64);
            let mut left = vec
                .iter()
                .filter(|s| s.0 == 0)
                .map(|s| s.1)
                .collect::<Vec<_>>();
            left.sort();
            let left = (0..size)
                .into_iter()
                .fold(0u64, |acc, v| (acc << 1) | left.contains(&v) as u64);
            (
                *s,
                (
                    Edges {
                        top,
                        right,
                        left,
                        bottom,
                    },
                    vec.clone(),
                ),
            )
        })
        .collect::<HashMap<_, _>>();
    let tiles_ids = input.tiles.keys().cloned().collect::<Vec<_>>();
    let corner = tiles_ids
        .iter()
        .find(|cur| {
            let left = &tiles[*cur];
            tiles_ids
                .iter()
                .filter(|s| *s != *cur)
                .filter(|s| left.0.is_neighbor(&tiles[*s].0, size))
                .count()
                == 2
        })
        .unwrap();
    let grid_size = (input.tiles.len() as f64).sqrt() as usize;

    let mut grid: Array2D<Option<TileId>> = Array2D::filled_with(None, grid_size, grid_size);

    loop {
        let right = corner.get_neighbours(Direction::Right, &tiles);
        let bottom = corner.get_neighbours(Direction::Bottom, &tiles);
        match (right, bottom) {
            (1, 1) => break,
            (0, 1) => corner.flip(&mut tiles),
            (_, _) => corner.rotate(&mut tiles),
        }
    }
    println!();
    println!("First row starting with: {}", **corner);
    grid[(0, 0)] = Some(*corner);
    let y = 0;
    for x in 1..grid_size {
        let left_candidates = {
            if let Some(cell) = grid.get(y, x - 1) {
                if let Some(cell) = cell {
                    let left = &tiles[cell];
                    tiles_ids
                        .iter()
                        .filter(|s| **s != *cell)
                        .filter(|s| {
                            left.0
                                .is_neighbor_with_dir(&tiles[*s].0, Direction::Right, size)
                        })
                        .map(|s| *s)
                        .collect::<Vec<_>>()
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        };

        grid[(y, x)] = Some(left_candidates[0].match_tile(
            &grid.get(y, x - 1).unwrap().unwrap(),
            Direction::Left,
            &mut tiles,
        ));
    }
    for y in 1..grid_size {
        for x in 0..grid_size {
            let candidates = {
                if let Some(cell) = grid.get(y - 1, x) {
                    if let Some(cell) = cell {
                        let left = &tiles[cell];
                        tiles_ids
                            .iter()
                            .filter(|s| **s != *cell)
                            .filter(|s| {
                                left.0
                                    .is_neighbor_with_dir(&tiles[*s].0, Direction::Bottom, size)
                            })
                            .map(|s| *s)
                            .collect::<Vec<_>>()
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            };

            grid[(y, x)] = Some(candidates[0].match_tile(
                &grid.get(y - 1, x).unwrap().unwrap(),
                Direction::Top,
                &mut tiles,
            ))
        }
    }

    println!("Remove Edges");
    let mut complete_map: Vec<(usize, usize)> = Vec::new();

    for (i, s) in grid.elements_row_major_iter().enumerate() {
        let first_index = (i % grid_size * 8, i / grid_size * 8);
        tiles[&s.unwrap()].1.iter().for_each(|(x, y)| {
            if (1..9).contains(x) && (1..9).contains(y) {
                complete_map.push((x - 1 + first_index.0, y - 1 + first_index.1));
            }
        });
    }

    let nessi_offsets = NESSI.lines().enumerate().fold(vec![], |acc, (y, v)| {
        v.chars().enumerate().fold(acc, |mut acc, (x, c)| {
            if c == '#' {
                acc.push((x, y))
            }
            acc
        })
    });
    let mut found: Vec<(usize, usize)> = vec![];
    // let hashedBigMap: HashSet<(usize, usize)> = bigMap.iter().cloned().collect();
    for _ in 0..4 {
        if found.len() == 0 {
            for y in 0..grid_size * 8 {
                for x in 0..grid_size * 8 {
                    if nessi_offsets
                        .iter()
                        .all(|(x2, y2)| complete_map.contains(&(x + x2, y + y2)))
                    {
                        found.push((x, y));
                    }
                }
            }

            if found.len() != 0 {
                break;
            }
            for (x, y) in complete_map.iter_mut() {
                let c = *x;
                *x = (grid_size * 8) - 1 - *y;
                *y = c;
            }
        }
    }
    if found.len() == 0 {
        for (x, _) in complete_map.iter_mut() {
            *x = (grid_size * 8) - 1 - *x;
        }
        // let hashedBigMap: HashSet<(usize, usize)> = bigMap.iter().cloned().collect();
        for _ in 0..4 {
            if found.len() == 0 {
                for y in 0..grid_size * 8 {
                    for x in 0..grid_size * 8 {
                        if nessi_offsets
                            .iter()
                            .all(|(x2, y2)| complete_map.contains(&(x + x2, y + y2)))
                        {
                            found.push((x, y));
                        }
                    }
                }
                if found.len() != 0 {
                    break;
                }
                for (x, y) in complete_map.iter_mut() {
                    let c = *x;
                    *x = (grid_size * 8) - 1 - *y;
                    *y = c;
                }
            }
        }
    }

    for (nx, ny) in found {
        for (x, y) in &nessi_offsets {
            let pos = complete_map
                .iter()
                .position(|s| *s == (x + nx, y + ny))
                .unwrap();
            complete_map.remove(pos);
        }
    }
    complete_map.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn sample1() {
        let input = input_generator(SAMPLE1);
        assert_eq!(input.tiles.iter().len(), 9);
        assert_eq!(part1(&input), 20899048083289);
    }

    #[test]
    fn sample2() {
        let input = input_generator(SAMPLE1);
        assert_eq!(input.tiles.iter().len(), 9);
        assert_eq!(part2(&input), 273);
    }

    #[test]
    fn test_rotation() {
        let tile = TileId(0);
        let size = 10;
        let mut tiles: HashMap<TileId, Vec<(usize, usize)>> = HashMap::new();
        tiles.insert(
            tile,
            vec![(0, 0), (8, 0), (9, 2), (9, 7), (0, 3), (5, 9), (2, 2)],
        );
        let mut tiles = tiles
            .iter()
            .map(|(s, vec)| {
                println!("{}", **s);
                let mut top = vec
                    .iter()
                    .filter(|s| s.1 == 0)
                    .map(|s| s.0)
                    .collect::<Vec<_>>();
                top.sort();
                let top = (0..size)
                    .into_iter()
                    .fold(0u64, |acc, v| (acc << 1) | top.contains(&v) as u64);
                let mut right = vec
                    .iter()
                    .filter(|s| s.0 == size - 1)
                    .map(|s| s.1)
                    .collect::<Vec<_>>();
                right.sort();
                let right = (0..size)
                    .into_iter()
                    .fold(0u64, |acc, v| (acc << 1) | right.contains(&v) as u64);
                let mut bottom = vec
                    .iter()
                    .filter(|s| s.1 == size - 1)
                    .map(|s| s.0)
                    .collect::<Vec<_>>();
                bottom.sort();
                let bottom = (0..size)
                    .into_iter()
                    .fold(0u64, |acc, v| (acc << 1) | bottom.contains(&v) as u64);
                let mut left = vec
                    .iter()
                    .filter(|s| s.0 == 0)
                    .map(|s| s.1)
                    .collect::<Vec<_>>();
                left.sort();
                let left = (0..size)
                    .into_iter()
                    .fold(0u64, |acc, v| (acc << 1) | left.contains(&v) as u64);
                (
                    *s,
                    (
                        Edges {
                            top,
                            right,
                            left,
                            bottom,
                        },
                        vec.clone(),
                    ),
                )
            })
            .collect::<HashMap<_, _>>();
        let copy = tiles[&tile].clone();
        tile.rotate(&mut tiles);
        assert_eq!(
            tiles[&tile].0,
            Edges {
                top: 0b1001,
                right: 0b1000000010,
                bottom: 0b10000100,
                left: 0b10000
            }
        );
        assert!(tiles[&tile].1.iter().all(|s| vec![
            (9, 0),
            (6, 0),
            (9, 8),
            (7, 9),
            (2, 9),
            (0, 5),
            (7, 2)
        ]
        .contains(s)));

        tile.rotate(&mut tiles);
        tile.rotate(&mut tiles);
        tile.rotate(&mut tiles);
        assert_eq!(copy.0, tiles[&tile].0);
    }
    #[test]
    fn test_flip() {
        let tile = TileId(0);
        let size = 10;
        let mut tiles: HashMap<TileId, Vec<(usize, usize)>> = HashMap::new();
        tiles.insert(
            tile,
            vec![(0, 0), (8, 0), (9, 2), (9, 7), (0, 3), (5, 9), (2, 2)],
        );
        let mut tiles = tiles
            .iter()
            .map(|(s, vec)| {
                println!("{}", **s);
                let mut top = vec
                    .iter()
                    .filter(|s| s.1 == 0)
                    .map(|s| s.0)
                    .collect::<Vec<_>>();
                top.sort();
                let top = (0..size)
                    .into_iter()
                    .fold(0u64, |acc, v| (acc << 1) | top.contains(&v) as u64);
                let mut right = vec
                    .iter()
                    .filter(|s| s.0 == size - 1)
                    .map(|s| s.1)
                    .collect::<Vec<_>>();
                right.sort();
                let right = (0..size)
                    .into_iter()
                    .fold(0u64, |acc, v| (acc << 1) | right.contains(&v) as u64);
                let mut bottom = vec
                    .iter()
                    .filter(|s| s.1 == size - 1)
                    .map(|s| s.0)
                    .collect::<Vec<_>>();
                bottom.sort();
                let bottom = (0..size)
                    .into_iter()
                    .fold(0u64, |acc, v| (acc << 1) | bottom.contains(&v) as u64);
                let mut left = vec
                    .iter()
                    .filter(|s| s.0 == 0)
                    .map(|s| s.1)
                    .collect::<Vec<_>>();
                left.sort();
                let left = (0..size)
                    .into_iter()
                    .fold(0u64, |acc, v| (acc << 1) | left.contains(&v) as u64);
                (
                    *s,
                    (
                        Edges {
                            top,
                            right,
                            left,
                            bottom,
                        },
                        vec.clone(),
                    ),
                )
            })
            .collect::<HashMap<_, _>>();
        let copy = tiles[&tile].clone();
        tile.flip(&mut tiles);
        assert_eq!(
            tiles[&tile].0,
            Edges {
                top: 0b0100000001,
                right: 0b1001000000,
                bottom: 0b0000100000,
                left: 0b0010000100
            }
        );
        assert!(tiles[&tile].1.iter().all(|s| vec![
            (9, 0),
            (1, 0),
            (0, 2),
            (0, 7),
            (9, 3),
            (4, 9),
            (7, 2)
        ]
        .contains(s)));
        tile.flip(&mut tiles);
        assert_eq!(copy.0, tiles[&tile].0);
    }
}
