use std::collections::{hash_map::Entry, VecDeque};

use arrayvec::ArrayVec;
use memchr::arch::all;

use super::*;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Clone, Debug)]
struct Gate {
    op: Operation,
    inputs: ArrayVec<bool, 2>,
    output: [u8; 3],
}

impl Gate {
    fn push_input(&mut self, input: bool) -> Option<([u8; 3], bool)> {
        self.inputs.push(input);
        (self.inputs.len() == 2).then(|| {
            (
                self.output,
                match self.op {
                    Operation::And => self.inputs[0] & self.inputs[1],
                    Operation::Or => self.inputs[0] | self.inputs[1],
                    Operation::Xor => self.inputs[0] ^ self.inputs[1],
                },
            )
        })
    }

    fn from_line(line: &str) -> (Self, ([u8; 3], [u8; 3])) {
        let bytes = line.as_bytes();
        let first_input = [bytes[0], bytes[1], bytes[2]];
        let (op, idx) = match bytes[4] {
            b'X' => (Operation::Xor, 8_usize),
            b'A' => (Operation::And, 8),
            b'O' => (Operation::Or, 7),
            _ => unreachable!(),
        };
        let second_input = [bytes[idx], bytes[idx + 1], bytes[idx + 2]];
        let output = [bytes[idx + 7], bytes[idx + 8], bytes[idx + 9]];
        (
            Self {
                op,
                output,
                inputs: ArrayVec::new(),
            },
            (first_input, second_input),
        )
    }
}

#[derive(Clone, Debug)]
enum Variable {
    Value(bool),
    Successors(Vec<usize>),
}

impl Variable {
    fn push_successor(&mut self, successor: usize) {
        match self {
            Variable::Successors(v) => v.push(successor),
            Variable::Value(_) => unreachable!(),
        }
    }

    fn set_value(&mut self, value: bool) -> Vec<usize> {
        match std::mem::replace(self, Variable::Value(value)) {
            Variable::Successors(v) => v,
            Variable::Value(_) => unreachable!(),
        }
    }

    fn get_value(&self) -> Option<bool> {
        match self {
            Variable::Value(b) => Some(*b),
            Variable::Successors(_) => None,
        }
    }
}

fn compute_circuit(
    mut variables: FxHashMap<[u8; 3], Variable>,
    mut gates: Vec<Gate>,
    mut new_values: VecDeque<([u8; 3], bool)>,
) -> u64 {
    while let Some((tag, value)) = new_values.pop_front() {
        let gates_to_resolve = match variables.entry(tag) {
            Entry::Occupied(mut occupied) => occupied.get_mut().set_value(value),
            Entry::Vacant(vacant) => {
                vacant.insert(Variable::Value(value));
                continue;
            }
        };
        new_values.extend(
            gates_to_resolve
                .into_iter()
                .filter_map(|idx| gates[idx].push_input(value)),
        );
    }

    let mut z = 0_u64;
    for i in (b'0'..=b'4').rev() {
        z = (z << 1)
            | variables
                .get(&[b'z', b'6', i])
                .and_then(|var| var.get_value())
                .unwrap_or(false) as u64;
    }
    for i in (b'0'..=b'5').rev() {
        for j in (b'0'..=b'9').rev() {
            z = (z << 1)
                | variables
                    .get(&[b'z', i, j])
                    .and_then(|var| var.get_value())
                    .unwrap_or(false) as u64;
        }
    }
    z
}

#[aoc(day24, part1)]
pub fn part1(s: &str) -> u64 {
    let mut variables = FxHashMap::<[u8; 3], Variable>::default();
    let (initial_variables, gates) = s.split_once("\n\n").unwrap();
    let mut gates = gates
        .lines()
        .map(Gate::from_line)
        .enumerate()
        .map(|(idx, triple)| {
            variables
                .entry(triple.1 .0)
                .and_modify(|var| var.push_successor(idx))
                .or_insert(Variable::Successors(vec![idx; 1]));
            variables
                .entry(triple.1 .1)
                .and_modify(|var| var.push_successor(idx))
                .or_insert(Variable::Successors(vec![idx; 1]));
            triple.0
        })
        .collect::<Vec<_>>();
    let mut new_values = initial_variables
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let tag = [bytes[0], bytes[1], bytes[2]];
            let value = match bytes[5] {
                b'0' => false,
                b'1' => true,
                _ => unreachable!(),
            };
            (tag, value)
        })
        .collect::<VecDeque<_>>();

    compute_circuit(variables, gates, new_values)
}

#[aoc(day24, part2)]
pub fn part2(s: &str) -> String {
    let mut variables = FxHashMap::<[u8; 3], Variable>::default();
    let (initial_variables, gates) = s.split_once("\n\n").unwrap();
    let mut gates = gates
        .lines()
        .map(Gate::from_line)
        .enumerate()
        .map(|(idx, triple)| {
            variables
                .entry(triple.1 .0)
                .and_modify(|var| var.push_successor(idx))
                .or_insert(Variable::Successors(vec![idx; 1]));
            variables
                .entry(triple.1 .1)
                .and_modify(|var| var.push_successor(idx))
                .or_insert(Variable::Successors(vec![idx; 1]));
            triple.0
        })
        .collect::<Vec<_>>();
    let mut new_values = initial_variables
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let tag = [bytes[0], bytes[1], bytes[2]];
            let value = match bytes[5] {
                b'0' => false,
                b'1' => true,
                _ => unreachable!(),
            };
            (tag, value)
        })
        .collect::<VecDeque<_>>();

    let y_start = new_values.len() / 2;
    let mut x = 0_u64;
    let mut y = 0_u64;
    for i in (0..y_start).rev() {
        x = (x << 1) | new_values[i].1 as u64;
        y = (y << 1) | new_values[i + y_start].1 as u64;
    }
    let target_z = x + y;
    let actual_z = compute_circuit(variables.clone(), gates.clone(), new_values.clone());
    let mismatches = target_z ^ actual_z;
    for i in 0..64 {
        if (mismatches >> i) % 2 != 0 {
            println!("mismatch at bit {i}");
        }
    }

    "".to_owned()
}
