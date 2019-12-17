#[derive(Debug)]
pub struct Computer {
    pub memory: Vec<i64>,
    ix: usize,
    inputs: Vec<i64>,
    relative_base: i64,
}

#[derive(Debug)]
enum ParamMode {
    Immediate,
    Position,
    Relative,
}

#[derive(Debug)]
struct ParamModes {
    n: i64,
}

impl ParamModes {
    pub fn new(n: i64) -> Self {
        Self { n }
    }
    pub fn next(&mut self) -> ParamMode {
        let mode = match self.n % 10 {
            2 => ParamMode::Relative,
            1 => ParamMode::Immediate,
            _ => ParamMode::Position,
        };
        // println!("next mode({}): {:?}", self.n, mode);
        self.n = self.n / 10;
        mode
    }
}

#[derive(Debug)]
enum Operation {
    Add(i64, i64, usize),
    Multiply(i64, i64, usize),
    Input(usize),
    Output(i64),
    JumpTrue(i64, usize),
    JumpFalse(i64, usize),
    LessThan(i64, i64, usize),
    EqualTo(i64, i64, usize),
    RelativeBase(i64),
    Exit,
}

impl Computer {
    pub fn new(memory: Vec<i64>) -> Self {
        Self {
            memory,
            ix: 0,
            inputs: Vec::new(),
            relative_base: 0,
        }
    }

    pub fn add_input(&mut self, input: i64) {
        // println!("add_input({})", input);
        self.inputs.push(input);
    }

    fn write_mem(&mut self, ix: usize, val: i64) {
        // println!("write({}) to {}", val, ix);
        if ix >= self.memory.len() {
            self.memory
                .extend(std::iter::repeat(0).take(ix - self.memory.len() + 1));
        }
        self.memory[ix] = val;
    }

    fn read_mem(&mut self, ix: usize) -> i64 {
        // println!("read_mem({})...", ix);
        if ix >= self.memory.len() {
            0
        } else {
            self.memory[ix]
        }
    }

    fn read_next(&mut self) -> i64 {
        // println!("read_next()...");
        let mem = self.read_mem(self.ix);
        // println!("read_next(): {} = {}", self.ix, mem);
        self.ix += 1;
        return mem;
    }

    fn read_next_op(&mut self) -> Operation {
        let next = self.read_next();
        let mut modes = ParamModes::new(next / 100);
        // println!("next,modes: {},{:?}", next, modes);
        match next % 100 {
            1 => Operation::Add(
                self.read_next_param(&mut modes),
                self.read_next_param(&mut modes),
                self.read_next_address(&mut modes),
            ),
            2 => Operation::Multiply(
                self.read_next_param(&mut modes),
                self.read_next_param(&mut modes),
                self.read_next_address(&mut modes),
            ),
            3 => Operation::Input(self.read_next_address(&mut modes)),
            4 => Operation::Output(self.read_next_param(&mut modes)),
            5 => Operation::JumpTrue(
                self.read_next_param(&mut modes),
                self.read_next_param(&mut modes) as usize,
            ),
            6 => Operation::JumpFalse(
                self.read_next_param(&mut modes),
                self.read_next_param(&mut modes) as usize,
            ),
            7 => Operation::LessThan(
                self.read_next_param(&mut modes),
                self.read_next_param(&mut modes),
                self.read_next_address(&mut modes),
            ),
            8 => Operation::EqualTo(
                self.read_next_param(&mut modes),
                self.read_next_param(&mut modes),
                self.read_next_address(&mut modes),
            ),
            9 => Operation::RelativeBase(self.read_next_param(&mut modes)),
            _ => Operation::Exit,
        }
    }

    fn read_next_address(&mut self, mode: &mut ParamModes) -> usize {
        let address = self.ix;
        let val = self.read_next() as usize;
        // println!("read_next_address({:?}) - {},{}", mode, address, val);
        // println!("{:?}", self);
        match mode.next() {
            ParamMode::Immediate => panic!("immediate mode not allowed"),
            ParamMode::Position => val,
            ParamMode::Relative => (val as i64 + self.relative_base) as usize,
        }
    }

    fn read_next_param(&mut self, mode: &mut ParamModes) -> i64 {
        let val = self.read_next();
        // println!("read_next_param({:?}) - {}", mode, val);
        match mode.next() {
            ParamMode::Immediate => val,
            ParamMode::Position => self.read_mem(val as usize),
            ParamMode::Relative => self.read_mem((val + self.relative_base) as usize),
        }
    }

    pub fn run(&mut self) -> State {
        loop {
            // println!("{:?}", self);
            let next_op = self.read_next_op();
            // println!("{:?}", next_op);
            match next_op {
                Operation::Add(a, b, t) => self.write_mem(t, a + b),
                Operation::Multiply(a, b, t) => self.write_mem(t, a * b),
                Operation::Input(t) => {
                    if self.inputs.is_empty() {
                        self.ix -= 2;
                        return State::AwaitingInput;
                    } else {
                        let val = self.inputs.remove(0).to_owned();
                        self.write_mem(t, val);
                    }
                }
                Operation::Output(a) => {
                    return State::Output(a);
                }
                Operation::JumpTrue(a, i) => {
                    if a != 0 {
                        self.ix = i;
                    }
                }
                Operation::JumpFalse(a, i) => {
                    if a == 0 {
                        self.ix = i;
                    }
                }
                Operation::LessThan(a, b, t) => {
                    if (a < b) {
                        self.write_mem(t, 1);
                    } else {
                        self.write_mem(t, 0);
                    }
                }
                Operation::EqualTo(a, b, t) => {
                    if (a == b) {
                        self.write_mem(t, 1);
                    } else {
                        self.write_mem(t, 0);
                    }
                }
                Operation::RelativeBase(a) => {
                    self.relative_base += a;
                }
                Operation::Exit => {
                    return State::Complete;
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum State {
    Idle,
    AwaitingInput,
    Output(i64),
    Complete,
}
