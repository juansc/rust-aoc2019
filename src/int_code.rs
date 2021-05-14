// IntCodeComputer Section
enum DsRead {
    // Signals that indicate a read could not succeed. Maybe make these errors?
    Closed,
    NoData,
    // The actual value
    Data(i32),
}

/// DataStream is represents a shared buffer that a producer and consumer can write and read to.
pub struct DataStream {
    buffer: Memory,
    is_closed: bool,
    producer_ind: u32,
    consumer_ind: u32,
}

impl DataStream {
    pub fn new() -> Self {
        Self {
            buffer: Memory {
                memory: vec![0i32; 1000],
            },
            is_closed: false,
            producer_ind: 0,
            consumer_ind: 0,
        }
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

    pub fn write(&mut self, val: i32) {
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

    pub fn read_all(&mut self) -> Vec<i32> {
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

    fn close(&mut self) {
        self.is_closed = true;
    }
}

/// Memory manages the memory of the IntCodeComputer. It can read from address, or it can read from
/// pointer. It can also write to address and write to pointer
pub struct Memory {
    memory: Vec<i32>,
}

impl Memory {
    /*
    fn new() -> Self {
        Memory { memory: Vec::new() }
    }
    */

    /// Returns the value at the specified address
    pub fn read(&self, addr: u32) -> i32 {
        self.memory[addr as usize]
    }

    fn read_mode(&self, val: u32, m: &ParamMode) -> i32 {
        match m {
            ParamMode::Position => self.read_ptr(val),
            ParamMode::Immediate => self.read(val),
        }
    }

    /// Returns the value at the pointer. It will read out the value at the given address, then use
    /// that value itself as an address and return what that points to.
    fn read_ptr(&self, ptr: u32) -> i32 {
        let addr = self.read(ptr);
        if addr < 0 {
            panic!("cannot use negative value {} as pointer", addr)
        }
        self.read(addr as u32)
    }

    /// Writes the specified value at the specified address.
    fn write(&mut self, addr: u32, val: i32) {
        self.memory[addr as usize] = val;
    }

    /// Writes the specified value at the address specified by the ptr.
    fn write_ptr(&mut self, ptr: u32, val: i32) {
        let addr = self.read(ptr);
        if addr < 0 {
            panic!("cannot use negative value {} as pointer", addr)
        }
        self.write(addr as u32, val);
    }
}

/// IntCodeComputer is initialized with memory and executes instructions until it encounters the
/// end of program code. It does not validate the code.
pub struct IntCodeComputer {
    ptr: u32,
    memory: Memory,
    pub input: DataStream,
    output: DataStream,
}

enum Instruction {
    Add { modes: [ParamMode; 3] },
    Mult { modes: [ParamMode; 3] },
    Read { modes: [ParamMode; 1] },
    Write { modes: [ParamMode; 1] },
    JumpIfTrue { modes: [ParamMode; 2] },
    JumpIfFalse { modes: [ParamMode; 2] },
    LessThan { modes: [ParamMode; 3] },
    Equals { modes: [ParamMode; 3] },
    End,
}

enum ParamMode {
    Position,
    Immediate,
}

impl ParamMode {
    fn parse(v: i32) -> Self {
        match v {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            _ => {
                panic!("unexpected val for param mode {}", v)
            }
        }
    }
}

fn parse_instruction(val: i32) -> Instruction {
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
        3 => Instruction::Read {
            modes: [ParamMode::parse(val / 100 % 10)],
        },
        4 => Instruction::Write {
            modes: [ParamMode::parse(val / 100 % 10)],
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
        99 => Instruction::End,
        _ => panic!("unexpected val for opcode {}", op_code),
    }
}

impl IntCodeComputer {
    /// Returns an IntCodeComputer initialized with the given memory.
    pub fn new(memory: Vec<i32>) -> Self {
        Self {
            ptr: 0,
            memory: Memory { memory },
            input: DataStream::new(),
            output: DataStream::new(),
        }
    }

    /// execute evaluates a single instruction. It returns a code indicating whether the execution
    /// was successful.
    fn execute(&mut self) -> (i8, u32) {
        let last_ptr = self.ptr;
        let instruction = parse_instruction(self.memory.read(self.ptr));
        match instruction {
            // Add
            Instruction::Add { modes } => {
                let (a, b, addr) = self.parse_trinary_op(modes);
                println!("At add {} {} {}", a, b, addr);
                self.add(a, b, addr);
                self.ptr += 4;
                (0, last_ptr)
            }
            // Mult
            Instruction::Mult { modes } => {
                let (a, b, addr) = self.parse_trinary_op(modes);
                println!("At mult {} {} {}", a, b, addr);
                self.mult(a, b, addr);
                self.ptr += 4;
                (0, last_ptr)
            }
            // Save
            Instruction::Read { modes: _ } => match self.input.read() {
                DsRead::Closed => {
                    panic!("Reading from a closed data stream")
                }
                DsRead::NoData => {
                    panic!("Attempting to read from data stream with no more data")
                }
                DsRead::Data(d) => {
                    let addr = self.memory.read(self.ptr + 1);
                    println!("At read addr={} d={}", addr, d);
                    self.memory.write(addr as u32, d);
                    self.ptr += 2;
                    (0, last_ptr)
                }
            },
            // Output
            Instruction::Write { modes } => {
                let val = self.parse_unary_op(&modes[0]);
                println!("At write {}", val);
                self.output.write(val);
                self.ptr += 2;
                (0, last_ptr)
            }
            Instruction::JumpIfTrue { modes } => {
                let (expr, addr) = self.parse_binary_op(modes);
                println!("At jit {} {}", expr, addr);
                if expr == 1 {
                    self.ptr = addr as u32
                } else {
                    self.ptr += 3;
                }
                (0, last_ptr)
            }
            Instruction::JumpIfFalse { modes } => {
                let (expr, addr) = self.parse_binary_op(modes);
                println!("At jif {} {}", expr, addr);
                if expr == 0 {
                    self.ptr = addr as u32
                } else {
                    self.ptr += 3;
                }
                (0, last_ptr)
            }
            Instruction::LessThan { modes } => {
                let (a, b, addr) = self.parse_trinary_op(modes);
                println!("At less than {} {} {}", a, b, addr);
                let val = if a < b { 1 } else { 0 };
                self.memory.write(addr, val);
                self.ptr += 4;
                (0, last_ptr)
            }
            Instruction::Equals { modes } => {
                let (a, b, addr) = self.parse_trinary_op(modes);
                println!("At equals {} {} {}", a, b, addr);
                let val = if a == b { 1 } else { 0 };
                self.memory.write(addr, val);
                self.ptr += 4;
                (0, last_ptr)
            }
            Instruction::End => (1, last_ptr),
        }
    }

    pub fn run(&mut self) {
        let mut counter = 0;
        loop {
            counter += 1;
            let (out, ptr_addr) = self.execute();
            match out {
                1 => return,
                -1 => panic!(
                    "encountered unknown opcode. Please inspect memory at {}",
                    ptr_addr
                ),
                _ => (),
            }
            if counter >= 10000 {
                panic!("program has run more than 10000 operations. Probably stuck in a loop.")
            }
        }
    }

    /// Returns a copy of memory. Note that this only represents a current snapshot; it will not be
    /// updated.
    pub fn dump_memory(&self) -> Memory {
        Memory {
            memory: self.memory.memory.clone(),
        }
    }

    fn parse_unary_op(&self, mode: &ParamMode) -> i32 {
        self.memory.read_mode(self.ptr + 1, mode)
    }

    fn parse_binary_op(&self, modes: [ParamMode; 2]) -> (i32, i32) {
        let a = self.memory.read_mode(self.ptr + 1, &modes[0]);
        let b = self.memory.read_mode(self.ptr + 2, &modes[1]);
        (a, b)
    }

    fn parse_trinary_op(&self, modes: [ParamMode; 3]) -> (i32, i32, u32) {
        let a = self.memory.read_mode(self.ptr + 1, &modes[0]);
        let b = self.memory.read_mode(self.ptr + 2, &modes[1]);
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
        let addr = self.memory.read_mode(self.ptr + 3, &ParamMode::Immediate);
        if addr < 0 {
            panic!("cannot use negative value {} as address", addr)
        }
        (a, b, addr as u32)
    }

    /// Computation Operation are simple and just perform arithmetic operations and write to the
    /// specified location. Any kind of work to determine if an operand is read from address or
    /// pointer should be done before calling the function.
    fn add(&mut self, a: i32, b: i32, addr: u32) {
        self.memory.write(addr, a + b)
    }

    fn mult(&mut self, a: i32, b: i32, addr: u32) {
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
}

#[cfg(test)]
mod tests {
    use crate::int_code::{DataStream, DsRead, IntCodeComputer, Memory};

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

    #[test]
    fn test_write_ptr() {
        let mut m = Memory { memory: vec![1, 2] };
        assert_eq!(m.memory, vec![1, 2]);
        m.write_ptr(0, 0);
        assert_eq!(m.memory, vec![1, 0]);
    }

    #[test]
    fn test_int_code_computer_basic_instructions() {
        assert_eq!(
            run_inc_code_computer(vec![1, 0, 0, 0, 99]),
            vec![2, 0, 0, 0, 99],
        );
        assert_eq!(
            run_inc_code_computer(vec![2, 3, 0, 3, 99]),
            vec![2, 3, 0, 6, 99],
        );
        assert_eq!(
            run_inc_code_computer(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801],
        );
        assert_eq!(
            run_inc_code_computer(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }

    #[test]
    fn test_int_code_with_params() {
        assert_eq!(
            run_inc_code_computer(vec![1002, 4, 3, 4, 33]),
            vec![1002, 4, 3, 4, 99],
        );
    }

    #[test]
    fn test_comparison_programs() {
        // This program tests whether or not the provided input is equal to 8.
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let actual = run_int_code_with_input(program.to_owned(), 8);
        assert_eq!(actual, vec![1]);

        let actual = run_int_code_with_input(program.to_owned(), 7);
        assert_eq!(actual, vec![0]);
    }

    #[test]
    fn test_jump() {
        // This program tests whether the provided input was non-zero.
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let actual = run_int_code_with_input(program.to_owned(), 0);
        assert_eq!(actual, vec![0]);

        let actual = run_int_code_with_input(program.to_owned(), 1);
        assert_eq!(actual, vec![1]);
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
        let actual = run_int_code_with_input(program.to_owned(), 7);
        assert_eq!(actual, vec![999]);

        let actual = run_int_code_with_input(program.to_owned(), 8);
        assert_eq!(actual, vec![1000]);

        let actual = run_int_code_with_input(program.to_owned(), 9);
        assert_eq!(actual, vec![1001]);
    }

    fn run_int_code_with_input(memory: Vec<i32>, input: i32) -> Vec<i32> {
        let mut computer = IntCodeComputer::new(memory.to_owned());
        computer.input.write(input);
        computer.run();
        computer.output.read_all()
    }

    fn run_inc_code_computer(input: Vec<i32>) -> Vec<i32> {
        let mut computer = IntCodeComputer::new(input);
        computer.run();
        computer.dump_memory().memory
    }
}
