use std::{
    fs,
    io::{self, BufRead, Write},
    path::Path,
};

pub fn read_program_file<T: AsRef<Path>>(file_path: T) -> io::Result<Vec<i64>> {
    let memory = fs::read_to_string(file_path)?
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    Ok(memory)
}

#[derive(PartialEq)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

struct ParamModes(u32);

impl ParamModes {
    fn next(&mut self) -> ParamMode {
        let mode = self.0 % 10;
        self.0 /= 10;
        match mode {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("unknown parameter mode {mode}"),
        }
    }
}

#[derive(Clone)]
pub struct Program {
    ip: usize,
    relative_base: usize,
    memory: Vec<i64>,
}

impl Program {
    #[must_use]
    pub fn new(memory: Vec<i64>) -> Program {
        Program {
            ip: 0,
            relative_base: 0,
            memory,
        }
    }

    pub fn from_file<T: AsRef<Path>>(file_path: T) -> io::Result<Program> {
        let memory = read_program_file(file_path)?;
        Ok(Self::new(memory))
    }

    #[must_use]
    pub fn memory(&self) -> &[i64] {
        &self.memory
    }

    pub fn run<R, W>(&mut self, mut reader: R, mut writer: W)
    where
        R: BufRead,
        W: Write,
    {
        while self.memory[self.ip] != 99 {
            self.execute_instruction(&mut reader, &mut writer);
        }
    }

    /// Run with provided input until program halts or requires more input.
    /// Returns true if program halts or false if program requires more input.
    pub fn run_with_input<W: Write>(&mut self, input: i64, mut writer: W) -> bool {
        let reader = input.to_string().into_bytes();
        let mut input_used = false;
        loop {
            let instruction = self.memory[self.ip];
            if instruction == 99 {
                return true;
            } else if instruction == 3 {
                if input_used {
                    return false;
                }
                input_used = true;
            }
            self.execute_instruction(reader.as_slice(), &mut writer);
        }
    }

    fn get_param(&self, offset: usize, mode: ParamMode) -> i64 {
        let mut value = self.memory[self.ip + offset];
        if mode == ParamMode::Immediate {
            return value;
        }
        if mode == ParamMode::Relative {
            value += i64::try_from(self.relative_base).unwrap();
        }
        let addr: usize = value.try_into().unwrap();

        self.memory.get(addr).copied().unwrap_or(0)
    }

    fn get_addr(&self, offset: usize, mode: ParamMode) -> usize {
        let mut addr = self.memory[self.ip + offset];
        if mode == ParamMode::Relative {
            addr += i64::try_from(self.relative_base).unwrap();
        }
        addr.try_into().unwrap()
    }

    fn write(&mut self, addr: usize, value: i64) {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }
        self.memory[addr] = value;
    }

    fn do_binop<F>(&mut self, mut param_modes: ParamModes, f: F)
    where
        F: Fn(i64, i64) -> i64,
    {
        let param1 = self.get_param(1, param_modes.next());
        let param2 = self.get_param(2, param_modes.next());
        let addr = self.get_addr(3, param_modes.next());
        self.write(addr, f(param1, param2));
    }

    fn execute_instruction<R, W>(&mut self, mut reader: R, mut writer: W)
    where
        R: BufRead,
        W: Write,
    {
        let instruction: u32 = self.memory[self.ip].try_into().unwrap();
        let opcode = instruction % 100;
        let mut param_modes = ParamModes(instruction / 100);
        match opcode {
            // add
            1 => {
                self.do_binop(param_modes, |x, y| x + y);
                self.ip += 4;
            }
            // multiply
            2 => {
                self.do_binop(param_modes, |x, y| x * y);
                self.ip += 4;
            }
            // input
            3 => {
                let addr = self.get_addr(1, param_modes.next());
                let mut buf = String::new();
                reader.read_line(&mut buf).unwrap();
                let input = buf.trim().parse().unwrap();
                self.write(addr, input);
                self.ip += 2;
            }
            // output
            4 => {
                let param = self.get_param(1, param_modes.next());
                writeln!(writer, "{param}").unwrap();
                self.ip += 2;
            }
            // jump-if-true
            5 => {
                let param1 = self.get_param(1, param_modes.next());
                let param2 = self.get_param(2, param_modes.next());
                if param1 != 0 {
                    self.ip = param2.try_into().unwrap();
                } else {
                    self.ip += 3;
                }
            }
            // jump-if-false
            6 => {
                let param1 = self.get_param(1, param_modes.next());
                let param2 = self.get_param(2, param_modes.next());
                if param1 == 0 {
                    self.ip = param2.try_into().unwrap();
                } else {
                    self.ip += 3;
                }
            }
            // less than
            7 => {
                self.do_binop(param_modes, |x, y| i64::from(x < y));
                self.ip += 4;
            }
            // equals
            8 => {
                self.do_binop(param_modes, |x, y| i64::from(x == y));
                self.ip += 4;
            }
            // relative base offset
            9 => {
                let value = self.get_param(1, param_modes.next());
                let abs_value: usize = value.abs().try_into().unwrap();
                if value.is_negative() {
                    self.relative_base -= abs_value;
                } else {
                    self.relative_base += abs_value;
                }
                self.ip += 2;
            }
            _ => panic!("Unknown opcode ({opcode}) at address {}", self.ip),
        }
    }
}
