use log::{debug, info};

// IntCodeComputer Section
enum DsRead {
    // Signals that indicate a read could not succeed. Maybe make these errors?
    Closed,
    NoData,
    // The actual value
    Data(i64),
}

/// DataStream is represents a shared buffer that a producer and consumer can write and read to.
pub struct DataStream {
    buffer: Memory,
    is_closed: bool,
    producer_ind: u64,
    consumer_ind: u64,
}

impl Default for DataStream {
    fn default() -> Self {
        DataStream::new()
    }
}

impl DataStream {
    pub fn new() -> Self {
        Self {
            buffer: Memory {
                memory: vec![0i64; 1000],
            },
            is_closed: false,
            producer_ind: 0,
            consumer_ind: 0,
        }
    }

    fn reset(&mut self) {
        self.buffer.clear();
        self.producer_ind = 0;
        self.consumer_ind = 0;
        self.is_closed = false;
    }

    // TODO: Maybe just use the macro
    fn copy(&self) -> Self {
        Self {
            buffer: Memory {
                memory: self.buffer.memory.clone(),
            },
            is_closed: self.is_closed,
            producer_ind: self.producer_ind,
            consumer_ind: self.consumer_ind,
        }
    }

    pub fn write(&mut self, val: i64) {
        self.buffer.write(self.producer_ind, val);
        self.producer_ind += 1;
    }

    fn read(&mut self) -> DsRead {
        if self.is_closed {
            return DsRead::Closed;
        }
        // Consumer is all caught up to the producer, so there is currently no data
        if self.consumer_ind == self.producer_ind {
            return DsRead::NoData;
        }
        let out = self.buffer.read(self.consumer_ind);
        self.consumer_ind += 1;
        DsRead::Data(out)
    }

    pub fn read_all(&mut self) -> Vec<i64> {
        let mut output = vec![];
        loop {
            let out = self.read();
            match out {
                DsRead::Closed | DsRead::NoData => {
                    return output;
                }
                DsRead::Data(d) => output.push(d),
            }
        }
    }

    pub fn close(&mut self) {
        self.is_closed = true;
    }
}

#[cfg(test)]
mod datastream_test {
    use crate::int_code::{DataStream, DsRead};

    #[test]
    fn test_basic_ds() {
        let mut ds = DataStream::new();
        // No data at first...
        assert!(matches!(ds.read(), DsRead::NoData));
        // We can read some after we've added data, but only once
        ds.write(5);
        assert!(matches!(ds.read(), DsRead::Data(5)));
        assert!(matches!(ds.read(), DsRead::NoData));

        // We can keep reading up to where it wrote
        ds.write(10);
        ds.write(20);
        assert!(matches!(ds.read(), DsRead::Data(10)));
        assert!(matches!(ds.read(), DsRead::Data(20)));
        assert!(matches!(ds.read(), DsRead::NoData));

        ds.close();
        assert!(matches!(ds.read(), DsRead::Closed));
    }
}

/// Memory manages the memory of the IntCodeComputer. It can read from address, or it can read from
/// pointer. It can also write to address and write to pointer
pub struct Memory {
    memory: Vec<i64>,
}

impl Memory {
    /// Returns the value at the specified address
    pub fn read(&self, addr: u64) -> i64 {
        self.memory[addr as usize]
    }

    fn clear(&mut self) {
        for i in 0..self.memory.len() {
            self.memory[i] = 0
        }
    }

    fn read_mode(&self, val: u64, rel_pos: u64, m: &ParamMode) -> i64 {
        match m {
            ParamMode::Position => self.read_ptr(val),
            ParamMode::Immediate => self.read(val),
            ParamMode::RelativeMode => {
                // Read out the value at the given address and then adjust it with the relative postition.
                // Then use that as the new address to read from.
                let val = (self.read(val) + rel_pos as i64) as u64;
                self.read(val)
            }
        }
    }

    /// Returns the value at the pointer. It will read out the value at the given address, then use
    /// that value itself as an address and return what that points to.
    fn read_ptr(&self, ptr: u64) -> i64 {
        let addr = self.read(ptr);
        if addr < 0 {
            panic!("cannot use negative value {} as pointer", addr)
        }
        self.read(addr as u64)
    }

