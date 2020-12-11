use anyhow::{anyhow, Result};
use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use array2d::*;
use itertools::*;
use rayon::prelude::*;
use std::cell::RefCell;
use std::fmt;
use std::sync::Arc;
use std::{
    fmt::{Debug, Display},
    iter::FromIterator,
    sync::{Mutex, RwLock},
};
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Occupied,
    Empty,
    Floor,
}
impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Floor => write!(f, "."),
            Tile::Occupied => write!(f, "#"),
            Tile::Empty => write!(f, "L"),
        }
    }
}
impl Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Floor => write!(f, "."),
            Tile::Occupied => write!(f, "#"),
            Tile::Empty => write!(f, "L"),
        }
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Array2D<Tile> {
    let rows = Vec::from_iter(input.lines().map(|s| {
        s.trim()
            .chars()
            .map(|b| match b {
                'L' => Tile::Empty,
                '#' => Tile::Occupied,
                '.' => Tile::Floor,
                _ => unimplemented!(),
            })
            .collect()
    }));
    Array2D::from_rows(&rows)
}

#[inline]
fn get_neighbours(input: &Array2D<Tile>, row: usize, column: usize) -> usize {
    match (row, column) {
        (0, 0) => {
            let kernel = &[(row, column + 1), (row + 1, column + 1), (row + 1, column)];
            kernel
                .iter()
                .filter_map(|(row, col)| input.get(*row, *col))
                .filter(|&&x| x == Tile::Occupied)
                .count()
        }
        (0, _) => {
            let kernel = &[
                (row, column + 1),
                (row + 1, column + 1),
                (row + 1, column),
                (row + 1, column - 1),
                (row, column - 1),
            ];
            kernel
                .iter()
                .filter_map(|(row, col)| input.get(*row, *col))
                .filter(|&&x| x == Tile::Occupied)
                .count()
        }
        (_, 0) => {
            let kernel = &[
                (row - 1, column),
                (row - 1, column + 1),
                (row, column + 1),
                (row + 1, column + 1),
                (row + 1, column),
            ];
            kernel
                .iter()
                .filter_map(|(row, col)| input.get(*row, *col))
                .filter(|&&x| x == Tile::Occupied)
                .count()
        }
        (_, _) => {
            let kernel = &[
                (row - 1, column - 1),
                (row - 1, column),
                (row - 1, column + 1),
                (row, column + 1),
                (row + 1, column + 1),
                (row + 1, column),
                (row + 1, column - 1),
                (row, column - 1),
            ];
            kernel
                .iter()
                .filter_map(|(row, col)| input.get(*row, *col))
                .filter(|&&x| x == Tile::Occupied)
                .count()
        }
    }
}

#[inline]
fn check_cell(input: &Array2D<Tile>, row: i32, column: i32) -> Result<&Tile> {
    if row < 0
        || row as usize >= input.num_rows()
        || column < 0
        || column as usize >= input.num_columns()
    {
        return Err(anyhow::anyhow!("Out of bounds"));
    }
    input
        .get(row as usize, column as usize)
        .ok_or(anyhow::anyhow!("Out of bounds"))
}



#[inline]
fn get_neighbours_distant(input: &Array2D<Tile>, row: usize, column: usize) -> usize {
    let directions: &[(i32, i32); 8] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];

    directions
        .par_iter()
        .filter(|(x, y)| {
            let mut found = false;
            for z in 1..99i32 {
                match check_cell(input, row as i32 + x * z, column as i32 + y * z) {
                    Err(_) => break,
                    Ok(Tile::Occupied) => {
                        found = true;
                        break;
                    }
                    Ok(Tile::Empty) => break,
                    _ => {}
                }
            }
            found
        })
        .count()
}




#[aoc(day11, part1)]
pub fn part1(input: &Array2D<Tile>) -> usize {
    let rows = input.num_rows();
    let columns = input.num_columns();
    let current_gen = RefCell::new(input.clone());
    let mut next_gen = current_gen.clone();
    loop {
        // for row_iter in current_gen.borrow().rows_iter() {
        //     for element in row_iter {
        //         print!("{} ", element);
        //     }
        //     println!();
        // }
        // println!();

        (0..rows)
            .cartesian_product(0..columns)
            .for_each(|(row, column)| match current_gen.borrow()[(row, column)] {
                Tile::Empty => {
                    if get_neighbours(&current_gen.borrow(), row, column) == 0 {
                        next_gen.borrow_mut()[(row, column)] = Tile::Occupied
                    }
                }
                Tile::Occupied => {
                    if get_neighbours(&current_gen.borrow(), row, column) >= 4 {
                        next_gen.borrow_mut()[(row, column)] = Tile::Empty
                    }
                }
                Tile::Floor => {}
            });
        // for row_iter in next_gen.borrow().rows_iter() {
        //     for element in row_iter {
        //         print!("{} ", element);
        //     }
        //     println!();
        // }
        // println!("---");
        if *current_gen.borrow() == *next_gen.borrow() {
            break;
        }
        current_gen.replace(next_gen.into_inner());
        next_gen = current_gen.clone()
    }
    return next_gen
        .borrow()
        .elements_row_major_iter()
        .filter(|x| **x == Tile::Occupied)
        .count();
}

