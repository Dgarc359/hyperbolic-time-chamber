use std::io::Read;

mod ram {

    pub struct Ram {
        bytes: Vec<u8>,
    }

    pub const INITIAL_RAM_IMAGE: &[u8] = include_bytes!("riscv-fight.bin");

    // Our memory map so far:
    // When *writing*:
    // 0 .. 16777216: bytes of RAM
    // 0xFFFFFFFC: actually output to the terminal instead of doing something in RAM
    // all other values: PANIC
    // When *reading*:
    // 0 .. 16777216: bytes of RAM
    // 0xFFFFFFFC: actually input from the terminal instead of doing something in RAM
    // all other values: PANIC

    impl Ram {
        pub fn new() -> Ram {
            // 16 mb ram
            let mut container = vec![0; 16_777_216];
            /* This code works: */
            for (i, b) in INITIAL_RAM_IMAGE.iter().enumerate() {
                container[i] = *b;
            }
            /* Does the same as: */
            // container[..INITIAL_RAM_IMAGE.len()].copy_from_slice(INITIAL_RAM_IMAGE);
            assert_eq!(container.len(), 16_777_216);
            Ram { bytes: container }
        }

        pub fn read_word(&mut self, address: u32) -> u32 {
            if address == 0xFFFFFFFC {
                // magical.
                use std::io::Read;
                let mut buf = [0; 1];
                std::io::stdin().read_exact(&mut buf).unwrap();
                buf[0] as u32
            } else {
                let byte_1 = self.bytes[address as usize];
                let byte_2 = self.bytes[(address.wrapping_add(1)) as usize];
                let byte_3 = self.bytes[(address.wrapping_add(2)) as usize];
                let byte_4 = self.bytes[(address.wrapping_add(3)) as usize];
                let word: u32 = u32::from_le_bytes([byte_1, byte_2, byte_3, byte_4]);
                word
            }
        }

        pub fn write_byte(&mut self, address: u32, value: u8) {
            self.bytes[address as usize] = value;
        }
        pub fn read_byte(&mut self, address: u32) -> u8 {
            self.bytes[address as usize]
        }

