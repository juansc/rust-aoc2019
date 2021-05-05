// IntCodeComputer Section

/// Memory manages the memory of the IntCodeComputer. It can read from address, or it can read from
/// pointer. It can also write to address and write to pointer
pub struct Memory {
    memory: Vec<i32>,
}

impl Memory {
    /// Returns the value at the specified address
    pub fn read(&self, addr: u32) -> i32 {
        self.memory[addr as usize]
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
}

impl IntCodeComputer {
    /// Returns an IntCodeComputer initialized with the given memory.
    pub fn new(memory: Vec<i32>) -> IntCodeComputer {
        IntCodeComputer {
            ptr: 0,
            memory: Memory { memory },
        }
    }

    /// execute evaluates a single instruction. It returns a code indicating whether the execution
    /// was successful.
    fn execute(&mut self) -> (i8, u32) {
        let last_ptr = self.ptr;
        let val = self.memory.read(self.ptr);
        match val {
            1 => {
                let (a, b, addr) = self.parse_binary_op();
                self.add(a, b, addr);
                self.ptr += 4;
                (0, last_ptr)
            }
            2 => {
                let (a, b, addr) = self.parse_binary_op();
                self.mult(a, b, addr);
                self.ptr += 4;
                (0, last_ptr)
            }
            99 => (1, last_ptr),
            _ => (-1, last_ptr),
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

    fn parse_binary_op(&self) -> (i32, i32, u32) {
        let a = self.memory.read_ptr(self.ptr + 1);
        let b = self.memory.read_ptr(self.ptr + 2);
        let addr = self.memory.read(self.ptr + 3);
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
}

#[cfg(test)]
mod tests {
    use crate::util::{IntCodeComputer, Memory};

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