    /// Writes the specified value at the specified address.
    fn write(&mut self, addr: u64, val: i64) {
        self.memory[addr as usize] = val;
    }
}

#[cfg(test)]
mod memory_tests {
    use crate::int_code::Memory;

    #[test]
    fn test_read() {
        let m = Memory {
            memory: vec![5, 4, 3, 2, 1],
        };
        assert_eq!(m.read(0), 5);
        assert_eq!(m.read(1), 4);
        assert_eq!(m.read(2), 3);
        assert_eq!(m.read(3), 2);
        assert_eq!(m.read(4), 1);
    }

    #[test]
    fn test_read_ptr() {
        let m = Memory {
            memory: vec![1, 2, 3, 4, 0],
        };
        assert_eq!(m.read_ptr(0), 2);
        assert_eq!(m.read_ptr(1), 3);
        assert_eq!(m.read_ptr(2), 4);
        assert_eq!(m.read_ptr(3), 0);
        assert_eq!(m.read_ptr(4), 1);
    }

    #[test]
    fn test_write() {
        let mut m = Memory { memory: vec![0] };
        m.write(0, 2);
        assert_eq!(m.memory[0], 2);
        m.write(0, 5);
        assert_eq!(m.memory[0], 5);
    }
}


/// IntCodeComputer is initialized with memory and executes instructions until it encounters the
/// end of program code. It does not validate the code.
pub struct IntCodeComputer {
    ptr: u64,
    memory: Memory,
    pub input: DataStream,
    rel_pos: u64,
    output: DataStream,
    state: ComputerState,
}

type BinaryModes = [ParamMode; 2];
type TrinaryModes = [ParamMode; 3];

enum Instruction {
    Add { modes: TrinaryModes },
    Mult { modes: TrinaryModes },
    ReadInput { modes: ParamMode },
    WriteOutput { modes: ParamMode },
    JumpIfTrue { modes: BinaryModes },
    JumpIfFalse { modes: BinaryModes },
    LessThan { modes: TrinaryModes },
    Equals { modes: TrinaryModes },
    AdjustRelativePosition { modes: ParamMode },
    End,
}

enum ParamMode {
    Position,
    Immediate,
    RelativeMode,
}

impl ParamMode {
    fn parse(v: i64) -> Self {
        match v {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::RelativeMode,
            _ => {
                panic!("unexpected val for param mode {}", v)
            }
        }
    }
}

type InstructionPointer = u64;

#[derive(Copy, Clone)]
enum ComputerState {
    Halted,
    ReadyForInstruction,
    WaitingForInput,
    Panic,
}

fn parse_instruction(val: i64) -> Instruction {
    let op_code = val % 100;
    match op_code {
        1 => Instruction::Add {
            modes: [
                ParamMode::parse((val / 100) % 10),
                ParamMode::parse((val / 1000) % 10),
                ParamMode::parse((val / 10000) % 10),
            ],
        },
        2 => Instruction::Mult {
            modes: [
                ParamMode::parse(val / 100 % 10),
                ParamMode::parse(val / 1000 % 10),
                ParamMode::parse(val / 10000 % 10),
            ],
        },
        3 => Instruction::ReadInput {
            modes: ParamMode::parse(val / 100 % 10),
        },
        4 => Instruction::WriteOutput {
            modes: ParamMode::parse(val / 100 % 10),
        },
        5 => Instruction::JumpIfTrue {
            modes: [
                ParamMode::parse((val / 100) % 10),
                ParamMode::parse((val / 1000) % 10),
            ],
        },
        6 => Instruction::JumpIfFalse {
            modes: [
                ParamMode::parse((val / 100) % 10),
                ParamMode::parse((val / 1000) % 10),
            ],
        },
        7 => Instruction::LessThan {
            modes: [
                ParamMode::parse((val / 100) % 10),
                ParamMode::parse((val / 1000) % 10),
                ParamMode::parse((val / 10000) % 10),
            ],
        },
        8 => Instruction::Equals {
            modes: [
                ParamMode::parse((val / 100) % 10),
                ParamMode::parse((val / 1000) % 10),
                ParamMode::parse((val / 10000) % 10),
            ],
        },
        9 => Instruction::AdjustRelativePosition {
            modes: ParamMode::parse((val / 100) % 10),
        },
        99 => Instruction::End,
        _ => panic!("unexpected val for opcode {}", op_code),
    }
}