#[inline]
fn get_neighbours_par(
    input: &Vec<Tile>,
    rows: usize,
    columns: usize,
    row: usize,
    column: usize,
) -> usize {
    match (row, column) {
        (0, 0) => {
            let kernel = &[(row, column + 1), (row + 1, column + 1), (row + 1, column)];
            kernel
                .iter()
                .filter(|(row, col)| *row < rows && *col < columns)
                .filter_map(|(row, col)| input.get(*row * columns + *col))
                .filter(|&&x| x == Tile::Occupied)
                .count()
        }
        (0, _) => {
            let kernel = &[
                (row, column + 1),
                (row + 1, column + 1),
                (row + 1, column),
                (row + 1, column - 1),
                (row, column - 1),
            ];
            kernel
                .iter()
                .filter(|(row, col)| *row < rows && *col < columns)
                .filter_map(|(row, col)| input.get(*row * columns + *col))
                .filter(|&&x| x == Tile::Occupied)
                .count()
        }
        (_, 0) => {
            let kernel = &[
                (row - 1, column),
                (row - 1, column + 1),
                (row, column + 1),
                (row + 1, column + 1),
                (row + 1, column),
            ];
            kernel
                .iter()
                .filter(|(row, col)| *row < rows && *col < columns)
                .filter_map(|(row, col)| input.get(*row * columns + *col))
                .filter(|&&x| x == Tile::Occupied)
                .count()
        }
        (_, _) => {
            let kernel = &[
                (row - 1, column - 1),
                (row - 1, column),
                (row - 1, column + 1),
                (row, column + 1),
                (row + 1, column + 1),
                (row + 1, column),
                (row + 1, column - 1),
                (row, column - 1),
            ];
            kernel
                .iter()
                .filter(|(row, col)| *row < rows && *col < columns)
                .filter_map(|(row, col)| input.get(*row * columns + *col))
                .filter(|&&x| x == Tile::Occupied)
                .count()
        }
    }
}

#[aoc(day11, part1, parallel)]
pub fn part1_par(input: &Array2D<Tile>) -> usize {
    let rows = input.num_rows();
    let columns = input.num_columns();
    let mut current_gen = input.as_row_major();
    let indices = (0..rows)
        .cartesian_product(0..columns)
        .collect::<Vec<(usize, usize)>>();
    loop {
        // for (i, x) in current_gen.iter().enumerate() {
        //     print!("{} ", x);
        //     if i % columns == columns-1 {
        //         println!();
        //     }
        // }
        // println!();
        let next_gen = indices
            .par_iter()
            .map(
                |(row, column)| match current_gen[*row * columns + *column] {
                    Tile::Empty => {
                        if get_neighbours_par(&current_gen, rows, columns, *row, *column) == 0 {
                            Tile::Occupied
                        } else {
                            Tile::Empty
                        }
                    }
                    Tile::Occupied => {
                        if get_neighbours_par(&current_gen, rows, columns, *row, *column) >= 4 {
                            Tile::Empty
                        } else {
                            Tile::Occupied
                        }
                    }
                    Tile::Floor => Tile::Floor,
                },
            )
            .collect::<Vec<Tile>>();
        // for (i, x) in next_gen.iter().enumerate() {
        //     print!("{} ", x);
        //     if i % columns == columns-1 {
        //         println!();
        //     }
        // }
        // println!("---");
        if current_gen == next_gen {
            break;
        }
        current_gen = next_gen;
    }
    return current_gen
        .par_iter()
        .filter(|x| **x == Tile::Occupied)
        .count();
}



