#[derive(Debug)]
pub struct Interpreter {
    pub codes: Vec<i64>,
    position: i64,
    pub is_running: bool
}

#[derive(Debug, PartialEq)]
pub enum OpCode {
    Add,
    Multiply,
    Halt
}

impl Interpreter {
    pub fn new(codes: std::vec::Vec<i64>) -> Interpreter {
        Interpreter { codes, position: 0, is_running: true }
    }

    pub fn step(&mut self) {
        let op = self.parse_opcode(self.fetch(self.position));

        if op == OpCode::Halt {
            self.is_running = false;
            return;
        }

        let operand_1 = self.get_operand(self.position + 1);
        let operand_2 = self.get_operand(self.position + 2);

        let result = match op {
            OpCode::Add => {
                operand_1 + operand_2
            },
            OpCode::Multiply => {
                operand_1 * operand_2
            },
            OpCode::Halt => {
                0
            }
        };

        self.put(self.fetch(self.position + 3), result);

        self.position += 4;
    }

    fn get_operand(&self, pos: i64) -> i64 {
        self.fetch(self.fetch(pos))
    }

    fn put(&mut self, pos: i64, data: i64) {
        self.codes[pos as usize] = data;
    }

    pub fn fetch(&self, pos: i64) -> i64 {
        self.codes[pos as usize]
    }

    fn parse_opcode(&self, op: i64) -> OpCode {
        match op {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            99 => OpCode::Halt,
            _ => panic!("Unimplemented opcode: {}", op)
        }
    }
}
