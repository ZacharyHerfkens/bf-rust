
/// Brainfuck instruction words
pub enum Instruction {
    IncPtr,
    DecPtr,
    IncCell,
    DecCell,
    Read,
    Write,
    LoopStart(usize),
    LoopEnd(usize),
}

impl Instruction {
    pub fn write_to_buf(&self, buf: &mut Vec<u8>) {
        buf.push(self.op_code());
        match self {
            Self::LoopStart(i) | Self::LoopEnd(i) => {
                buf.extend((*i as u32).to_le_bytes())
            }
            _ => ()
        }
    }

    pub fn parse_from_buf(buf: &[u8], start: usize) -> Option<Self> {
        fn read_offset(buf: &[u8]) -> Option<usize> {
            let mut bytes = [0; 8];
            let operand = buf.get(..4)?;
            for i in 0..operand.len() {
                bytes[i] = operand[i];
            }
            Some(usize::from_le_bytes(bytes))
        }
        let op_code = buf[start];
        let instr = match op_code {
            0 => Self::IncPtr,
            1 => Self::DecPtr,
            2 => Self::IncCell,
            3 => Self::DecCell,
            4 => Self::Read,
            5 => Self::Write,
            6 => buf.get(start+1..).and_then(read_offset).map(Self::LoopStart)?,
            7 => buf.get(start+1..).and_then(read_offset).map(Self::LoopEnd)?,
            _ => return None
        };
        Some(instr)
    }

    pub fn op_code(&self) -> u8 {
        match self {
            Self::IncPtr => 0,
            Self::DecPtr => 1,
            Self::IncCell => 2,
            Self::DecCell => 3,
            Self::Read => 4,
            Self::Write => 5,
            Self::LoopStart(_) => 6,
            Self::LoopEnd(_) => 7,
        }
    }

    pub fn byte_size(&self) -> usize {
        1 + match self {
            Self::LoopStart(_) | Self::LoopEnd(_) => 4,
            _ => 0
        }
    }
}


pub fn compile(prog: &str) -> Result<Box<[u8]>, String> {
    let mut instrs = Vec::new();
    let mut loop_stack = Vec::new();
    let mut ptr = 0;

    for (i, c) in prog.chars().enumerate().filter(|(_, c)| "+-<>[],.".contains(*c)) {
        let instr = match c {
            '+' => Instruction::IncCell,
            '-' => Instruction::DecCell,
            '>' => Instruction::IncPtr,
            '<' => Instruction::DecPtr,
            ',' => Instruction::Read,
            '.' => Instruction::Write,
            '[' => {
                let ls = Instruction::LoopStart(0);
                loop_stack.push((instrs.len(), ptr+ls.byte_size()));
                ls
            }
            ']' => {
                let (start_pos, start_ptr) = loop_stack.pop()
                    .ok_or_else(|| format!("unmatched ']' at position {}", i))?;
                let le = Instruction::LoopEnd(start_ptr);
                instrs[start_pos] = Instruction::LoopStart(ptr+le.byte_size());
                le
            }
            _ => unreachable!("invalid character in program")
        };

        ptr += instr.byte_size();
        instrs.push(instr);
    }

    if !loop_stack.is_empty() {
        return Err(format!("unmatched '[' at position {}", loop_stack.pop().unwrap().0));
    }

    let byte_code = instrs.iter().fold(Vec::new(), |mut buf, instr| {
        instr.write_to_buf(&mut buf);
        buf
    });
    Ok(byte_code.into())
}