const ROM: &[u8] = include_bytes!("rom.bin");

struct Memory {
    ram: [u8; 16384],
}

impl Memory {
    pub fn fetch_byte(address: u16) -> u8 {
        if address >= 0x8000 {
            return ROM[(address - 0x8000) as usize];
        }
        else if address >= 0x4000 {
            todo!("io lol")
        }
        else {
            return self.ram[address as usize];
        }
    }
    pub fn store_byte(address: u16, data: u8) {
        if address >= 0x8000 {
            panic!("you can't store in ROM!")
        }
        else if address >= 0x4000 {
            todo!("io lol")
        }
        else {
            self.ram[address as usize] = data
        }
    }
}

struct Cpu {
    accumulator: u8,
    index_x: u8,
    index_y: u8,
    status: u8,
    stack_pointer: u8,
    program_counter: u16,
}

// 0x12 0x34 0x56 0x78
// 0x12345678 - big endian, the "biggest" byte comes first
// 0x78563412 - little endian, the "littlest" byte comes first

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            accumulator: 0,
            index_x: 0,
            index_y: 0,
            status: 0,
            stack_pointer: 0,
            program_counter: u16::from_le_bytes([
                ?????.fetch_byte(0xFFFC), // looks at these mem addrs to know where to start
                ?????.fetch_byte(0xFFFD),
            ])
        }
    }
    
    pub fn run_one_instruction(&mut self, mem: &mut Memory) {
        // fetch @ program counter
        let opcode = ?????.fetch_byte(self.program_counter);
        // decode
        // exec
    }
}

fn main() {
    
}