fn pad_memory(memory: Vec<i64>) -> Vec<i64> {
    let mut new_memory = memory.clone();
    new_memory.resize(10000, 0);
    new_memory
}

impl IntCodeComputer {
    /// Returns an IntCodeComputer initialized with the given memory.
    pub fn new(memory: Vec<i64>) -> Self {
        Self {
            ptr: 0,
            memory: Memory { memory: pad_memory(memory) },
            input: DataStream::new(),
            rel_pos: 0,
            output: DataStream::new(),
            state: ComputerState::ReadyForInstruction,
        }
    }

    fn exec_add(&mut self, modes: TrinaryModes) {
        let (a, b, addr) = self.parse_trinary_op(modes);
        debug!("inst: ADD");
        debug!("a:    {}", a);
        debug!("b:    {}", b);
        debug!("addr: {}", addr);
        self.add(a, b, addr);
        self.ptr += 4;
        self.state = ComputerState::ReadyForInstruction;
    }

    fn exec_mult(&mut self, modes: TrinaryModes) {
        let (a, b, addr) = self.parse_trinary_op(modes);
        debug!("inst: MULT");
        debug!("a:    {}", a);
        debug!("b:    {}", b);
        debug!("addr: {}", addr);
        self.mult(a, b, addr);
        self.ptr += 4;
        self.state = ComputerState::ReadyForInstruction;
    }

    fn exec_read(&mut self, mode: ParamMode) {
        match self.input.read() {
            DsRead::Closed => {
                panic!("Reading from a closed data stream")
            }
            DsRead::NoData => {
                debug!("attempted to read from input but there was none available");
                self.state = ComputerState::WaitingForInput;
            }
            DsRead::Data(d) => {
                let addr = self.memory.read(self.ptr + 1);
                //                let addr = self.memory.read_mode(self.ptr + 1, self.rel_pos, &mode);
                debug!("inst: READ");
                debug!("addr: {}", addr);
                debug!("data: {}", d);
                match mode {
                    ParamMode::RelativeMode => {
                        self.memory.write((addr + self.rel_pos as i64) as u64, d);
                    }
                    _ => {
                        self.memory.write(addr as u64, d);
                    }
                }
                self.ptr += 2;
                self.state = ComputerState::ReadyForInstruction;
            }
        }
    }

    fn exec_write(&mut self, mode: ParamMode) {
        let val = self.parse_unary_op(&mode);
        debug!("inst: WRITE");
        debug!("val: {}", val);
        self.output.write(val);
        self.ptr += 2;
        self.state = ComputerState::ReadyForInstruction;
    }

    fn exec_jump_if_true(&mut self, modes: BinaryModes) {
        let (expr, addr) = self.parse_binary_op(modes);
        debug!("inst: JUMP_IF_TRUE");
        debug!("expr: {}", expr);
        debug!("addr: {}", addr);
        self.ptr = if expr != 0 {
            addr as u64
        } else {
            self.ptr + 3
        };
        self.state = ComputerState::ReadyForInstruction;
    }

    fn exec_jump_if_false(&mut self, modes: BinaryModes) {
        let (expr, addr) = self.parse_binary_op(modes);
        debug!("inst: JUMP_IF_FALSE");
        debug!("expr: {}", expr);
        debug!("addr: {}", addr);
        self.ptr = if expr == 0 {
            addr as u64
        } else {
            self.ptr + 3
        };
        self.state = ComputerState::ReadyForInstruction;
    }

