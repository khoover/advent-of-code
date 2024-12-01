use crate::utils::*;
use anyhow::{Context, Error, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use core::hint::assert_unchecked;
use nom::{
    bytes::complete::{tag, take},
    character::complete::newline,
    multi::{fold_many1, many1},
    sequence::{delimited, preceded, separated_pair},
    Parser,
};
use std::{
    fmt::{Display, Write},
    ops::Index,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn nom(input: &str) -> StrIResult<'_, Self> {
        take(1_usize)
            .map_res(|s: &str| match s.as_bytes()[0] {
                b'L' => Ok(Self::Left),
                b'R' => Ok(Self::Right),
                c => Err(Error::msg(format!("Invalid byte {c}"))),
            })
            .parse(input)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct NodeTag([u8; 3]);

impl NodeTag {
    fn nom(input: &str) -> StrIResult<'_, Self> {
        take(3_usize)
            .map_res(|s: &str| {
                let bytes = s.as_bytes();
                Ok::<Self, Error>(Self([
                    Self::convert_byte(bytes[0])?,
                    Self::convert_byte(bytes[1])?,
                    Self::convert_byte(bytes[2])?,
                ]))
            })
            .parse(input)
    }

    fn convert_byte(byte: u8) -> Result<u8> {
        if byte.is_ascii_uppercase() {
            Ok(byte - b'A')
        } else {
            Err(Error::msg(format!("Byte {byte} is out of range")))
        }
    }

    // SAFETY: We only construct this via Self::nom, which uses Self::convert_byte to map the input byte into 0..26
    fn assume_bounds(self) {
        unsafe {
            assert_unchecked(self.0[0] < 26);
            assert_unchecked(self.0[1] < 26);
            assert_unchecked(self.0[2] < 26);
        }
    }
}

impl Display for NodeTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.assume_bounds();
        for c in self
            .0
            .iter()
            .copied()
            .map(|b| char::from_u32((b + b'A') as u32).unwrap())
        {
            f.write_char(c)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Leaf {
    left: NodeTag,
    right: NodeTag,
}

impl Leaf {
    fn nom(input: &str) -> StrIResult<'_, Self> {
        delimited(
            tag("("),
            separated_pair(NodeTag::nom, tag(", "), NodeTag::nom),
            tag(")"),
        )
        .map(|(left, right)| Self { left, right })
        .parse(input)
    }
}

impl Index<Direction> for Leaf {
    type Output = NodeTag;
    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct InternalNode<T> {
    children: [T; 26],
}

impl<T: Default> InternalNode<T> {
    fn new() -> Self {
        Self::default()
    }
}

type TagMap = InternalNode<InternalNode<InternalNode<Option<Leaf>>>>;

impl TagMap {
    fn insert(&mut self, tag: NodeTag, value: Leaf) -> Option<Leaf> {
        tag.assume_bounds();
        let first = &mut self.children[tag.0[0] as usize];
        let second = &mut first.children[tag.0[1] as usize];
        second.children[tag.0[2] as usize].replace(value)
    }

    fn get(&self, tag: NodeTag) -> Option<Leaf> {
        tag.assume_bounds();
        let first = &self.children[tag.0[0] as usize];
        let second = &first.children[tag.0[1] as usize];
        second.children[tag.0[2] as usize]
    }

    fn list_part2_nodes(&self) -> impl Iterator<Item = NodeTag> + use<'_> {
        self.children.iter().enumerate().flat_map(|(a, first)| {
            first
                .children
                .iter()
                .enumerate()
                .filter_map(move |(b, second)| {
                    if second.children[0].is_some() {
                        Some(NodeTag([a as u8, b as u8, 0]))
                    } else {
                        None
                    }
                })
        })
    }

    #[allow(unused)]
    fn entries(&self) -> impl Iterator<Item = (NodeTag, Leaf)> + use<'_> {
        self.children.iter().enumerate().flat_map(|(a, first)| {
            first
                .children
                .iter()
                .enumerate()
                .flat_map(move |(b, second)| {
                    second.children.iter().copied().enumerate().filter_map(
                        move |(c, maybe_leaf)| {
                            maybe_leaf.map(|leaf| (NodeTag([a as u8, b as u8, c as u8]), leaf))
                        },
                    )
                })
        })
    }
}

