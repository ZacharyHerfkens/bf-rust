use crate::compile::{Instruction, compile};

/// Run a brainfuck program
pub fn run<I, O>(
    program: String,
    mut input: I,
    mut output: O,
) -> Result<(), String> 
where
    I: std::io::Read,
    O: std::io::Write,
{
    let mut memory = [0u8; 2usize.pow(15)];
    let mut m_ptr = 0;
    let mut i_ptr = 0;
    let byte_prog = compile(&program)?;

    while i_ptr < byte_prog.len() {
        let instr = Instruction::parse_from_buf(&byte_prog, i_ptr).expect("Invalid Opcode encountered");
        i_ptr += instr.byte_size();

        match instr {
            Instruction::IncCell => {
                memory[m_ptr] = memory[m_ptr].wrapping_add(1)
            }
            Instruction::DecCell => {
                memory[m_ptr] = memory[m_ptr].wrapping_sub(1)
            }
            Instruction::IncPtr => {
                m_ptr += 1;
                if m_ptr > memory.len() {
                    return Err("cell pointer out of bounds".to_owned());
                }
            }
            Instruction::DecPtr => {
                m_ptr = m_ptr.checked_sub(1).ok_or_else(|| "cell pointer out of bounds".to_owned())?;
            }
            Instruction::Read => {
                memory[m_ptr] = read_byte(&mut input)?;
            }

            Instruction::Write => {
                write_byte(&mut output, memory[m_ptr])?;
            }
            Instruction::LoopStart(offset) => {
                if memory[m_ptr] == 0 {
                    i_ptr = offset;
                }
            }
            Instruction::LoopEnd(offset) => {
                if memory[m_ptr] != 0 {
                    i_ptr = offset;
                }
            }
        }
    }

    Ok(())
}

fn read_byte(input: &mut impl std::io::Read) -> Result<u8, String> {
    let mut buf = [0u8; 1];
    input.read_exact(&mut buf).map_err(|e| e.to_string())?;
    Ok(buf[0])
}

fn write_byte(output: &mut impl std::io::Write, byte: u8) -> Result<(), String> {
    output.write_all(&[byte]).map_err(|e| e.to_string())?;
    Ok(())
}