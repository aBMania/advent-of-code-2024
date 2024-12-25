use std::mem;
use itertools::Itertools;
use std::str::FromStr;

advent_of_code::solution!(17);

#[derive(Debug, Default, Clone)]
struct Program {
    a: u64,
    b: u64,
    c: u64,
    instructions: Vec<u64>,
    pc: usize,
    out: Vec<u64>,
}

// 2,4: b = a % 8 (last 3 bits)
// 1,2: b = (b xor 0b10) flip 2nd digit
// 7,5: c = a / (2 ^ b)
// 4,1: b = b xor c
// 1,3: b = (b xor 0b11) flip 2 first digits
// 5,5: out = (b % 8) (last 3 bits)
// 0,3: a = a / 8
// 3,0: if a != 0, loop

#[derive(Debug)]
struct ParseProgramError;

impl FromStr for Program {
    type Err = ParseProgramError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (registers, instructions) = s.split_once("\n\n").ok_or(ParseProgramError)?;

        let (a, b, c) = registers
            .split([':', '\n'])
            .skip(1)
            .step_by(2)
            .filter_map(|s| s.trim().parse::<u64>().ok())
            .next_tuple()
            .ok_or(ParseProgramError)?;

        let instructions = instructions
            .split([' ', ','])
            .skip(1)
            .map(|s| s.trim().parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseProgramError)?;

        let mut program = Program::default();
        program.a = a;
        program.b = b;
        program.c = c;
        program.instructions = instructions;
        Ok(program)
    }
}

impl Program {
    #[inline]
    fn opcode(&self) -> u64 {
        self.instructions[self.pc]
    }

    #[inline]
    fn literal_operand(&self) -> u64 {
        self.instructions[self.pc + 1]
    }

    #[inline]
    fn combo_operand(&self) -> u64 {
        match self.literal_operand() {
            operand @ (0 | 1 | 2 | 3) => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
    fn run(&mut self) {
        while self.pc < self.instructions.len() {
            match self.opcode() {
                0 => self.adv(),
                1 => self.bxl(),
                2 => self.bst(),
                3 => self.jnz(),
                4 => self.bxc(),
                5 => self.out(),
                6 => self.bdv(),
                7 => self.cdv(),
                _ => unreachable!(),
            };
        }
    }

    fn adv(&mut self) {
        self.a = self.a / 2u64.pow(self.combo_operand() as u32);
        self.pc += 2;
    }
    fn bxl(&mut self) {
        self.b = self.b ^ self.literal_operand();
        self.pc += 2;
    }
    fn bst(&mut self) {
        self.b = self.combo_operand() % 8;
        self.pc += 2;
    }
    fn jnz(&mut self) {
        if self.a == 0 {
            self.pc += 2;
            return;
        }
        self.pc = self.literal_operand() as usize;
    }
    fn bxc(&mut self) {
        self.b = self.b ^ self.c;
        self.pc += 2;
    }
    fn out(&mut self) {
        self.out.push(self.combo_operand() % 8);
        self.pc += 2;
    }
    fn bdv(&mut self) {
        self.b = self.a / 2u64.pow(self.combo_operand() as u32);
        self.pc += 2;
    }
    fn cdv(&mut self) {
        self.c = self.a / 2u64.pow(self.combo_operand() as u32);
        self.pc += 2;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut program = input.parse::<Program>().ok()?;
    program.run();

    Some(
        program
            .out
            .iter()
            .rev()
            .enumerate()
            .map(|(pow, x)| x * 10u64.pow(pow as u32))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut program = input.parse::<Program>().ok()?;

    let a = 0b100 << ((program.instructions.len() - 1) * 3);

    let mut possibles_a = vec![a];

    for i in (0..program.instructions.len()).rev() {
        let mut tmp_possible_a = vec![];

        for j in (0..=0b111).rev() {
            for possible_a in &possibles_a {
                let tmp = *possible_a ^ (j << (i * 3));

                // Reset program
                program.out.clear();
                program.a = tmp;
                program.b = 0;
                program.c = 0;
                program.pc = 0;
                program.run();

                if program.out.len() != program.instructions.len() {
                    continue;
                }

                if program.out[i] == program.instructions[i] {
                    tmp_possible_a.push(tmp);
                }
            }
        }

        mem::swap(&mut possibles_a, &mut tmp_possible_a);
        tmp_possible_a.clear();
    }

    Some(*possibles_a.iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_1() {
        let mut program = Program::default();
        program.instructions = vec![2, 6];
        program.c = 9;
        program.run();
        assert_eq!(program.b, 1);
    }

    #[test]
    fn program_2() {
        let mut program = Program::default();
        program.instructions = vec![5, 0, 5, 1, 5, 4];
        program.a = 10;
        program.run();
        assert_eq!(program.out, vec![0, 1, 2]);
    }

    #[test]
    fn program_3() {
        let mut program = Program::default();
        program.instructions = vec![0, 1, 5, 4, 3, 0];
        program.a = 2024;
        program.run();
        assert_eq!(program.out, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(program.a, 0);
    }

    #[test]
    fn program_4() {
        let mut program = Program::default();
        program.instructions = vec![1, 7];
        program.b = 29;
        program.run();
        assert_eq!(program.b, 26);
    }

    #[test]
    fn program_5() {
        let mut program = Program::default();
        program.instructions = vec![4, 0];
        program.b = 2024;
        program.c = 43690;
        program.run();
        assert_eq!(program.b, 44354);
    }

    #[test]
    fn program_custom_1() {
        let mut program = Program::default();
        program.instructions = vec![2, 4, 1, 2, 7, 5, 4, 1, 1, 3, 5, 5, 0, 3, 3, 0];
        program.a = 37221261688308;
        program.run();
        assert_eq!(program.out, program.instructions);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4635635210));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
