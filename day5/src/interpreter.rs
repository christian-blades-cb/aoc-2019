#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<usize> for ParameterMode {
    fn from(x: usize) -> Self {
        match x {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("invalid mode"),
        }
    }
}

/// instruction -> (opcode, (mode_pos1, mode_pos2, mode_pos3))
fn extract_opcode(x: usize) -> (usize, (ParameterMode, ParameterMode, ParameterMode)) {
    let opcode = (x % 10) + (((x / 10) % 10) * 10);
    let mode_1 = (x / 100) % 10;
    let mode_2 = (x / 1000) % 10;
    let mode_3 = (x / 10000) % 10;

    (opcode, (mode_1.into(), mode_2.into(), mode_3.into()))
}

/// decode
fn decode_param(n: usize, mode: ParameterMode, prog: &[isize], pc: usize) -> isize {
    let v = prog[pc + n];
    match mode {
        ParameterMode::Position => prog[v as usize],
        ParameterMode::Immediate => v,
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Default,
    Input(usize),
    Output(isize),
    Halt,
}

pub fn interpret(prog: &mut Vec<isize>, pc: usize) -> (usize, State) {
    use std::convert::TryInto;

    let instruction = prog[pc];
    let (op, modes) = extract_opcode(instruction.try_into().unwrap());
    // println!("{:?}", prog);
    match op {
        1 => {
            // sum
            let left = decode_param(1, modes.0, &prog, pc);
            let right = decode_param(2, modes.1, &prog, pc);
            let dest_pos: usize = prog[pc + 3].try_into().unwrap();

            let acc = left + right;
            dbg!("sum", left, right, dest_pos, acc);
            prog[dest_pos] = acc;

            let new_pc = pc + 4;
            (new_pc, State::Default)
        }
        2 => {
            // product
            let left = decode_param(1, modes.0, &prog, pc);
            let right = decode_param(2, modes.1, &prog, pc);
            let dest_pos: usize = prog[pc + 3].try_into().unwrap();

            let acc = left * right;
            dbg!("product", left, right, dest_pos, acc);
            prog[dest_pos] = acc;

            let new_pc = pc + 4;
            (new_pc, State::Default)
        }
        3 => {
            // save input to pos
            let dest_pos: usize = prog[pc + 1].try_into().unwrap();
            dbg!("input to", dest_pos);

            let new_pc = pc + 2;
            (new_pc, State::Input(dest_pos))
        }
        4 => {
            // output
            let v = decode_param(1, modes.0, &prog, pc);
            dbg!("output", v);

            let new_pc = pc + 2;

            (new_pc, State::Output(v))
        }
        5 => {
            // jmp if true
            let v = decode_param(1, modes.0, &prog, pc);
            let new_pc = match v {
                0 => pc + 3,
                _ => decode_param(2, modes.1, &prog, pc).try_into().unwrap(),
            };
            dbg!("jmp if true", v, new_pc);

            (new_pc, State::Default)
        }
        6 => {
            // jmp if false
            let v = decode_param(1, modes.0, &prog, pc);
            let new_pc = match v {
                0 => decode_param(2, modes.1, &prog, pc).try_into().unwrap(),
                _ => pc + 3,
            };

            (new_pc, State::Default)
        }
        7 => {
            // less than
            let l = decode_param(1, modes.0, &prog, pc);
            let r = decode_param(2, modes.1, &prog, pc);
            let dest_pos: usize = prog[pc + 3].try_into().unwrap();
            prog[dest_pos] = if l < r { 1 } else { 0 };

            let new_pc = pc + 4;

            (new_pc, State::Default)
        }
        8 => {
            // equals
            let l = decode_param(1, modes.0, &prog, pc);
            let r = decode_param(2, modes.1, &prog, pc);
            let dest_pos: usize = prog[pc + 3].try_into().unwrap();
            prog[dest_pos] = if l == r { 1 } else { 0 };

            let new_pc = pc + 4;

            (new_pc, State::Default)
        }
        99 => (pc, State::Halt),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_opcode() {
        let input: usize = 1002;
        let expected = (
            2,
            (
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position,
            ),
        );
        let actual = extract_opcode(input);
        assert_eq!(expected, actual);

        let input = 11002;
        let expected = (
            2,
            (
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Immediate,
            ),
        );
        let actual = extract_opcode(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_interpret() {
        let mut prog = vec![1101, 100, -1, 4, 0];
        let (pc, st) = interpret(&mut prog, 0);
        assert_eq!(4, pc);
        assert_eq!(99, prog[4]);
        assert_eq!(st, State::Default);
    }

    #[test]
    fn test_sum() {
        let mut input = vec![1101, 3, 4, 4, 99];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(4, pc);
        assert_eq!(State::Default, state);
        assert_eq!(input[4], 7);
    }

    #[test]
    fn test_product() {
        let mut input = vec![1102, 3, 4, 4, 99];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(4, pc);
        assert_eq!(State::Default, state);
        assert_eq!(input[4], 12);
    }

    #[test]
    fn test_store() {
        let mut input = vec![1103, 2, 3, 99];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(2, pc);
        assert_eq!(State::Input(2), state);

        let mut input = vec![3, 2, 3, 99];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(2, pc);
        assert_eq!(State::Input(2), state);
    }

    #[test]
    fn test_output() {
        let mut input = vec![1104, 2, 3, 4];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(2, pc);
        assert_eq!(State::Output(2), state);

        let mut input = vec![4, 2, 3, 4];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(2, pc);
        assert_eq!(State::Output(3), state);
    }

    #[test]
    fn test_jump_if_true() {
        let mut input = vec![1105, 0, 5];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(2, pc);
        assert_eq!(State::Default, state);

        let mut input = vec![1105, 1, 5];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(5, pc);
        assert_eq!(State::Default, state);

        let mut input = vec![1005, 3, 7, 0];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(2, pc);
        assert_eq!(State::Default, state);

        let mut input = vec![1005, 3, 7, 5];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(7, pc);
        assert_eq!(State::Default, state);

        let mut input = vec![5, 3, 4, 0, 7];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(2, pc);
        assert_eq!(State::Default, state);

        let mut input = vec![5, 3, 4, 5, 9];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(9, pc);
        assert_eq!(State::Default, state);
    }

    #[test]
    fn test_jump_if_false() {
        let mut input = vec![1106, 0, 5];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(5, pc);
        assert_eq!(State::Default, state);

        let mut input = vec![1106, 1, 5];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(2, pc);
        assert_eq!(State::Default, state);

        let mut input = vec![1006, 3, 7, 0];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(7, pc);
        assert_eq!(State::Default, state);

        let mut input = vec![1006, 3, 7, 5];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(2, pc);
        assert_eq!(State::Default, state);

        let mut input = vec![6, 3, 4, 0, 9];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(9, pc);
        assert_eq!(State::Default, state);

        let mut input = vec![6, 3, 4, 5, 9];
        let (pc, state) = interpret(&mut input, 0);
        assert_eq!(2, pc);
        assert_eq!(State::Default, state);
    }
}