    fn exec_less_than(&mut self, modes: TrinaryModes) {
        let (a, b, addr) = self.parse_trinary_op(modes);
        let val = if a < b { 1 } else { 0 };
        debug!("inst: LESS_THAN");
        debug!("addr: {}", addr);
        debug!("val: {}", val);
        self.memory.write(addr, val);
        self.ptr += 4;
        self.state = ComputerState::ReadyForInstruction;
    }

    fn exec_equals(&mut self, modes: TrinaryModes) {
        let (a, b, addr) = self.parse_trinary_op(modes);
        let val = if a == b { 1 } else { 0 };
        debug!("inst: EQUALS");
        debug!("addr: {}", addr);
        debug!("val: {}", val);
        self.memory.write(addr, val);
        self.ptr += 4;
        self.state = ComputerState::ReadyForInstruction;
    }

    fn exec_adjust_rel_pos(&mut self, mode: ParamMode) {
        let val = self.parse_unary_op(&mode);
        debug!("inst: ADJUST REL POS");
        debug!("val: {}", val);
        self.rel_pos = (self.rel_pos as i64 + val) as u64;
        self.ptr += 2;
        self.state = ComputerState::ReadyForInstruction;
    }

    /// execute evaluates a single instruction. It returns a code indicating whether the execution
    /// was successful.
    fn execute(&mut self) -> (ComputerState, InstructionPointer) {
        let last_ptr = self.ptr;
        debug!("===========================");
        debug!("Ptr:    {}", last_ptr);
        debug!("OpCode: {}", self.memory.read(self.ptr));
        let instruction = parse_instruction(self.memory.read(self.ptr));
        match instruction {
            Instruction::Add { modes } => { self.exec_add(modes); }
            Instruction::AdjustRelativePosition { modes } => { self.exec_adjust_rel_pos(modes); }
            Instruction::Mult { modes } => { self.exec_mult(modes); }
            Instruction::ReadInput { modes } => { self.exec_read(modes); }
            Instruction::WriteOutput { modes } => { self.exec_write(modes); }
            Instruction::JumpIfTrue { modes } => { self.exec_jump_if_true(modes); }
            Instruction::JumpIfFalse { modes } => { self.exec_jump_if_false(modes); }
            Instruction::LessThan { modes } => { self.exec_less_than(modes); }
            Instruction::Equals { modes } => { self.exec_equals(modes); }
            Instruction::End => { self.state = ComputerState::Halted; }
        }
        (self.state, last_ptr)
    }

    pub fn run(&mut self) {
        let mut counter = 0;
        loop {
            counter += 1;
            let (out, ptr_addr) = self.execute();
            match out {
                ComputerState::Halted => return,
                ComputerState::ReadyForInstruction => (),
                ComputerState::Panic => panic!(
                    "encountered unknown opcode. Please inspect memory at {}",
                    ptr_addr
                ),
                ComputerState::WaitingForInput => {
                    debug!("int code computer halted, waiting on input");
                    return;
                }
            }
            if counter >= 10000 {
                panic!("program has run more than 10000 operations. Probably stuck in a loop.")
            }
        }
    }

    pub fn is_halted(&self) -> bool {
        matches!(self.state, ComputerState::Halted)
    }

    pub fn is_waiting_for_input(&self) -> bool {
        matches!(self.state, ComputerState::WaitingForInput)
    }

    /// Returns a copy of memory. Note that this only represents a current snapshot; it will not be
    /// updated.
    pub fn dump_memory(&self) -> Memory {
        Memory {
            memory: self.memory.memory.clone(),
        }
    }

    fn parse_unary_op(&self, mode: &ParamMode) -> i64 {
        self.memory.read_mode(self.ptr + 1, self.rel_pos, mode)
    }

    fn parse_binary_op(&self, modes: BinaryModes) -> (i64, i64) {
        let a = self.memory.read_mode(self.ptr + 1, self.rel_pos, &modes[0]);
        let b = self.memory.read_mode(self.ptr + 2, self.rel_pos, &modes[1]);
        (a, b)
    }

