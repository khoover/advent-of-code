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
use std::{ops::Index, thread::current};

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

    fn list_part2_nodes(&self) -> Vec<NodeTag> {
        self.children
            .iter()
            .enumerate()
            .flat_map(|(a, first)| {
                first
                    .children
                    .iter()
                    .enumerate()
                    .filter_map(move |(b, second)| {
                        if second.children[0].is_some() {
                            Some(TagMap([a as u8, b as u8, 0]))
                        } else {
                            None
                        }
                    })
            })
            .collect()
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

    let mut current_nodes = map.list_part2_nodes();
    if current_nodes.len() == 0 {
        return Err(Error::msg("No valid starting nodes"));
    }
    let mut steps = 0_u64;
    let mut iter = dirs.iter().copied().cycle();

    loop {
        let dir = iter.next().context("Cycle ended")?;
        steps += 1;
    }

    Ok(steps)
}

fn take_step(map: &TagMap, current_tag: NodeTag, direction: Direction) -> Option<NodeTag> {
    map.get(current_tag).map(|leaf| leaf[direction])
}
