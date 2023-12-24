use std::{path::PathBuf, fmt::{Display, Formatter}};

use clap::Parser;

use crate::utils::{AsciiReader, slurp_bytes};

use super::{CommandImpl, DynError};

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

enum TileKind {
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
    Forward,
    Backward,
}

impl Display for TileKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TileKind::Empty => write!(f, "."),
            TileKind::VerticalSplitter => write!(f, "|"),
            TileKind::HorizontalSplitter => write!(f, "-"),
            TileKind::Forward => write!(f, "/"),
            TileKind::Backward => write!(f, "\\"),
        }
    }
}

pub struct Tile {
    kind: TileKind,
    visited_north: bool,
    visited_south: bool,
    visited_east: bool,
    visited_west: bool,
}

impl Tile {
    fn new(kind: TileKind) -> Self {
        Self {
            kind,
            visited_north: false,
            visited_south: false,
            visited_east: false,
            visited_west: false,
        }
    }

    fn reset(&mut self) {
        self.visited_north = false;
        self.visited_south = false;
        self.visited_east = false;
        self.visited_west = false;
    }

    fn visit(&mut self, direction: &Direction) {
        match direction {
            Direction::North => self.visited_north = true,
            Direction::South => self.visited_south = true,
            Direction::East => self.visited_east = true,
            Direction::West => self.visited_west = true,
        }
    }

    fn visited_from(&self, direction: &Direction) -> bool {
        match direction {
            Direction::North => self.visited_north,
            Direction::South => self.visited_south,
            Direction::East => self.visited_east,
            Direction::West => self.visited_west,
        }
    }

    fn visited(&self) -> bool {
        self.visited_north || self.visited_south || self.visited_east || self.visited_west
    }
}

pub struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile.kind)?
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn new(reader: &mut AsciiReader) -> Self {
        let mut tiles = vec![];
        loop {
            let line = reader.read_line();
            if line.is_none() || line.unwrap().is_empty() {
                break;
            }
            let line = line.unwrap();
            let row = line.iter().map(|c| {
                match c {
                    b'.' => Tile::new(TileKind::Empty),
                    b'|' => Tile::new(TileKind::VerticalSplitter),
                    b'-' => Tile::new(TileKind::HorizontalSplitter),
                    b'/' => Tile::new(TileKind::Forward),
                    b'\\' => Tile::new(TileKind::Backward),
                    _ => panic!("Unexpected tile kind: {}", *c as char)
                }
            }).collect::<Vec<_>>();
            tiles.push(row);
        }

        Self {
            tiles,
        }
    }

    pub fn reset(&mut self) {
        for row in &mut self.tiles {
            for tile in row {
                tile.reset();
            }
        }
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn count_visited(&self) -> usize {
        self.tiles.iter().map(|row| {
            row.iter().filter(|tile| tile.visited()).count()
        }).sum()
    }

    pub fn cast_ray(&mut self, x: usize, y: usize, initial_direction: Direction) {
        let mut visit_next_stack = vec![(x, y, initial_direction)];

        while let Some((x, y, direction)) = visit_next_stack.pop() {
            if x >= self.width() || y >= self.height() {
                continue;
            }
            let mut tile = &mut self.tiles[y][x];
            if tile.visited_from(&direction) {
                continue;
            }
            tile.visit(&direction);

            match tile.kind {
                TileKind::Empty => {
                    match direction {
                        Direction::North => {
                            visit_next_stack.push((x, y - 1, Direction::North));
                        },
                        Direction::South => {
                            visit_next_stack.push((x, y + 1, Direction::South));
                        },
                        Direction::East => {
                            visit_next_stack.push((x + 1, y, Direction::East));
                        },
                        Direction::West => {
                            visit_next_stack.push((x - 1, y, Direction::West));
                        },
                    }
                },
                TileKind::VerticalSplitter => {
                    match direction {
                        Direction::North => {
                            visit_next_stack.push((x, y - 1, Direction::North));
                        },
                        Direction::South => {
                            visit_next_stack.push((x, y + 1, Direction::South));
                        },
                        Direction::East | Direction::West => {
                            visit_next_stack.push((x, y - 1, Direction::North));
                            visit_next_stack.push((x, y + 1, Direction::South));
                        },
                    }
                },
                TileKind::HorizontalSplitter => {
                    match direction {
                        Direction::North | Direction::South => {
                            visit_next_stack.push((x + 1, y, Direction::East));
                            visit_next_stack.push((x - 1, y, Direction::West));
                        },
                        Direction::East => {
                            visit_next_stack.push((x + 1, y, Direction::East));
                        },
                        Direction::West => {
                            visit_next_stack.push((x - 1, y, Direction::West));
                        },
                    }
                },
                TileKind::Forward => {
                    match direction {
                        Direction::North => {
                            visit_next_stack.push((x + 1, y, Direction::East));
                        },
                        Direction::South => {
                            visit_next_stack.push((x - 1, y, Direction::West));
                        },
                        Direction::East => {
                            visit_next_stack.push((x, y - 1, Direction::North));
                        },
                        Direction::West => {
                            visit_next_stack.push((x, y + 1, Direction::South));
                        },
                    }
                },
                TileKind::Backward => {
                    match direction {
                        Direction::North => {
                            visit_next_stack.push((x - 1, y, Direction::West));
                        },
                        Direction::South => {
                            visit_next_stack.push((x + 1, y, Direction::East));
                        },
                        Direction::East => {
                            visit_next_stack.push((x, y + 1, Direction::South));
                        },
                        Direction::West => {
                            visit_next_stack.push((x, y - 1, Direction::North));
                        },
                    }
                },
                _ => panic!("Unexpected tile kind: {}", tile.kind),
            }
        }
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

        map.cast_ray(0, 0, Direction::East);

        map.count_visited() as u64
    }
}

#[derive(Parser, Debug)]
pub struct Day16a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day16a {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day 16 A: {answer}");
        Ok(())
    }
}
