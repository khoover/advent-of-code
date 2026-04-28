use std::fmt::Display;

use super::*;

use nom::{
    bytes::complete::{tag, take},
    character::complete::{newline, u64},
    combinator::consumed,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
    Parser,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Combo {
    Literal0,
    Literal1,
    Literal2,
    Literal3,
    RegisterA,
    RegisterB,
    RegisterC,
}

impl Combo {
    fn from_byte(b: u8) -> Self {
        match b {
            b'0' => Self::Literal0,
            b'1' => Self::Literal1,
            b'2' => Self::Literal2,
            b'3' => Self::Literal3,
            b'4' => Self::RegisterA,
            b'5' => Self::RegisterB,
            b'6' => Self::RegisterC,
            _ => unreachable!(),
        }
    }
}

impl Display for Combo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Literal0 => "0",
            Self::Literal1 => "1",
            Self::Literal2 => "2",
            Self::Literal3 => "3",
            Self::RegisterA => "a",
            Self::RegisterB => "b",
            Self::RegisterC => "c",
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Instruction {
    Adv(Combo),
    Bdv(Combo),
    Cdv(Combo),
    Bxl(u8),
    Bxc,
    Bst(Combo),
    Out(Combo),
    Jnz(u8),
}

impl Instruction {
    fn nom(s: &str) -> StrIResult<'_, Self> {
        separated_pair(take::<usize, &str, _>(1_usize), tag(","), take(1_usize))
            .map(|(opcode, operand)| {
                let operand = operand.as_bytes()[0];
                match opcode.as_bytes()[0] {
                    b'0' => Self::Adv(Combo::from_byte(operand)),
                    b'1' => Self::Bxl(operand - b'0'),
                    b'2' => Self::Bst(Combo::from_byte(operand)),
                    b'3' => Self::Jnz(operand - b'0'),
                    b'4' => Self::Bxc,
                    b'5' => Self::Out(Combo::from_byte(operand)),
                    b'6' => Self::Bdv(Combo::from_byte(operand)),
                    b'7' => Self::Cdv(Combo::from_byte(operand)),
                    _ => unreachable!(),
                }
            })
            .parse(s)
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Adv(combo) => write!(f, "a = a >> {}", combo),
            Instruction::Bdv(combo) => write!(f, "b = a >> {}", combo),
            Instruction::Cdv(combo) => write!(f, "c = a >> {}", combo),
            Instruction::Bxl(lit) => write!(f, "b ^= {:#b}", lit),
            Instruction::Bxc => f.write_str("b ^= c"),
            Instruction::Bst(combo) => write!(f, "b = {} % 8", combo),
            Instruction::Out(combo) => write!(f, "print {}", combo),
            Instruction::Jnz(lit) => write!(f, "if a != 0: jmp {}", lit / 2),
        }
    }
}

#[derive(Clone, Debug)]
struct State {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    program_counter: usize,
}

impl State {
    fn advance(&mut self, instruction: Instruction) -> Option<u8> {
        let res = match instruction {
            Instruction::Adv(combo) => {
                self.register_a >>= self.combo_to_literal(combo);
                None
            }
            Instruction::Bdv(combo) => {
                self.register_b = self.register_a >> self.combo_to_literal(combo);
                None
            }
            Instruction::Cdv(combo) => {
                self.register_c = self.register_a >> self.combo_to_literal(combo);
                None
            }
            Instruction::Bxl(lit) => {
                self.register_b ^= lit as u64;
                None
            }
            Instruction::Bxc => {
                self.register_b ^= self.register_c;
                None
            }
            Instruction::Bst(combo) => {
                self.register_b = self.combo_to_literal(combo) % 8;
                None
            }
            Instruction::Jnz(lit) => {
                if self.register_a != 0 {
                    self.program_counter = lit as usize;
                    return None;
                }
                None
            }
            Instruction::Out(combo) => Some(self.combo_to_literal(combo) as u8 % 8),
        };
        self.program_counter += 2;
        res
    }

    fn combo_to_literal(&self, combo: Combo) -> u64 {
        match combo {
            Combo::Literal0 => 0,
            Combo::Literal1 => 1,
            Combo::Literal2 => 2,
            Combo::Literal3 => 3,
            Combo::RegisterA => self.register_a,
            Combo::RegisterB => self.register_b,
            Combo::RegisterC => self.register_c,
        }
    }
}

fn parse_register(s: &str) -> StrIResult<'_, u64> {
    preceded((tag("Register "), take(1_usize), tag(": ")), u64).parse(s)
}

#[aoc(day17, part1)]
pub fn part1(s: &str) -> Result<String> {
    let register_parser = (
        terminated(parse_register, newline),
        terminated(parse_register, newline),
        terminated(parse_register, newline),
    )
        .map(|(register_a, register_b, register_c)| State {
            register_a,
            register_b,
            register_c,
            program_counter: 0,
        });
    let (mut state, instructions) = run_parse(
        s,
        separated_pair(
            register_parser,
            newline,
            preceded(
                tag("Program: "),
                separated_list1(tag(","), Instruction::nom),
            ),
        ),
    )?;
    let mut output = Vec::new();
    while let Some(instruction) = instructions.get(state.program_counter / 2) {
        if let Some(new_output) = state.advance(*instruction) {
            output.push(new_output.to_string());
        }
    }

    Ok(output.join(","))
}

#[aoc(day17, part2)]
pub fn part2(s: &str) -> Result<u64> {
    let register_parser = (
        terminated(parse_register, newline),
        terminated(parse_register, newline),
        terminated(parse_register, newline),
    )
        .map(|(register_a, register_b, register_c)| State {
            register_a,
            register_b,
            register_c,
            program_counter: 0,
        });
    let (original_state, program) = run_parse(
        s,
        separated_pair(
            register_parser,
            newline,
            preceded(
                tag("Program: "),
                separated_list1(tag(","), consumed(Instruction::nom)),
            ),
        ),
    )?;
    let target_output: Vec<u8> = program
        .iter()
        .map(|(x, _)| x)
        .flat_map(|triple| {
            let bytes = triple.as_bytes();
            [bytes[0] - b'0', bytes[2] - b'0']
        })
        .collect();
    let instructions: Vec<Instruction> = program.into_iter().map(|(_, x)| x).collect();
    recursive(
        &original_state,
        &target_output,
        target_output.len() - 1,
        0,
        &instructions,
    )
    .context("Recursion failed to find answer")
}

fn recursive(
    original_state: &State,
    target_output: &[u8],
    suffix_start: usize,
    base_a: u64,
    instructions: &[Instruction],
) -> Option<u64> {
    if suffix_start == 0 {
        for a in base_a << 3..(base_a << 3) + 8 {
            let mut state = original_state.clone();
            state.register_a = a;
            let mut output = Vec::new();
            while let Some(instruction) = instructions.get(state.program_counter / 2) {
                if let Some(new_output) = state.advance(*instruction) {
                    output.push(new_output);
                }
            }
            if target_output == output {
                return Some(a);
            }
        }
        None
    } else {
        for a in base_a << 3..(base_a << 3) + 8 {
            let mut state = original_state.clone();
            state.register_a = a;
            let mut output = Vec::new();
            while let Some(instruction) = instructions.get(state.program_counter / 2) {
                if let Some(new_output) = state.advance(*instruction) {
                    output.push(new_output);
                }
            }
            if target_output[suffix_start..] == output {
                let recursive_result = recursive(
                    original_state,
                    target_output,
                    suffix_start - 1,
                    a,
                    instructions,
                );
                if recursive_result.is_some() {
                    return recursive_result;
                }
            }
        }
        None
    }
}
