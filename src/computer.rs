pub struct Computer {
    pub memory: Vec<i32>,
    ix: usize,
    param_modes: ParamModes,
    inputs: Vec<i32>,
    output: isize,
    state: State,
}

enum ParamMode {
    Immediate,
    Position,
}

struct ParamModes {
    n: i32,
}

impl ParamModes {
    pub fn new(n: i32) -> Self {
        Self { n }
    }
    pub fn next(&mut self) -> ParamMode {
        let mode = match self.n % 10 {
            1 => ParamMode::Position,
            _ => ParamMode::Immediate,
        };
        self.n = self.n / 10;
        mode
    }
}

enum Operation {
    Add(i32, i32, usize),
    Multiply(i32, i32, usize),
    Halt,
}

impl Computer {
    pub fn new(memory: Vec<i32>) -> Self {
        Self {
            memory,
            ix: 0,
            param_modes: ParamModes::new(0),
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
            _ => Operation::Halt,
        }
    }

    fn read_next_address(&mut self, mode: &mut ParamModes) -> usize {
        let address = (self.read_next()) as usize;
        match mode.next() {
            ParamMode::Immediate => address,
            ParamMode::Position => self.memory[address] as usize,
        }
    }

    fn read_next_param(&mut self, mode: &mut ParamModes) -> i32 {
        let address = self.read_next_address(mode);
        self.memory[address]
    }

    pub fn run(&mut self) -> State {
        match self.state {
            State::Complete => State::Complete,
            State::Idle | State::Running => self.run_program(),
            State::AwaitingInput => {
                if self.inputs.is_empty() {
                    State::AwaitingInput
                } else {
                    self.run_program()
                }
            }
        }
    }

    fn run_program(&mut self) -> State {
        self.state = State::Running;
        loop {
            match self.read_next_op() {
                Operation::Add(a, b, t) => {
                    self.memory[t] = a + b;
                }
                Operation::Multiply(a, b, t) => {
                    self.memory[t] = a * b;
                }
                _ => {
                    return State::Complete;
                }
            }
        }
    }
}

pub enum State {
    Idle,
    Running,
    AwaitingInput,
    Complete,
}