#[aoc(day11, part2)]
pub fn part2(input: &Array2D<Tile>) -> usize {
    let rows = input.num_rows();
    let columns = input.num_columns();
    let current_gen = RefCell::new(input.clone());
    let mut next_gen = current_gen.clone();
    loop {
        // for row_iter in current_gen.borrow().rows_iter() {
        //     for element in row_iter {
        //         print!("{} ", element);
        //     }
        //     println!();
        // }
        // println!();

        (0..rows)
            .cartesian_product(0..columns)
            .for_each(|(row, column)| match current_gen.borrow()[(row, column)] {
                Tile::Empty => {
                    if get_neighbours_distant(&current_gen.borrow(), row, column) == 0 {
                        next_gen.borrow_mut()[(row, column)] = Tile::Occupied
                    }
                }
                Tile::Occupied => {
                    if get_neighbours_distant(&current_gen.borrow(), row, column) >= 5 {
                        next_gen.borrow_mut()[(row, column)] = Tile::Empty
                    }
                }
                Tile::Floor => {}
            });
        // for row_iter in next_gen.borrow().rows_iter() {
        //     for element in row_iter {
        //         print!("{} ", element);
        //     }
        //     println!();
        // }
        // println!("---");
        if *current_gen.borrow() == *next_gen.borrow() {
            break;
        }
        current_gen.replace(next_gen.into_inner());
        next_gen = current_gen.clone()
    }
    return next_gen
        .borrow()
        .elements_row_major_iter()
        .filter(|x| **x == Tile::Occupied)
        .count();
}

#[inline]
fn check_cell_par(
    input: &Vec<Tile>,
    rows: usize,
    columns: usize,
    row: i32,
    column: i32,
) -> Result<&Tile> {
    if row < 0 || row as usize >= rows || column < 0 || column as usize >= columns {
        return Err(anyhow::anyhow!("Out of bounds"));
    }
    input
        .get(row as usize * columns + column as usize)
        .ok_or(anyhow::anyhow!("Out of bounds"))
}

#[inline]
fn get_neighbours_par_distant(
    input: &Vec<Tile>,
    rows: usize,
    columns: usize,
    row: usize,
    column: usize,
) -> usize {
    let directions: &[(i32, i32); 8] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];

    directions
        .iter()
        .filter(|(x, y)| {
            let mut found = false;
            for z in 1..99i32 {
                match check_cell_par(
                    input,
                    rows,
                    columns,
                    row as i32 + x * z,
                    column as i32 + y * z,
                ) {
                    Err(_) => break,
                    Ok(Tile::Occupied) => {
                        found = true;
                        break;
                    }
                    Ok(Tile::Empty) => break,
                    _ => {}
                }
            }
            found
        })
        .count()
}

#[aoc(day11, part2, parallel)]
pub fn part2_par(input: &Array2D<Tile>) -> usize {
    let rows = input.num_rows();
    let columns = input.num_columns();
    let mut current_gen = input.as_row_major();
    let indices = (0..rows)
        .cartesian_product(0..columns)
        .collect::<Vec<(usize, usize)>>();
    loop {
        // for (i, x) in current_gen.iter().enumerate() {
        //     print!("{} ", x);
        //     if i % columns == columns-1 {
        //         println!();
        //     }
        // }
        // println!();
        let next_gen = indices
            .par_iter()
            .map(
                |(row, column)| match current_gen[*row * columns + *column] {
                    Tile::Empty => {
                        if get_neighbours_par_distant(&current_gen, rows, columns, *row, *column)
                            == 0
                        {
                            Tile::Occupied
                        } else {
                            Tile::Empty
                        }
                    }
                    Tile::Occupied => {
                        if get_neighbours_par_distant(&current_gen, rows, columns, *row, *column)
                            >= 5
                        {
                            Tile::Empty
                        } else {
                            Tile::Occupied
                        }
                    }
                    Tile::Floor => Tile::Floor,
                },
            )
            .collect::<Vec<Tile>>();
        // for (i, x) in next_gen.iter().enumerate() {
        //     print!("{} ", x);
        //     if i % columns == columns-1 {
        //         println!();
        //     }
        // }
        // println!("---");
        if current_gen == next_gen {
            break;
        }
        current_gen = next_gen;
    }
    return current_gen
        .par_iter()
        .filter(|x| **x == Tile::Occupied)
        .count();
}

#[inline]
fn check_cell_par2(
    input: &Vec<Arc<RwLock<Vec<Tile>>>>,
    rows: usize,
    columns: usize,
    row: i32,
    column: i32,
) -> Result<Tile> {
    if row < 0 || row as usize >= rows || column < 0 || column as usize >= columns {
        return Err(anyhow::anyhow!("Out of bounds"));
    }
    let row = input
        .get(row as usize)
        .ok_or(anyhow::anyhow!("Out of bounds"))?
        .read()
        .unwrap();
    row.get(column as usize)
        .cloned()
        .ok_or(anyhow::anyhow!("Out of bounds"))
}

