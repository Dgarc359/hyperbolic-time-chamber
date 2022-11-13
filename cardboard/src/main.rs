/// The actual bytes of our entire ROM image.
const ROM: &[u8] = include_bytes!("rom.bin");
/// The magic sequence to output to an ANSI-compliant terminal that will make
/// it clear itself.
const CLEAR_SCREEN: &str = "\x1B[1;1H\x1B[J";
fn clear_screen() {
    if cfg!(debug_assertions) {
        // Instead of clearing the screen, let's just output some newlines.
        // This way we won't wipe out a panic message sometimes.
        print!("\n\n");
    }
    else {
        print!("{}", CLEAR_SCREEN);
    }
}

struct Memory {
    ram: [u8; 16384],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            ram: [0xFF; 16384],
        }
    }
    pub fn fetch_byte(&self, address: u16) -> u8 {
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
    pub fn store_byte(&mut self, address: u16, data: u8) {
        if address >= 0x8000 {
            panic!("you can't store in ROM!")
        }
        else if address >= 0x4000 {
            if data >= 0x80 {
                // High bit set -> clear the screen
                clear_screen();
            }
            else if data == 0x00 {
                // No bits set -> clear the keyboard buffer
                // ... but we don't have a keyboard buffer O_O
            }
            else {
                // Any other value -> write that character to the terminal
                print!("{}", data as char);
            }
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
    /// Status is a set of bits: NV-BDIZC
    status: u8,
    stack_pointer: u8,
    program_counter: u16,
}

/// Negative. If 1, the last value had a negative result. If 0, it had a non-
/// negative (positive or zero) result.
#[allow(unused)] const STATUS_N: u8 = 1 << 7;
/// Overflow. If 1, the result had a signed overflow. I spent almost a month
/// of my fucking life trying to understand this flag and I never did, so I'm
/// not going to make you implement it, goddammit.
#[allow(unused)] const STATUS_V: u8 = 1 << 6;
// 0x20 is not wired to anything in the real processor o_O
/// Break. If 1, the interrupt was a BRK pretending to be an interrupt.
#[allow(unused)] const STATUS_B: u8 = 1 << 4;
/// Decimal mode. If 1, we're a decimal processor, not a binary one! Fuck that
#[allow(unused)] const STATUS_D: u8 = 1 << 3;
/// Interrupt mask. If 1, interrupts are disabled. If 0, they're enabled.
#[allow(unused)] const STATUS_I: u8 = 1 << 2;
/// Zero flag. If 1, the last operation resulted in a zero. If 0, it resulted
/// in something non-zero.
#[allow(unused)] const STATUS_Z: u8 = 1 << 1;
/// Carry. If 1, the result carried, i.e. it didn't fit into a byte. If 0, the
/// result didn't carry.
#[allow(unused)] const STATUS_C: u8 = 1 << 0;

// 0x12 0x34 0x56 0x78
// 0x12345678 - big endian, the "biggest" byte comes first
// 0x78563412 - little endian, the "littlest" byte comes first

impl Cpu {
    pub fn new(mem: &Memory) -> Self {
        Cpu {
            accumulator: 0,
            index_x: 0,
            index_y: 0,
            status: 0,
            stack_pointer: 0,
            program_counter: u16::from_le_bytes([
                mem.fetch_byte(0xFFFC), // looks at these mem addrs to know where to start
                mem.fetch_byte(0xFFFD),
            ])
        }
    }
    pub fn fetch_pc_postincrement(&mut self, mem: &mut Memory) -> u8 {
        let mem = mem.fetch_byte(self.program_counter);
        self.program_counter += 1;
        mem
    }
    pub fn push(&mut self, mem: &mut Memory, value: u8) {
        // the stack exists at address 0x01XX
        let address = u16::from_le_bytes([self.stack_pointer, 0x01]);
        mem.store_byte(address, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }
    pub fn pull(&mut self, mem: &mut Memory) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let addr = u16::from_le_bytes([self.stack_pointer, 0x01]);
        mem.fetch_byte(addr)
    }
    /// Set the Z and N status flags correctly for the given value, and return
    /// the passed-in value.
    pub fn status_nz(&mut self, value: u8) -> u8 {
        // -- Setting the Z flag --
        if value == 0 {
            // set Z to one here...
            self.status = self.status | STATUS_Z;
        }
        else {
            // set Z to zero here...
            self.status = self.status & !STATUS_Z; // ! bitwise complement in rust
        }
        // -- Setting the N flag --
        if (value as i8) < 0 {
            // set N to one here...
            self.status = self.status | STATUS_N;
        } else {
            // set N to zero here...
            self.status = self.status & !STATUS_N;
        }
        value
    }
    /// Set the N, V, Z, and C status flags correctly for the given value, and
    /// return the passed-in value as a byte.
    pub fn status_nvzc(&mut self, value: u16) -> u8 {
        // -- Setting the V flag --
        // FUCK THE V FLAG
        // -- Setting the C flag --

        self.status_nz(value as u8)
    }
    /// Absolute addressing mode: Fetch two additional bytes, use them as the
    /// final address.
    pub fn fetch_absolute_address(&mut self, mem: &mut Memory) -> u16 {
        let lo = self.fetch_pc_postincrement(mem);
        let hi = self.fetch_pc_postincrement(mem);
        u16::from_le_bytes([lo, hi])
    }
    /// Zero Page addressing mode: Fetch one additional byte, fill in the high
    /// byte of the address with zeroes.
    pub fn fetch_zero_page_address(&mut self, mem: &mut Memory) -> u16 {
        let lo = self.fetch_pc_postincrement(mem);
        u16::from_le_bytes([lo, 0])
    }
    /// Zero Page Indirect Y-Indexed addressing mode: Fetch one additional
    /// byte. This byte is the address of the REAL address. Except NOT QUITE,
    /// because we're going to ADD the value of Y to that address to get the
    /// REAL CORRECT address!
    pub fn fetch_zero_page_indirect_y_index_address(&mut self, mem: &mut Memory) -> u16 {
        // my_pointer[Y]
        let addr = self.fetch_zero_page_address(mem);
        let lo = mem.fetch_byte(addr);
        let hi = mem.fetch_byte(addr+1);
        let addr = u16::from_le_bytes([lo, hi]);
        addr.wrapping_add(self.index_y as u16)
    }
    /// Branch target: Fetch one byte. Sign extend it. Add that to the program
    /// counter and that's our branch target.
    pub fn fetch_branch_target(&mut self, mem: &mut Memory) -> u16 {
        let byte = self.fetch_pc_postincrement(mem);
        let offset = (byte as i8) as u16;
        self.program_counter.wrapping_add(offset)
    }
    /// Is Equal!?
    pub fn is_equal(&self) -> bool {
        (self.status & STATUS_Z) != 0
    }
    pub fn run_one_instruction(&mut self, mem: &mut Memory) {
        // fetch @ program counter
        let opcode = self.fetch_pc_postincrement(mem);
        // decode and exec
        match opcode {
            0x09 => {
                // ORA #imm
                // bitwise OR Accumulator (IMMediate)
                let value = self.fetch_pc_postincrement(mem);
                self.accumulator = self.status_nz(self.accumulator | value);
            },
            0x20 => {
                // JSR abs
                // Jump to SubRoutine (ABSolute address)
                let address = self.fetch_absolute_address(mem);
                let pc_bytes = self.program_counter.to_le_bytes();
                self.push(mem, pc_bytes[1]);
                self.push(mem, pc_bytes[0]);
                self.program_counter = address;
            },
            0x48 => {
                // PHA
                // PusH Accumulator (onto the stack)
                self.push(mem, self.accumulator);
            },
            0x5A => {
                // PHY
                // PusH Y register (onto the stack)
                self.push(mem, self.index_y);
            },
            0x60 => {
                // RTS
                // ReTurn from Subroutine
                let lo = self.pull(mem);
                let hi = self.pull(mem);
                self.program_counter = u16::from_le_bytes([lo, hi]);
            },
            0x68 => {
                // PLA
                // PuLl Accumulator (from the stack)
                self.accumulator= self.pull(mem);
            },
            0x7A => {
                // PLY
                // PuLl Y register (from the stack)
                self.index_y = self.pull(mem);
            },
            0x85 => {
                // STA zp
                // STore Accumulator (Zero Page address)
                let addr = self.fetch_zero_page_address(mem);
                mem.store_byte(addr, self.accumulator);
            },
            0x80 => {
                // BRA target
                // BRanch Always
                let target = self.fetch_branch_target(mem);
                self.program_counter = target;
            },
            0x8D => {
                // STA abs
                // STore Accumulator (ABSolute addr)
                let address = self.fetch_absolute_address(mem);
                mem.store_byte(address, self.accumulator);
            },
            0x8E => {
                // STX abs
                // STore X (ABSolute address)
                let address = self.fetch_absolute_address(mem);
                mem.store_byte(address, self.index_x);
            },
            0x9A => {
                // TXS
                // Transfer X register into Stack pointer
                self.stack_pointer = self.index_x;
            },
            0x9C => {
                // STZ abs
                // STore Zero (ABSolute address)
                let address = self.fetch_absolute_address(mem);
                mem.store_byte(address, 0);
            },
            0xA0 => {
                // LDY #imm
                // LoaD Y register (IMMediate)
                let val = self.fetch_pc_postincrement(mem);
                self.index_y = self.status_nz(val);
            },
            0xA2 => {
                // LDX #imm (operation, addressing mode)
                // LoaD X register (IMMediate)
                let value = self.fetch_pc_postincrement(mem);
                self.index_x = self.status_nz(value);
            },
            0xA5 => {
                // LDA zp
                // LoaD Accumulator (Zero Page)
                let address = self.fetch_zero_page_address(mem);
                self.accumulator = self.status_nz(mem.fetch_byte(address));
            },
            0xA9 => {
                // LDA #imm
                // LoaD Accumulator (IMMediate)
                let val = self.fetch_pc_postincrement(mem);
                self.accumulator = self.status_nz(val);
            },
            0xB1 => {
                // LDA (zp),Y
                // LoaD Accumulator (Zero Page indirect Y-indexed)
                let address = self.fetch_zero_page_indirect_y_index_address(mem);
                self.accumulator = self.status_nz(mem.fetch_byte(address));
            },
            0xC8 => {
                // INY
                // INcrement Y register
                self.index_y = self.status_nz(self.index_y.wrapping_add(1));
            },
            0xC9 => {
                // CMP #imm
                // CoMPare accumulator (immediate value)
                let a = self.accumulator as u16;
                let b = !self.fetch_pc_postincrement(mem) as u16;
                let c = 1;
                // because this is generally true in binary apparently
                // a + !b + 1 = a - b
                let result = a + b + c;
                // since this is a comparison, and not an actual add/sub, we
                // discard the result...
                self.status_nvzc(result);
                // ...but not before using it to set flags!
            },
            0xE8 => {
                // INX
                // INcrement X register
                // self.index_x += 1;
                self.index_x = self.status_nz(self.index_x.wrapping_add(1));
            },
            0xF0 => {
                // BEQ target
                // Branch if EQual
                let target = self.fetch_branch_target(mem);
                if self.is_equal() {
                    self.program_counter = target;
                }
            },
            x => panic!("Unknown opcode 0x{:02X}", x),
        }
    }
}

fn main() {
    let mut mem = Memory::new();
    let mut cpu = Cpu::new(&mem);
    loop {
        cpu.run_one_instruction(&mut mem);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test] #[should_panic]
    fn cannot_write_rom() {
        let mut mem = Memory::new();
        mem.store_byte(0x8000, 0x45);
    }
}