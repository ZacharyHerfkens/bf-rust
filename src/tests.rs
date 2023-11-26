#[cfg(test)]
mod interpreter_tests {
    use crate::interpret::{self, run};

    #[test]
    fn test_write_a() {
        let mut bytes = Vec::new();
        let prog = "++++++++[>++++++++<-]>+.";

        run(prog, [].as_slice(), &mut bytes).unwrap();
        assert_eq!(String::from_utf8(bytes).unwrap(), "A");
    }

    #[test]
    fn test_read_back() {
        let mut bytes = Vec::new();
        let prog = ",[.,]";
        let input = "Hello, World!".as_bytes();

        run(prog, input, &mut bytes).unwrap();
        let out = String::from_utf8(bytes).unwrap();
        assert_eq!(out, "Hello, World!");
    }

    #[test]
    fn test_hello_world() {
        let mut bytes = Vec::new();
        let prog = include_str!("bf_examples/hello_world.bf");

        run(prog, [].as_slice(), &mut bytes).unwrap();
        let out = String::from_utf8(bytes).unwrap();
        assert_eq!(out, "Hello World!\n");
    }

    #[test]
    fn test_cell_oob_left() {
        let mut bytes = Vec::new();
        let prog = "<";

        match run(prog, [].as_slice(), &mut bytes) {
            Err(interpret::Error::CellOutOfBounds) => (),
            _ => panic!("Expected cell out of bounds error"),
        }
    }

    #[test]
    fn test_cell_oob_right() {
        let mut bytes = Vec::new();
        let prog = "+[>+]";

        match run(prog, [].as_slice(), &mut bytes) {
            Err(interpret::Error::CellOutOfBounds) => (),
            _ => panic!("Expected cell out of bounds error"),
        }
    }
}

#[cfg(test)]
mod compile_tests {
    use crate::compile::*;

    #[test]
    fn test_compile() {
        let prog = ",+[.,+]";
        let instrs = compile(prog).unwrap();
        let expected = vec![
            Instruction::Read,
            Instruction::Add(1),
            Instruction::LoopStart(7),
            Instruction::Write,
            Instruction::Read,
            Instruction::Add(1),
            Instruction::LoopEnd(3),
        ];

        assert_eq!(instrs, expected);
    }

    #[test]
    fn test_compile_unmatched_open_bracket() {
        let prog = "[";
        match compile(prog) {
            Err(e) if e.to_string() == "Unmatched '[' at position 0" => (),
            _ => panic!("Expected error"),
        }
    }

    #[test]
    fn test_compile_unmatched_close_bracket() {
        let prog = "]";
        match compile(prog) {
            Err(e) if e.to_string() == "Unmatched ']' at position 0" => (),
            _ => panic!("Expected error"),
        }
    }
}