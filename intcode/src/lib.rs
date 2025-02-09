use std::{
    fs,
    io::{self, BufRead, Write},
    path::Path,
};

pub fn read_program_file<T: AsRef<Path>>(file_path: T) -> io::Result<Vec<i32>> {
    let memory = fs::read_to_string(file_path)?
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    Ok(memory)
}

enum ParamMode {
    Position,
    Immediate,
}

struct ParamModes(u32);

impl ParamModes {
    fn next(&mut self) -> ParamMode {
        let mode = self.0 % 10;
        self.0 /= 10;
        match mode {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            _ => panic!("unknown parameter mode {mode}"),
        }
    }
}

#[derive(Clone)]
pub struct Program {
    ip: usize,
    memory: Vec<i32>,
}

impl Program {
    #[must_use]
    pub fn new(memory: Vec<i32>) -> Program {
        Program { ip: 0, memory }
    }

    pub fn from_file<T: AsRef<Path>>(file_path: T) -> io::Result<Program> {
        let memory = read_program_file(file_path)?;
        Ok(Self::new(memory))
    }

    #[must_use]
    pub fn memory(&self) -> &[i32] {
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
    pub fn run_with_input<W: Write>(&mut self, input: i32, mut writer: W) -> bool {
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

    fn get_param(&self, offset: usize, mode: ParamMode) -> i32 {
        let value = self.memory[self.ip + offset];
        match mode {
            ParamMode::Position => self.memory[usize::try_from(value).unwrap()],
            ParamMode::Immediate => value,
        }
    }

    fn get_addr(&self, offset: usize) -> usize {
        self.memory[self.ip + offset].try_into().unwrap()
    }

    fn do_binop<F>(&mut self, mut param_modes: ParamModes, f: F)
    where
        F: Fn(i32, i32) -> i32,
    {
        let param1 = self.get_param(1, param_modes.next());
        let param2 = self.get_param(2, param_modes.next());
        let addr = self.get_addr(3);
        self.memory[addr] = f(param1, param2);
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
                let addr = self.get_addr(1);
                let mut buf = String::new();
                reader.read_line(&mut buf).unwrap();
                let input = buf.trim().parse().unwrap();
                self.memory[addr] = input;
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
                self.do_binop(param_modes, |x, y| i32::from(x < y));
                self.ip += 4;
            }
            // equals
            8 => {
                self.do_binop(param_modes, |x, y| i32::from(x == y));
                self.ip += 4;
            }
            _ => panic!("Unknown opcode ({opcode}) at address {}", self.ip),
        }
    }
}