#[inline]
fn get_neighbours_par2(
    input: &Vec<Arc<RwLock<Vec<Tile>>>>,
    rows: usize,
    columns: usize,
    row: usize,
    column: usize,
    distant: bool,
) -> usize {
    let directions: &[(i32, i32); 8] = &[
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
    ];

    directions
        .iter()
        .filter(|(x, y)| {
            let mut found = false;
            for z in 1..99i32 {
                match check_cell_par2(
                    input,
                    rows,
                    columns,
                    row as i32 + x * z,
                    column as i32 + y * z,
                ) {
                    Err(_) => break,
                    Ok(Tile::Occupied) => {
                        found = true;
                        break;
                    }
                    Ok(Tile::Empty) => break,
                    _ => {
                        if !distant {
                            break;
                        }
                    }
                }
            }
            found
        })
        .count()
}

#[aoc(day11, part2, parallel2)]
pub fn part2_par2(input: &Array2D<Tile>) -> usize {
    let rows = input.num_rows();
    let columns = input.num_columns();
    let test = input.as_rows();
    let first = test
        .iter()
        .map(|x| Arc::new(RwLock::new(x.clone())))
        .collect::<Vec<Arc<RwLock<Vec<Tile>>>>>();
    let second = test
        .iter()
        .map(|x| Arc::new(RwLock::new(x.clone())))
        .collect::<Vec<Arc<RwLock<Vec<Tile>>>>>();
    let current_gen = Arc::new(RwLock::new(&first));
    let next_gen = Arc::new(RwLock::new(&second));
    let mut first_current = true;
    loop {
        // for x in current_gen.read().unwrap().iter(){
        //     for c in x.read().unwrap().iter() {
        //         print!("{} ", c);
        //     }
        //     println!();
        // }
        // println!();
        (*current_gen)
            .read()
            .unwrap()
            .par_iter()
            .enumerate()
            .for_each(|(row_index, column)| {
                let row = &mut next_gen.read().unwrap()[row_index].write().unwrap();
                column
                    .read()
                    .unwrap()
                    .iter()
                    .enumerate()
                    .for_each(|(column_index, cell)| {
                        row[column_index] = match cell {
                            Tile::Empty => {
                                if get_neighbours_par2(
                                    &current_gen.read().unwrap(),
                                    rows,
                                    columns,
                                    row_index,
                                    column_index,
                                    true,
                                ) == 0
                                {
                                    Tile::Occupied
                                } else {
                                    Tile::Empty
                                }
                            }
                            Tile::Occupied => {
                                if get_neighbours_par2(
                                    &current_gen.read().unwrap(),
                                    rows,
                                    columns,
                                    row_index,
                                    column_index,
                                    true,
                                ) >= 5
                                {
                                    Tile::Empty
                                } else {
                                    Tile::Occupied
                                }
                            }
                            Tile::Floor => Tile::Floor,
                        }
                    })
            });
        // for x in next_gen.read().unwrap().iter(){
        //     for c in x.read().unwrap().iter() {
        //         print!("{} ", c);
        //     }
        //     println!();
        // }
        // println!("---");
        if current_gen
            .read()
            .unwrap()
            .iter()
            .map(|x| x.read().unwrap())
            .zip(next_gen.read().unwrap().iter().map(|x| x.read().unwrap()))
            .map(|(x, y)| *x == *y)
            .all(|x| x == true)
        {
            break;
        }
        if first_current {
            *current_gen.write().unwrap() = &second;
            *next_gen.write().unwrap() = &first;
        } else {
            *current_gen.write().unwrap() = &first;
            *next_gen.write().unwrap() = &second;
        }
        first_current = !first_current;
    }
    return current_gen
        .read()
        .unwrap()
        .par_iter()
        .map(|x| {
            x.read()
                .unwrap()
                .iter()
                .filter(|x| **x == Tile::Occupied)
                .count()
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator(&SAMPLE)), 37);
    }
    #[test]
    fn sample1_par() {
        assert_eq!(part1_par(&input_generator(&SAMPLE)), 37);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&input_generator(&SAMPLE)), 26);
    }

    #[test]
    fn sample2_par2() {
        assert_eq!(part2_par2(&input_generator(&SAMPLE)), 26);
    }
}
