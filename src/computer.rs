#[derive(Debug)]
pub struct Computer {
    pub memory: Vec<i32>,
    ix: usize,
    // param_modes: ParamModes,
    inputs: Vec<i32>,
    pub output: i32,
    state: State,
}

#[derive(Debug)]
enum ParamMode {
    Immediate,
    Position,
}

#[derive(Debug)]
struct ParamModes {
    n: i32,
}

impl ParamModes {
    pub fn new(n: i32) -> Self {
        Self { n }
    }
    pub fn next(&mut self) -> ParamMode {
        let mode = match self.n % 10 {
            1 => ParamMode::Immediate,
            _ => ParamMode::Position,
        };
        self.n = self.n / 10;
        mode
    }
}

#[derive(Debug)]
enum Operation {
    Add(i32, i32, usize),
    Multiply(i32, i32, usize),
    Input(usize),
    Output(i32),
    JumpTrue(i32, usize),
    JumpFalse(i32, usize),
    LessThan(i32, i32, usize),
    EqualTo(i32, i32, usize),
    Exit,
}

impl Computer {
    pub fn new(memory: Vec<i32>) -> Self {
        Self {
            memory,
            ix: 0,
            // param_modes: ParamModes::new(0),
            inputs: Vec::new(),
            output: 0,
            state: State::Idle,
        }
    }

    pub fn add_input(&mut self, input: i32) {
        self.inputs.push(input);
    }

    fn read_next(&mut self) -> i32 {
        self.ix += 1;
        self.memory[self.ix - 1]
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
            _ => Operation::Exit,
        }
    }

    fn read_next_address(&mut self, mode: &mut ParamModes) -> usize {
        let address = self.ix;
        let val = self.read_next() as usize;
        // println!("read_next_address({:?}) - {},{}", mode, address, val);
        match mode.next() {
            ParamMode::Immediate => address,
            ParamMode::Position => val,
        }
    }

    fn read_next_param(&mut self, mode: &mut ParamModes) -> i32 {
        let val = self.read_next();
        match mode.next() {
            ParamMode::Immediate => val,
            ParamMode::Position => self.memory[val as usize],
        }
    }

    pub fn run(&mut self) -> State {
        match self.state {
            State::Complete => State::Complete,
            _ => self.run_program(),
        }
    }

    fn run_program(&mut self) -> State {
        self.state = State::Running;
        loop {
            let next_op = self.read_next_op();
            // println!("{:?}", next_op);
            match next_op {
                Operation::Add(a, b, t) => self.memory[t] = a + b,
                Operation::Multiply(a, b, t) => self.memory[t] = a * b,
                Operation::Input(t) => {
                    if self.inputs.is_empty() {
                        self.ix -= 2;
                        self.state = State::AwaitingInput;
                        return State::AwaitingInput;
                    } else {
                        self.memory[t] = self.inputs.remove(0);
                    }
                }
                Operation::Output(a) => self.output = a,
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
                        self.memory[t] = 1;
                    } else {
                        self.memory[t] = 0;
                    }
                }
                Operation::EqualTo(a, b, t) => {
                    if (a == b) {
                        self.memory[t] = 1;
                    } else {
                        self.memory[t] = 0;
                    }
                }
                Operation::Exit => {
                    self.state = State::Complete;
                    return State::Complete;
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum State {
    Idle,
    Running,
    AwaitingInput,
    Complete,
}