    fn parse_trinary_op(&self, modes: TrinaryModes) -> (i64, i64, u64) {
        let a = self.memory.read_mode(self.ptr + 1, self.rel_pos, &modes[0]);
        let b = self.memory.read_mode(self.ptr + 2, self.rel_pos, &modes[1]);
        // The last param is never supposed to be interpreted as a pointer, it should be read
        // as an immediate. However, according to docs, the last one is never an immediate, it's always
        // a postitional. It seems like there are two types: ints and pointers. The first two arguments
        // are ints. If in position mode, evaluate the POINTERS and you get the values that you must add
        // together. If in immediate mode, just read the value. However, the last argument should just
        // be read at face value -- when you read 0, it actually means "pointer to position 0". You
        // can't write to "0", since that's a value, but you can write to "pointer to position 0". In
        // this way, it technically is never in IMMEDIATE mode.
        // The signature is something like this:
        //   ADD(int, int, ptr)
        // So when you read 01002, 0, 0, 0, you should read it as
        // ADD(val_at(0), 0, ptr_to(0)), where val_at -> int and ptr_to -> ptr type.
        // I'll see how the int code evolves, but I will probably have to change the type system
        // to accomodate this apparently contradictory statement about the write arg never being
        // in immediate, even though it clearly is here. >:(
        let addr = match modes[2] {
            ParamMode::RelativeMode => {
                self.memory.read_mode(self.ptr + 3, self.rel_pos, &ParamMode::RelativeMode)
            }
            _ => {
                self.memory.read_mode(self.ptr + 3, self.rel_pos, &ParamMode::Immediate)
            }
        };
        if addr < 0 {
            panic!("cannot use negative value {} as address", addr)
        }
        (a, b, addr as u64)
    }

    /// Computation Operation are simple and just perform arithmetic operations and write to the
    /// specified location. Any kind of work to determine if an operand is read from address or
    /// pointer should be done before calling the function.
    fn add(&mut self, a: i64, b: i64, addr: u64) {
        self.memory.write(addr, a + b)
    }

    fn mult(&mut self, a: i64, b: i64, addr: u64) {
        self.memory.write(addr, a * b)
    }

    pub fn attach_input(&mut self, input: DataStream) {
        self.input = input
    }

    pub fn dump_output(&self) -> DataStream {
        let mut out = self.output.copy();
        // Reset the consumer ind to allow the caller to fully read out the output
        // TODO: Maybe just return the memory here?
        out.consumer_ind = 0;
        out
    }

    pub fn clear_output(&mut self) {
        self.output.reset()
    }
}

#[cfg(test)]
mod tests {
    use crate::int_code::{DataStream, DsRead, IntCodeComputer, Memory, pad_memory};

    struct IntCodeDump {
        memory: Vec<i64>,
        output: Vec<i64>,
    }

    fn run_int_code_with_input(memory: Vec<i64>, input: i64) -> IntCodeDump {
        let mut computer = IntCodeComputer::new(memory.to_owned());
        computer.input.write(input);
        computer.run();
        IntCodeDump {
            output: computer.output.read_all(),
            memory: computer.dump_memory().memory,
        }
    }

    fn run_int_code_computer(input: Vec<i64>) -> IntCodeDump {
        let mut computer = IntCodeComputer::new(input);
        computer.run();
        IntCodeDump {
            output: computer.output.read_all(),
            memory: computer.dump_memory().memory,
        }
    }

    fn assert_int_code_computer_memory_matches(actual: IntCodeDump, expected: Vec<i64>) {
        // An intcode computer pads its memory with extra space for computation. When comparing
        // outputs, we also pad the expected memory so we match. The amount of padding is hidden
        // from the caller.
        assert_eq!(actual.memory, pad_memory(expected))
    }

    fn assert_int_code_computer_output_matches(actual: IntCodeDump, expected: Vec<i64>) {
        assert_eq!(actual.output, expected)
    }

