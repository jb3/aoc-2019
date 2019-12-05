#[derive(Debug)]
pub struct Interpreter {
    pub codes: Vec<i64>,
    position: i64,
    pub is_running: bool,
    input: Vec<i64>,
    pub last_output: i64
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Position,
    Immediate,
}

#[derive(Debug, PartialEq)]
pub enum OpCode {
    Add(Mode, Mode),
    Multiply(Mode, Mode),
    Halt,
    Input,
    Output(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode),
    Equals(Mode, Mode),
}

impl Interpreter {
    pub fn new(codes: Vec<i64>, mut input: Vec<i64>) -> Interpreter {
        input.reverse();
        Interpreter {
            codes,
            position: 0,
            is_running: true,
            input: input,
            last_output: 0
        }
    }

    pub fn step(&mut self) {
        let op = self.parse_opcode(self.fetch(self.position));

        if op == OpCode::Halt {
            self.is_running = false;
            return;
        }

        match op {
            OpCode::Add(p1_mode, p2_mode) => {
                let operand_1 = self.get_operand(self.position + 1, p1_mode);
                let operand_2 = self.get_operand(self.position + 2, p2_mode);
                let result = operand_1 + operand_2;
                self.put(self.fetch(self.position + 3), result);

                self.position += 4
            }
            OpCode::Multiply(p1_mode, p2_mode) => {
                let operand_1 = self.get_operand(self.position + 1, p1_mode);
                let operand_2 = self.get_operand(self.position + 2, p2_mode);
                let result = operand_1 * operand_2;
                self.put(self.fetch(self.position + 3), result);

                self.position += 4
            }
            OpCode::Halt => self.position += 1,
            OpCode::Input => {
                let inp = self.input.pop().expect("No inputs left");
                self.put(self.fetch(self.position + 1), inp);

                self.position += 2;
            }
            OpCode::Output(fetch_mode) => {
                let output = self.get_operand(self.position + 1, fetch_mode);
                self.last_output = output;

                self.position += 2;
            },
            OpCode::JumpIfTrue(p1_mode, p2_mode) => {
                let comparison = self.get_operand(self.position + 1, p1_mode);
                let to_jump = self.get_operand(self.position + 2, p2_mode);

                if comparison != 0 {
                    self.position = to_jump;
                } else {
                    self.position += 3;
                }
            },
            OpCode::JumpIfFalse(p1_mode, p2_mode) => {
                let comparison = self.get_operand(self.position + 1, p1_mode);
                let to_jump = self.get_operand(self.position + 2, p2_mode);

                if comparison == 0 {
                    self.position = to_jump;
                } else {
                    self.position += 3;
                }
            },
            OpCode::LessThan(p1_mode, p2_mode) => {
                let comparison_1 = self.get_operand(self.position + 1, p1_mode);
                let comparison_2 = self.get_operand(self.position + 2, p2_mode);
                let store_loc = self.get_operand(self.position + 3, Mode::Immediate);

                if comparison_1 < comparison_2 {
                    self.put(store_loc, 1)
                } else {
                    self.put(store_loc, 0)
                }

                self.position += 4;
            }
            OpCode::Equals(p1_mode, p2_mode) => {
                let comparison_1 = self.get_operand(self.position + 1, p1_mode);
                let comparison_2 = self.get_operand(self.position + 2, p2_mode);
                let store_loc = self.get_operand(self.position + 3, Mode::Immediate);

                if comparison_1 == comparison_2 {
                    self.put(store_loc, 1)
                } else {
                    self.put(store_loc, 0)
                }

                self.position += 4;
            }
        };
    }

    fn get_operand(&self, pos: i64, mode: Mode) -> i64 {
        let mut res = self.fetch(pos);

        if mode == Mode::Position {
            res = self.fetch(res);
        }

        res
    }

    fn put(&mut self, pos: i64, data: i64) {
        self.codes[pos as usize] = data;
    }

    fn get_digits(&self, number: i64) -> (Mode, Mode, Mode, i64) {
        let mut digits: Vec<i64> = number
            .to_string()
            .chars()
            .filter_map(|x| x.to_digit(10))
            .map(|x| (x as i64))
            .collect();

        digits.reverse();

        while digits.len() < 5 {
            digits.push(0);
        }

        digits.reverse();

        (
            self.parse_mode(digits[2]),
            self.parse_mode(digits[1]),
            self.parse_mode(digits[0]),
            digits[3] * 10 + digits[4],
        )
    }

    pub fn fetch(&self, pos: i64) -> i64 {
        self.codes[pos as usize]
    }

    fn parse_mode(&self, mode: i64) -> Mode {
        if mode == 1 {
            Mode::Immediate
        } else {
            Mode::Position
        }
    }

    fn parse_opcode(&self, op: i64) -> OpCode {
        let op = self.get_digits(op);

        match op {
            (p1_mode, p2_mode, _, 1) => OpCode::Add(p1_mode, p2_mode),
            (p1_mode, p2_mode, _, 2) => OpCode::Multiply(p1_mode, p2_mode),
            (_, _, _, 3) => OpCode::Input,
            (p1_mode, _, _, 4) => OpCode::Output(p1_mode),
            (p1_mode, p2_mode, _, 5) => OpCode::JumpIfTrue(p1_mode, p2_mode),
            (p1_mode, p2_mode, _, 6) => OpCode::JumpIfFalse(p1_mode, p2_mode),
            (p1_mode, p2_mode, _, 7) => OpCode::LessThan(p1_mode, p2_mode),
            (p1_mode, p2_mode, _, 8) => OpCode::Equals(p1_mode, p2_mode),
            (_, _, _, 99) => OpCode::Halt,
            _ => panic!("Unimplemented opcode: {:?}", op),
        }
    }
}