#[aoc_generator(day8)]
fn day8_gen(input: &str) -> Result<(Vec<Direction>, Box<TagMap>)> {
    run_parse(
        input,
        separated_pair(
            many1(Direction::nom),
            newline,
            fold_many1(
                preceded(
                    newline,
                    ws(separated_pair(NodeTag::nom, tag(" = "), Leaf::nom)),
                ),
                || Box::new(TagMap::new()),
                |mut tag_map, (tag, leaf)| {
                    tag_map.insert(tag, leaf);
                    tag_map
                },
            ),
        ),
    )
}

#[aoc(day8, part1)]
fn day8_part1(input: &(Vec<Direction>, Box<TagMap>)) -> Result<u64> {
    let (dirs, map) = input;

    let mut steps = 0_u64;
    let mut current_leaf = map.get(NodeTag([0; 3])).context("Missing starting node")?;
    let mut iter = dirs.iter().copied().cycle();

    loop {
        let dir = iter.next().context("Cycle ended")?;
        steps += 1;
        let next_tag = current_leaf[dir];
        if next_tag.0 == [25; 3] {
            break;
        }
        current_leaf = map.get(next_tag).context("Missing a tag")?;
    }

    Ok(steps)
}

#[aoc(day8, part2)]
fn day8_part2(input: &(Vec<Direction>, Box<TagMap>)) -> Result<u64> {
    let (dirs, map) = input;

    let res = map
        .list_part2_nodes()
        .map(|mut node| {
            let mut steps = 0_u64;
            let mut dir_iter = dirs.iter().copied().cycle();
            while node.0[2] != b'Z' - b'A' {
                steps += 1;
                node = take_step(map, node, &mut dir_iter)?;
            }
            Ok::<u64, Error>(steps)
        })
        .try_fold(0, |a, b| if a == 0 { b } else { Ok(lcm(a, b?)) })?;
    if res == 0 {
        Err(Error::msg("No starting tags given"))
    } else {
        Ok(res)
    }
}

fn take_step(
    map: &TagMap,
    current_tag: NodeTag,
    direction: &mut impl Iterator<Item = Direction>,
) -> Result<NodeTag> {
    map.get(current_tag)
        .context("Missing a tag")
        .and_then(|leaf| Ok(leaf[direction.next().context("direction iter empty")?]))
}

fn lcm(a: u64, b: u64) -> u64 {
    let gcd = gcd(a, b);
    a * (b / gcd)
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut big = a.max(b);
    let mut small = a.min(b);
    while small > 0 {
        big = big.rem_euclid(small);
        std::mem::swap(&mut big, &mut small);
    }
    big
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use std::path::Path;

    static INPUT: &'static str = include_str!("../input/2023/day8.txt");
    static THIS_FILE: &'static str = file!();

    #[test]
    fn generate_dotgraph() -> Result<()> {
        let path = Path::new(THIS_FILE)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("output/day8.dot");
        let mut f = BufWriter::new(File::create(path)?);
        writeln!(f, "digraph day8 {{")?;

        let (_, map) = day8_gen(INPUT)?;
        for (tag, leaf) in map.entries() {
            writeln!(f, "  {} -> {}", tag, leaf.left)?;
            writeln!(f, "  {} -> {}", tag, leaf.right)?;
        }

        writeln!(f, "}}")?;
        Ok(())
    }

    #[test]
    fn get_phase_and_interval() -> Result<()> {
        let (dirs, map) = day8_gen(INPUT)?;
        let starting_nodes = map.list_part2_nodes();

        for start in starting_nodes {
            let mut phase = 0;
            let mut current = start;
            let mut iter = dirs.iter().copied().cycle();
            loop {
                phase += 1;
                current = take_step(&map, current, &mut iter)?;
                if current.0[2] == b'Z' - b'A' {
                    break;
                }
            }
            let mut interval = 0;
            loop {
                interval += 1;
                current = take_step(&map, current, &mut iter)?;
                if current.0[2] == b'Z' - b'A' {
                    break;
                }
            }
            println!("Start {} has interval {}, phase {}", start, interval, phase);
        }

        Ok(())
    }
}