    #[test]
    fn test_int_code_computer_basic_instructions() {
        assert_int_code_computer_memory_matches(
            run_int_code_computer(vec![1, 0, 0, 0, 99]),
            vec![2, 0, 0, 0, 99],
        );
        assert_int_code_computer_memory_matches(
            run_int_code_computer(vec![2, 3, 0, 3, 99]),
            vec![2, 3, 0, 6, 99],
        );
        assert_int_code_computer_memory_matches(
            run_int_code_computer(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801],
        );
        assert_int_code_computer_memory_matches(
            run_int_code_computer(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }

    #[test]
    fn test_int_code_with_params() {
        assert_int_code_computer_memory_matches(
            run_int_code_computer(vec![1002, 4, 3, 4, 33]),
            vec![1002, 4, 3, 4, 99],
        );
    }

    #[test]
    fn test_input_eq_8_pos_mode() {
        // This program tests whether or not the provided input is equal to 8.
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let actual = run_int_code_with_input(program.to_owned(), 8);
        assert_int_code_computer_output_matches(actual, vec![1]);

        let actual = run_int_code_with_input(program.to_owned(), 7);
        assert_int_code_computer_output_matches(actual, vec![0]);
    }

    #[test]
    fn test_input_le_8_pos_mode() {
        // This program tests whether or not the provided input is equal to 8.
        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let actual = run_int_code_with_input(program.to_owned(), 8);
        assert_int_code_computer_output_matches(actual, vec![0]);

        let actual = run_int_code_with_input(program.to_owned(), 7);
        assert_int_code_computer_output_matches(actual, vec![1]);
    }

    #[test]
    fn test_input_eq_8_imm_mode() {
        // This program tests whether or not the provided input is equal to 8.
        let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let actual = run_int_code_with_input(program.to_owned(), 8);
        assert_int_code_computer_output_matches(actual, vec![1]);

        let actual = run_int_code_with_input(program.to_owned(), 7);
        assert_int_code_computer_output_matches(actual, vec![0]);
    }

    #[test]
    fn test_input_le_8_imm_mode() {
        // This program tests whether or not the provided input is equal to 8.
        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        let actual = run_int_code_with_input(program.to_owned(), 8);
        assert_int_code_computer_output_matches(actual, vec![0]);

        let actual = run_int_code_with_input(program.to_owned(), 7);
        assert_int_code_computer_output_matches(actual, vec![1]);
    }

    #[test]
    fn test_jump() {
        // This program tests whether the provided input was non-zero.
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let actual = run_int_code_with_input(program.to_owned(), 0);
        assert_int_code_computer_output_matches(actual, vec![0]);

        let actual = run_int_code_with_input(program.to_owned(), 1);
        assert_int_code_computer_output_matches(actual, vec![1]);
    }

    #[test]
    fn test_is_halted() {
        let mut computer = IntCodeComputer::new(vec![3, 0, 99]);
        computer.run();
        assert!(computer.is_waiting_for_input());
        computer.run();
        assert!(computer.is_waiting_for_input());
        computer.input.write(1);
        computer.run();
        assert!(computer.is_halted());
        assert_eq!(computer.dump_memory().memory, pad_memory(vec![1, 0, 99]));
    }

    #[test]
    fn test_large_input_example() {
        // This program returns 999 if the input is less than 8, 1000 if the input is 8, and 1001
        // if the input is greater than 8.
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let actual = run_int_code_with_input(program.clone(), 7);
        assert_int_code_computer_output_matches(actual, vec![999]);

        let actual = run_int_code_with_input(program.clone(), 8);
        assert_int_code_computer_output_matches(actual, vec![1000]);

        let actual = run_int_code_with_input(program.clone(), 9);
        assert_int_code_computer_output_matches(actual, vec![1001]);
    }

    #[test]
    fn test_adj_rel_pos() {
        // Quine Program is a program that takes in no input and prints out itself.
        let quine_program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        assert_int_code_computer_output_matches(
            run_int_code_computer(quine_program.clone()),
            quine_program.clone(),
        );
        let output_16_digit = vec![
            1102, 34915192, 34915192, 7, 4, 7, 99, 0,
        ];
        assert_int_code_computer_output_matches(
            run_int_code_computer(output_16_digit.clone()),
            vec![1219070632396864],
        );
        let output_16_digit = vec![
            104, 1125899906842624, 99,
        ];
        assert_int_code_computer_output_matches(
            run_int_code_computer(output_16_digit.clone()),
            vec![1125899906842624],
        );
    }
}
