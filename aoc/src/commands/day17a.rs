use std::{path::PathBuf, fmt::{Display, Formatter}, cmp::Ordering};

use clap::Parser;

use crate::utils::{AsciiReader, slurp_bytes};

use super::{CommandImpl, DynError};

#[derive(Debug, Default)]
pub struct Costs {
    /// Estimated cost to end position
    pub h: u64,
    /// Cost to get to this position
    pub g: u64,
}

impl Costs {
    fn f(&self) -> u64 {
        self.h + self.g
    }
}

#[derive(Debug)]
pub struct Tile {
    weight: u8,
    traversed: bool,
    costs: Option<Costs>,
    from: Option<(usize, usize)>,
}

impl Tile {
    fn new(weight: u8) -> Self {
        Self {
            weight,
            traversed: false,
            costs: None,
            from: None,
        }
    }
}

pub struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile.weight)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn new(reader: &mut AsciiReader) -> Self {
        let mut tiles = Vec::new();
        while let Some(line) = reader.read_line() {
            let mut row = line.iter().map(|c| Tile::new(c - b'0')).collect();
            tiles.push(row);
        }

        Self { tiles }
    }

    fn distance(&self, from: (usize, usize), to: (usize, usize)) -> u64 {
        let (x1, y1) = from;
        let (x2, y2) = to;
        ((x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs()) as u64
    }

    fn calc_position_cost(&mut self, position: (usize, usize), from: (usize, usize), start: (usize, usize), end: (usize, usize)) {
        if position == start {
            return;
        }
        println!("calc_position_cost: {:?} -> {:?}", from, position);

        let g_cost = self.tiles[from.1][from.0].costs.as_ref().unwrap().g + self.tiles[position.1][position.0].weight as u64;
        let h_cost = self.distance(position, end);
        let total = g_cost + h_cost;

        let tile = &mut self.tiles[position.1][position.0];
        if tile.costs.is_none() || tile.costs.as_ref().unwrap().f() > total {
            tile.costs = Some(Costs { h: h_cost, g: g_cost });
            tile.from = Some(from);
            println!("costs: {:?}", tile.costs);
        }   
    }

    pub fn min_cost(&mut self, start: (usize, usize), end: (usize, usize)) -> u64 {
        let mut cost = 0;
        println!("min_cost: {:?} -> {:?}", start, end);

        self.tiles[start.1][start.0].costs = Some(Default::default());
        self.tiles[start.1][start.0].traversed = true;

        let mut visited: Vec<(usize, usize)> = vec![];

        loop {
            println!("visited: {:?}", visited);
            let (x, y) = {
                if visited.is_empty() {
                    println!("visited is empty");
                    start
                } else {
                    let mut min_cost = u64::MAX;
                    let mut min_positions: Vec<(usize, usize)> = vec![];
                    for position in &visited {
                        let tile = &self.tiles[position.1][position.0];
                        if tile.traversed {
                            continue;
                        }
                        let tile_cost = tile.costs.as_ref().unwrap().f();
                        match tile_cost.cmp(&min_cost) {
                            Ordering::Less => {
                                min_cost = tile_cost;
                                min_positions.clear();
                                min_positions.push(*position);
                            },
                            Ordering::Equal => {
                                min_positions.push(*position);
                            },
                            Ordering::Greater => {},
                        }
                    }

                    println!("min_positions: {:?}", min_positions);

                    if min_positions.len() > 1 {
                        let mut min_h_cost = u64::MAX;
                        let mut min_h_positions: Vec<(usize, usize)> = vec![];
                        for position in &min_positions {
                            let tile = &self.tiles[position.1][position.0];
                            let tile_cost = tile.costs.as_ref().unwrap().h;
                            match tile_cost.cmp(&min_h_cost) {
                                Ordering::Less => {
                                    min_h_cost = tile_cost;
                                    min_h_positions.clear();
                                    min_h_positions.push(*position);
                                },
                                Ordering::Equal => {
                                    min_h_positions.push(*position);
                                },
                                Ordering::Greater => {},
                            }
                        }
                        min_h_positions[0]
                    } else {
                        min_positions[0]
                    }
                }
            };
            let tile = &mut self.tiles[y][x];
            println!("position: {:?} tile: {:?}", (x, y), tile);
            tile.traversed = true;

            if x == end.0 && y == end.1 {
                println!("Found end");
                break;
            }

            for position in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].iter() {
                if position.1 >= self.tiles.len() || position.0 >= self.tiles[0].len() || position == &start {
                    continue;
                }
                self.calc_position_cost(*position, (x, y), start, end);
                if !visited.contains(position) {
                    visited.push(*position);
                }
            }
        }

        let mut current = end;
        loop {
            let tile = &self.tiles[current.1][current.0];
            // println!("tile: {:?}", tile);
            if let Some(from) = tile.from {
                // println!("from: {:?}", from);
                cost += self.tiles[from.1][from.0].weight as u64;
                if from == start {
                    break;
                }
                current = from;
                self.tiles[from.1][from.0].weight = 0;
            } else {
                break;
            }
        }

        println!("map: \n{}", self);

        cost
    }
}

struct Solver {
    reader: AsciiReader,
}

impl Solver {
    fn new(input: Vec<u8>) -> Self {
        Self {
            reader: AsciiReader::new(input),
        }
    }
    
    fn solve(&mut self) -> u64 {
        let mut map = Map::new(&mut self.reader);

        println!("width: {}", map.tiles[0].len());
        println!("height: {}", map.tiles.len());
        println!("map:\n{}", map);
        
        map.min_cost((0,0), (map.tiles.len() - 1, map.tiles[0].len() - 1))
    }
}

#[derive(Parser, Debug)]
pub struct Day17a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day17a {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day 17 A: {answer}");
        Ok(())
    }
}