        pub fn write_word(&mut self, address: u32, word_to_write: u32) {
            if address == 0xFFFFFFFC {
                // magical.
                use std::io::Write;
                std::io::stdout().write_all(&[word_to_write as u8]).unwrap();
            } else {
                let bytes_to_write = word_to_write.to_le_bytes();
                self.bytes[address as usize] = bytes_to_write[0];
                self.bytes[address as usize + 1] = bytes_to_write[1];
                self.bytes[address as usize + 2] = bytes_to_write[2];
                self.bytes[address as usize + 3] = bytes_to_write[3];
            }
        }
    }
}
use ram::*;

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
        Cpu {
            registers: [0; 32],
            pc: 0,
        }
    }

    fn get_register(&self, register_number: u32) -> u32 {
        self.registers[register_number as usize]
    }

    fn set_register(&mut self, register_number: u32, value: u32) {
        match register_number {
            0 => {
                // throw the write into the bit bucket!
                // do NOTHING!!!
            }
            x => {
                self.registers[register_number as usize] = value;
            }
        }
    }

    fn step(&mut self, ram: &mut Ram) {
        eprintln!("PC is: {:08X}", self.pc);
        let address_of_instruction = self.pc;
        let next_instruction = ram.read_word(address_of_instruction);
        // Now that we've read the PC, bump it forward.
        self.pc += 4;
        eprintln!("Execute instruction: {next_instruction:08X}");
        let opcode = next_instruction & 0b11_111_11;
        eprintln!("Opcode: {opcode:07b}");
        let rd = (next_instruction >> 7) & 0b11111;
        let rs1 = (next_instruction >> 15) & 0b11111;
        let rs2 = (next_instruction >> 20) & 0b11111;
        let funct3 = (next_instruction >> 12) & 0b111;

        match opcode {
            0b01_101_11 => {
                eprintln!("Executing LUI");
                // LUI
                let imm = (next_instruction >> 12) << 12;
                self.set_register(rd, imm);
                eprintln!("rd now says: {:08X}", self.get_register(rd));
            }
            0b00_100_11 => {
                // OP-IMM
                let imm = (next_instruction as i32 >> 20) as u32;
                match funct3 {
                    0b000 => {
                        // ADDI
                        self.set_register(rd, self.registers[rs1 as usize].wrapping_add(imm));
                    }
                    0b001 => todo!("SLLI"),
                    0b010 => todo!("SLTI"),
                    0b011 => todo!("SLTIU"),
                    0b100 => todo!("XORI"),
                    0b101 => todo!("SRLI/SRAI"),
                    0b110 => todo!("ORI"),
                    0b111 => {
                        self.set_register(rd, self.registers[rs1 as usize] & imm);
                    }
                    // Notes that will live somewhere some day:
                    // SRA: Shift Right ARITHMETIC - Shift in copies of the sign bit
                    // SRL: Shift Right LOGICAL - Shift in zeroes
                    _ => unreachable!(),
                }
            }
            0b_11_011_11 => {
                eprintln!("Executing JAL");
                // JAALL
                let imm_20 = (next_instruction as i32 >> 31) as u32;
                let imm_1 = (next_instruction >> 21) & 0b11111_11111;
                let imm_11 = (next_instruction >> 20) & 0b1;
                let imm_12 = (next_instruction >> 12) & 0b1111_1111;
                let imm = (imm_20 << 20) | (imm_1 << 1) | (imm_11 << 11) | (imm_12 << 12);
                self.set_register(rd, self.pc);
                self.pc = address_of_instruction.wrapping_add(imm);
            }
            // TODO: review, this may be incorrect
            0b_01_000_11 => {
                // STORE
                let imm_0 = (next_instruction >> 7) & 0b11111;
                let imm_5 = (next_instruction as i32 >> 25) as u32;
                let imm = (imm_0 << 0) | (imm_5 << 5);
                let address = self.get_register(rs1).wrapping_add(imm);
                match funct3 {
                    // SB
                    0b000 => {
                        ram.write_byte(address, self.get_register(rs2) as u8);
                    }
                    // SH
                    0b001 => todo!("SH"),
                    // SW
                    0b010 => {
                        eprintln!("Executing STORE SW");
                        // store 32-bit values from low bits of rs2 to mem
                        let value = self.get_register(rs2);
                        eprintln!("base: {}", self.get_register(rs1));
                        eprintln!("offset: {}", imm as i32);
                        ram.write_word(address, value);
                    }
                    x => panic!("implement other things for STORE. {:03b}", x),
                }
                eprintln!("finished STORE");
            }
            0b00_101_11 => {
                eprintln!("Executing Add Unsigned Immediate to PC (AUIPC)");
                // AUIPC
                let imm_12 = (next_instruction >> 12) << 12;
                self.set_register(rd, address_of_instruction.wrapping_add(imm_12));
            }
            0b11_001_11 => {
                eprintln!("Executing JALR");
                // JALR
                // add sign extended 12 bit immediate to register rs1
                let imm_0 = (next_instruction as i32 >> 20) as u32;
                // self.set_register(rs1, address_of_instruction.wrapping_add(imm_0));
                let addr = self.registers[rs1 as usize].wrapping_add(imm_0) & !1;
                // & 0b11111111_11111111_11111111_11111110;
                // write address of instruction after jump (pc +4) to register rd
                self.set_register(rd, self.pc);
                self.pc = addr;
            }
            0b00_000_11 => {
                eprintln!("Executing LOAD");
                // pull 8 bits from mem
                // sign extend to 32 bits

                let imm = (next_instruction as i32 >> 20) as u32;
                let addr = self.get_register(rs1).wrapping_add(imm);

                match funct3 {
                    0b000 => todo!("LB (load byte)"),
                    0b001 => todo!("LH (load half word)"),
                    0b010 => {
                        // LW (load whole word, signed)
                        self.set_register(rd, ram.read_word(addr));
                    }
                    0b011 => panic!("LD (load double word) but we're not 64-bit!"),
                    0b100 => {
                        // LBU (load byte, unsigned)
                        // read unsigned byte
                        // zero extend to 32 bits
                        self.set_register(rd, ram.read_byte(addr) as u32);
                    }
                    0b101 => todo!("LHU (load half unsigned)"),
                    0b110 => panic!("LWU (load word unsigned) but we're not 64-bit!"),
                    0b111 => todo!("LDU (load double word unsigned) but we're not 128-bit!"),
                    _ => unreachable!(),
                }
                // let mem = ram.bytes.pop().unwrap() as i32;
                // // store in rd
                // self.set_register(rd, mem as u32);
            }
            0b11_000_11 => {
                eprintln!("Executing BRANCH");
                let imm_12 = (next_instruction as i32 >> 31) as u32;
                let imm_5 = (next_instruction >> 25) & 0b111_111;
                let imm_1 = (next_instruction >> 8) & 0b1111;
                let imm_11 = (next_instruction >> 7) & 0b1;
                let imm = (imm_12 << 12) | (imm_5 << 5) | (imm_1 << 1) | (imm_11 << 11);
                let target_addr = address_of_instruction.wrapping_add(imm);
                let val_one = self.get_register(rs1);
                let val_two = self.get_register(rs2);
                let should_take_branch = match funct3 {
                    0b000 => val_one == val_two,                   // BEQ
                    0b001 => val_one != val_two,                   // BNE
                    0b100 => (val_one as i32) < (val_two as i32),  // BLT
                    0b101 => (val_one as i32) >= (val_two as i32), // BGE
                    0b110 => val_one < val_two,                    // BLTU
                    0b111 => val_one >= val_two,                   // BGEU
                    _ => unreachable!(),
                };
                if should_take_branch {
                    // What does take the branch mean?
                    self.pc = target_addr;
                }
            }
            x => panic!("Unknown opcode: {x:07b}"),
        }
    }
}

impl Computer {
    fn new() -> Computer {
        Computer {
            cpu: Cpu::new(),
            ram: Ram::new(),
        }
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
