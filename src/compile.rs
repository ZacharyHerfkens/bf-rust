type Error = Box<dyn std::error::Error>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    MovePtr(isize),
    Add(u8),
    Read,
    Write,
    LoopStart(usize),
    LoopEnd(usize),
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
                loop_stack.push(instrs.len());
            }
            ']' => {
                let start = loop_stack.pop().ok_or_else(|| {
                    format!("Unmatched ']' at position {}", i)
                })?;
                instrs.push(Instruction::LoopEnd(start));
                instrs[start - 1] = Instruction::LoopStart(instrs.len());
            }
            _ => (),
        }
    }

    if !loop_stack.is_empty() {
        return Err(format!("Unmatched '[' at position {}", loop_stack.last().unwrap() - 1).into());
    }

    Ok(instrs)
}