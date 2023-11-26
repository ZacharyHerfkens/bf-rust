use crate::compile::{self, Instruction, compile};

#[derive(Debug)]
pub enum Error {
    CellOutOfBounds,
    IoError(std::io::Error),
    CompileError(compile::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<compile::Error> for Error {
    fn from(err: compile::Error) -> Self {
        Self::CompileError(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self { 
            Self::CellOutOfBounds => write!(f, "cell out of bounds"),
            Self::IoError(err) => write!(f, "IO error: {}", err),
            Self::CompileError(err) => write!(f, "compile error: {}", err),
        }
    }
}

pub fn run<I, O>(prog: &str, mut input: I, mut output: O) -> Result<(), Error> 
where
    I: std::io::Read,
    O: std::io::Write,
{
    let instrs = compile(prog)?;
    let mut state = State::new(2usize.pow(16));

    while let Some(instr) = next_instr(&mut state, &instrs) {
        match instr {
            Instruction::MovePtr(v) => state.move_ptr(v)?,
            Instruction::Add(v) => state.add_cell(v),
            Instruction::Read => *state.cell() = read_byte(&mut input)?,
            Instruction::Write => write_byte(&mut output, *state.cell())?,
            Instruction::LoopStart(end) => {
                if *state.cell() == 0 {
                    state.jmp(end);
                }
            }
            Instruction::LoopEnd(start) => {
                if *state.cell() != 0 {
                    state.jmp(start);
                }
            }
        }
    }

    Ok(())
}

fn next_instr(state: &mut State, instrs: &[Instruction]) -> Option<Instruction> {
    let instr = instrs.get(state.instr_ptr)?;
    state.instr_ptr += 1;
    Some(*instr)
}

fn read_byte<I: std::io::Read>(input: &mut I) -> Result<u8, Error> {
    let mut buf = [0; 1];
    input.read(&mut buf)?;
    Ok(buf[0])
}

fn write_byte<O: std::io::Write>(output: &mut O, byte: u8) -> Result<(), Error> {
    output.write(&[byte])?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    mem: Box<[u8]>,
    mem_ptr: usize,
    instr_ptr: usize,
}


impl State {
    fn new(mem_size: usize) -> Self {
        Self {
            mem: vec![0; mem_size].into_boxed_slice(),
            mem_ptr: 0,
            instr_ptr: 0,
        }
    }

    fn cell(&mut self) -> &mut u8 {
        &mut self.mem[self.mem_ptr]
    }

    fn add_cell(&mut self, v: u8) {
        self.mem[self.mem_ptr] = self.mem[self.mem_ptr].wrapping_add(v);
    }

    fn move_ptr(&mut self, v: isize) -> Result<(), Error> {
        self.mem_ptr = self.mem_ptr.checked_add_signed(v).ok_or(Error::CellOutOfBounds)?;
        if self.mem_ptr >= self.mem.len() {
            return Err(Error::CellOutOfBounds);
        }
        Ok(())
    }

    fn jmp(&mut self, v: usize) {
        self.instr_ptr = v;
    }
}