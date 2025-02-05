use std::{
    fs,
    io::{self, BufRead, Write},
    path::Path,
};

pub fn read_file<T: AsRef<Path>>(file_path: T) -> io::Result<Vec<i32>> {
    let memory = fs::read_to_string(file_path)?
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    Ok(memory)
}

struct ParamModes(u32);

impl ParamModes {
    fn next(&mut self) -> u8 {
        let mode = (self.0 % 10) as u8;
        self.0 /= 10;
        mode
    }
}

fn get_param(memory: &[i32], index: usize, mode: u8) -> i32 {
    let value = memory[index];
    match mode {
        0 => memory[usize::try_from(value).unwrap()],
        1 => value,
        _ => panic!("unknown parameter mode {mode}"),
    }
}

fn get_addr(memory: &[i32], index: usize) -> usize {
    memory[index].try_into().unwrap()
}

fn do_binop<F>(memory: &mut [i32], ip: usize, mut param_modes: ParamModes, f: F)
where
    F: Fn(i32, i32) -> i32,
{
    let param1 = get_param(memory, ip + 1, param_modes.next());
    let param2 = get_param(memory, ip + 2, param_modes.next());
    let addr = get_addr(memory, ip + 3);
    memory[addr] = f(param1, param2);
}

pub fn run<R, W>(memory: &mut [i32], mut reader: R, mut writer: W)
where
    R: BufRead,
    W: Write,
{
    let mut ip = 0;
    while memory[ip] != 99 {
        let instruction: u32 = memory[ip].try_into().unwrap();
        let opcode = instruction % 100;
        let mut param_modes = ParamModes(instruction / 100);
        match opcode {
            1 => {
                do_binop(memory, ip, param_modes, |x, y| x + y);
                ip += 4;
            }
            2 => {
                do_binop(memory, ip, param_modes, |x, y| x * y);
                ip += 4;
            }
            3 => {
                let addr = get_addr(memory, ip + 1);
                print!("Enter input: ");
                io::stdout().flush().unwrap();
                let mut buf = String::new();
                reader.read_line(&mut buf).unwrap();
                let input = buf.trim().parse().unwrap();
                memory[addr] = input;
                ip += 2;
            }
            4 => {
                let param = get_param(memory, ip + 1, param_modes.next());
                writeln!(writer, "{param}").unwrap();
                ip += 2;
            }
            5 => {
                let param1 = get_param(memory, ip + 1, param_modes.next());
                let param2 = get_param(memory, ip + 2, param_modes.next());
                if param1 != 0 {
                    ip = param2.try_into().unwrap();
                } else {
                    ip += 3;
                }
            }
            6 => {
                let param1 = get_param(memory, ip + 1, param_modes.next());
                let param2 = get_param(memory, ip + 2, param_modes.next());
                if param1 == 0 {
                    ip = param2.try_into().unwrap();
                } else {
                    ip += 3;
                }
            }
            7 => {
                do_binop(memory, ip, param_modes, |x, y| i32::from(x < y));
                ip += 4;
            }
            8 => {
                do_binop(memory, ip, param_modes, |x, y| i32::from(x == y));
                ip += 4;
            }
            _ => panic!("Unknown opcode ({opcode}) at address {ip}"),
        }
    }
}
