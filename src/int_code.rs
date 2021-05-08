// IntCodeComputer Section
enum DSRead {
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
    fn new() -> Self {
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

    fn write(&mut self, val: i32) {
        self.buffer.write(self.producer_ind, val);
        self.producer_ind += 1;
    }

    fn read(&mut self) -> DSRead {
        if self.is_closed {
            return DSRead::Closed;
        }
        // Consumer is all caught up to the producer, so there is currently no data
        if self.consumer_ind == self.producer_ind {
            return DSRead::NoData;
        }
        let out = self.buffer.read(self.consumer_ind);
        self.consumer_ind += 1;
        DSRead::Data(out)
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
    fn new() -> Self {
        Memory { memory: Vec::new() }
    }
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
    input: DataStream,
    output: DataStream,
}

struct Instruction {
    op: Op,
    modes: Vec<ParamMode>,
}

enum Op {
    Add,
    Mult,
    Read,
    Write,
    End,
}

enum InstructionV2 {
    Add { modes: [ParamMode; 3] },
    Mult { modes: [ParamMode; 3] },
    Read { modes: [ParamMode; 1] },
    Write { modes: [ParamMode; 1] },
    End,
}

impl Op {
    fn parse(v: i32) -> Self {
        match v {
            1 => Op::Add,
            2 => Op::Mult,
            3 => Op::Read,
            4 => Op::Write,
            99 => Op::End,
            _ => {
                panic!("unexpected val for op code {}", v)
            }
        }
    }
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

fn parse_instruction(val: i32) -> InstructionV2 {
    let op_code = Op::parse(val % 100);
    match op_code {
        Op::Add => InstructionV2::Add {
            modes: [
                ParamMode::parse((val / 100) % 10),
                ParamMode::parse((val / 1000) % 10),
                ParamMode::parse((val / 10000) % 10),
            ],
        },
        Op::Mult => InstructionV2::Mult {
            modes: [
                ParamMode::parse(val / 100 % 10),
                ParamMode::parse(val / 1000 % 10),
                ParamMode::parse(val / 10000 % 10),
            ],
        },
        Op::Read => InstructionV2::Read {
            modes: [ParamMode::parse(val / 100 % 10)],
        },
        Op::Write => InstructionV2::Write {
            modes: [ParamMode::parse(val / 100 % 10)],
        },
        Op::End => InstructionV2::End,
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
            InstructionV2::Add { modes } => {
                let (a, b, addr) = self.parse_trinary_op(modes);
                println!("{} {} {} ", a, b, addr);
                self.add(a, b, addr);
                self.ptr += 4;
                (0, last_ptr)
            }
            // Mult
            InstructionV2::Mult { modes } => {
                let (a, b, addr) = self.parse_trinary_op(modes);
                self.mult(a, b, addr);
                self.ptr += 4;
                (0, last_ptr)
            }
            // Save
            InstructionV2::Read { modes } => match self.input.read() {
                DSRead::Closed => {
                    panic!("Reading from a closed data stream")
                }
                DSRead::NoData => {
                    panic!("Attempting to read from data stream with no more data")
                }
                DSRead::Data(d) => {
                    let addr = self.parse_unary_op();
                    self.memory.write(addr as u32, d);
                    self.ptr += 2;
                    (0, last_ptr)
                }
            },
            // Output
            InstructionV2::Write { modes } => {
                let val = self.memory.read_ptr(self.ptr + 1);
                self.output.write(val);
                self.ptr += 2;
                (0, last_ptr)
            }
            InstructionV2::End => (1, last_ptr),
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

    fn parse_unary_op(&self) -> i32 {
        self.memory.read_ptr(self.ptr + 1)
    }

    fn parse_trinary_op(&self, modes: [ParamMode; 3]) -> (i32, i32, u32) {
        let a = self.memory.read_mode(self.ptr + 1, &modes[0]);
        let b = self.memory.read_mode(self.ptr + 2, &modes[1]);
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

    fn attach_input(&mut self, input: DataStream) {
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
    use crate::int_code::{DSRead, DataStream, IntCodeComputer, Memory};

    #[test]
    fn test_basic_ds() {
        let mut ds = DataStream::new();
        // No data at first...
        assert!(matches!(ds.read(), DSRead::NoData));
        // We can read some after we've added data, but only once
        ds.write(5);
        assert!(matches!(ds.read(), DSRead::Data(5)));
        assert!(matches!(ds.read(), DSRead::NoData));

        // We can keep reading up to where it wrote
        ds.write(10);
        ds.write(20);
        assert!(matches!(ds.read(), DSRead::Data(10)));
        assert!(matches!(ds.read(), DSRead::Data(20)));
        assert!(matches!(ds.read(), DSRead::NoData));

        ds.close();
        assert!(matches!(ds.read(), DSRead::Closed));
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

    fn run_inc_code_computer(input: Vec<i32>) -> Vec<i32> {
        let mut computer = IntCodeComputer::new(input);
        computer.run();
        computer.dump_memory().memory
    }
}
