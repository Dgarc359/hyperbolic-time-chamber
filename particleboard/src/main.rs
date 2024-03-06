use std::io::Read;


struct Ram {
    bytes: Vec<u8>,
}

const INITIAL_RAM_IMAGE: &[u8] = include_bytes!("riscv-fight.bin");

impl Ram {
    fn new() -> Ram {
        let mut container = vec![0; 10_000_000];
        /* This code works: */
        for (i,b) in INITIAL_RAM_IMAGE.iter().enumerate() {
            container[i] = *b;
        }
        /* Does the same as: */
        // container[..INITIAL_RAM_IMAGE.len()].copy_from_slice(INITIAL_RAM_IMAGE);
        assert_eq!(container.len(), 10_000_000);
        Ram { bytes: container }
    }
    
    fn read_word(&mut self, address: u32) -> u32 {
        let byte_1 = self.bytes[address as usize];
        let byte_2 = self.bytes[(address.wrapping_add(1)) as usize];
        let byte_3 = self.bytes[(address.wrapping_add(2)) as usize];
        let byte_4 = self.bytes[(address.wrapping_add(3)) as usize];
        let word: u32 = u32::from_le_bytes([byte_1, byte_2, byte_3, byte_4]);
        word
    }

    fn read_byte(&mut self, address: u32) -> u8 {
       self.bytes[address as usize] 
    }
}

struct Computer {
    cpu: Cpu,
    ram: Ram,
}

struct Cpu {
    registers: [u32; 32],
    pc: u32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu { registers: [0; 32], pc: 0 }
    }

    fn set_register(&mut self, register_number: u32, value: u32) {
        match register_number {
            0 => {
                // throw the write into the bit bucket!
                // do NOTHING!!!
            },
            x => {
                self.registers[register_number as usize] = value;
            }
        }
    }

    fn step(&mut self, ram: &mut Ram) {
        println!("PC is: {:08X}", self.pc);
        let address_of_instruction = self.pc;
        let next_instruction = ram.read_word(address_of_instruction);
        // Now that we've read the PC, bump it forward.
        self.pc += 4;
        println!("Execute instruction: {next_instruction:08X}");
        let opcode = next_instruction & 0b11_111_11;
        println!("Opcode: {opcode:07b}");
        let rd = (next_instruction >> 7) & 0b11111;
        let rs1 = (next_instruction >> 15) & 0b11111;
        let rs2 = (next_instruction >> 20) & 0b11111;
        let funct3 = (next_instruction >> 12) & 0b111;

        match opcode {
            0b01_101_11 => {
                // LUI
                let imm = (next_instruction >> 12) << 12;
                self.set_register(rd, imm);
            },
            0b00_100_11 => {
                // OP-IMM
                let imm = (next_instruction >> 20);
                match funct3 {
                    000 => {
                        // ADDI
                        self.set_register(rd, self.registers[rs1 as usize].wrapping_add(imm));
                    },
                    // Notes that will live somewhere some day:
                    // SRA: Shift Right ARITHMETIC - Shift in copies of the sign bit
                    // SRL: Shift Right LOGICAL - Shift in zeroes
                    x => panic!("implement other things. {:03b}", x)
                }
            },
            0b_11_011_11 => {
                // JAALL
                let imm_20 = (next_instruction as i32 >> 31) as u32;
                let imm_1 = (next_instruction >> 21) & 0b11111_11111;
                let imm_11 = (next_instruction >> 20) & 0b1;
                let imm_12 = (next_instruction >> 12) & 0b1111_1111;
                let imm = (imm_20 << 20) | (imm_1 << 1) | (imm_11 << 11) | (imm_12 << 12);
                self.set_register(rd, self.pc);
                self.pc = address_of_instruction.wrapping_add(imm);
            },
            x => panic!("Unknown opcode: {x:07b}"),
        }
    }
}

impl Computer {
    fn new() -> Computer {
        Computer { cpu: Cpu::new(), ram: Ram::new() }
    }
    fn step(&mut self) {
        self.cpu.step(&mut self.ram)
    }
}

fn main() {
    let mut particle_board_pc = Computer::new();
    loop {
        particle_board_pc.step();
    }
}
