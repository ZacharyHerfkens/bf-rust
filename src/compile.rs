#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    MovePtr(isize),
    Add(u8),
    Read,
    Write,
    LoopStart(usize),
    LoopEnd(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    UnmatchedOpenBracket(usize),
    UnmatchedCloseBracket(usize),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Error::*;
        match self {
            UnmatchedOpenBracket(i) => write!(f, "Unmatched '[' at position {}", i),
            UnmatchedCloseBracket(i) => write!(f, "Unmatched ']' at position {}", i),
        }
    }
}

pub fn compile(prog: &str) -> Result<Vec<Instruction>, Error> {
    let mut instrs = Vec::new();
    let mut loop_stack = Vec::new();

    for (i, c) in prog.chars().enumerate() {
        match c {
            '>' => instrs.push(Instruction::MovePtr(1)),
            '<' => instrs.push(Instruction::MovePtr(-1)),
            '+' => instrs.push(Instruction::Add(1)),
            '-' => instrs.push(Instruction::Add(u8::MAX)),
            '.' => instrs.push(Instruction::Write),
            ',' => instrs.push(Instruction::Read),
            '[' => {
                instrs.push(Instruction::LoopStart(usize::MAX));
                loop_stack.push((instrs.len(), i));
            }
            ']' => {
                let (start, _) = loop_stack.pop().ok_or(Error::UnmatchedCloseBracket(i))?;
                instrs.push(Instruction::LoopEnd(start));
                instrs[start - 1] = Instruction::LoopStart(instrs.len());
            }
            _ => (),
        }
    }

    if !loop_stack.is_empty() {
        let (_, i) = loop_stack.pop().unwrap();
        return Err(Error::UnmatchedOpenBracket(i));
    }

    Ok(instrs)
}