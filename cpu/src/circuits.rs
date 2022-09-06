use crate::{bits_to_u8, gates::*};
use std::collections::HashMap;

/*
* Logic circuits
*/

fn equals(a: Bit, b: Bit) -> Bit {
    or_gate(and_gate(not_gate(a), not_gate(b)), and_gate(a, b))
}

fn m_bit_equals(a: Vec<Bit>, b: Vec<Bit>) -> Bit {
    let matched_bits: Vec<(&Bit, &Bit)> = a.iter().zip(b.iter()).collect();
    for (&a_bit, &b_bit) in matched_bits {
        if !equals(a_bit, b_bit) {
            return false;
        }
    }
    true
}

/*
* Arithmetic circuits
*/

fn one_bit_adder(a: Bit, b: Bit, carry_in: Bit) -> (Bit, Bit) {
    let sum1 = xor_gate(a, b);
    let carry_over1 = and_gate(a, b);

    let sum2 = xor_gate(sum1, carry_in);
    let carry_over2 = and_gate(sum1, carry_in);

    let carry_out = or_gate(carry_over1, carry_over2);
    (sum2, carry_out)
}

fn m_bit_adder(bits: usize, a: Vec<Bit>, b: Vec<Bit>) -> (Vec<Bit>, Bit) {
    let mut sum = vec![false; bits];
    let mut carry_over = false;
    let matched_bits: Vec<(&Bit, &Bit)> = a.iter().zip(b.iter()).collect();

    for idx in (0..bits).rev() {
        let (&a_bit, &b_bit) = matched_bits[idx];
        let result = one_bit_adder(a_bit, b_bit, carry_over);
        sum[idx] = result.0;
        carry_over = result.1;
    }

    (sum, carry_over)
}

/*
* Control circuits
*/

// multiplexer on two bits
// selects one of the two bits based on select bit
fn two_way_mux(a: Bit, b: Bit, select: Bit) -> Bit {
    or_gate(and_gate(a, select), and_gate(b, not_gate(select)))
}

fn m_bit_two_way_mux(a: Vec<Bit>, b: Vec<Bit>, select: Bit) -> Vec<Bit> {
    let matched_bits: Vec<(&Bit, &Bit)> = a.iter().zip(b.iter()).collect();
    let mut bits = Vec::with_capacity(matched_bits.capacity());
    for (&a_bit, &b_bit) in matched_bits {
        bits.push(two_way_mux(a_bit, b_bit, select));
    }
    bits
}

const BITS: usize = 8;

// This BITS-bit ALU (arithmetic logic unit) has two operations:
//     EQUALS - opcode is 0 (false)
//     ADD - opcode is 1 (true)
// All operations' circuits are executed, but only one is returned based on opcode
pub fn alu(opcode: Bit, a: Vec<Bit>, b: Vec<Bit>) -> Vec<Bit> {
    // ADD
    let (sum, _) = m_bit_adder(BITS, a.clone(), b.clone());

    // EQUAL
    let mut equal = vec![false; BITS];
    equal[BITS - 1] = m_bit_equals(a, b);

    // choose which circuit value to return
    m_bit_two_way_mux(sum, equal, opcode)
}

#[derive(Default)]
pub struct RegisterFile {
    registers: HashMap<u8, Vec<Bit>>,
}

impl RegisterFile {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set(&mut self, register: u8, value: Vec<Bit>) {
        self.registers.insert(register, value);
    }

    fn read(&self, reg1: u8, reg2: u8) -> (Vec<Bit>, Vec<Bit>) {
        (
            self.registers
                .get(&reg1)
                .expect("Nothing found in register")
                .clone(),
            self.registers
                .get(&reg2)
                .expect("Nothing found in register")
                .clone(),
        )
    }
}

#[derive(Debug)]
pub struct RAM {
    memory: Vec<Bit>,
}

impl RAM {
    pub fn load(instructions: Vec<Bit>) -> Self {
        Self {
            memory: instructions,
        }
    }

    fn fetch(&self, address: usize, size: usize) -> Vec<Bit> {
        self.memory[address..size].to_vec()
    }
}

/*
* Encodings
*   ADD = 0000
*   RET = 0001
*
*   Reg0 = 0000
*   Reg1 = 0001
*   Reg2 = 0010
*   Reg3 = 0011
*
* All instructions are 16-bits long
*/
pub struct CPU {
    ram: RAM,
    program_counter: usize,
    instruction_register: Vec<Bit>,
    instruction_size: usize,
    register_file: RegisterFile,
}

impl CPU {
    pub fn new(ram: RAM, register_file: RegisterFile) -> Self {
        Self {
            ram,
            register_file,
            program_counter: 0,
            instruction_register: Vec::new(),
            instruction_size: 16,
        }
    }

    // These steps would normally be driven by the clock
    pub fn run(&mut self) -> u8 {
        self.fetch();
        let (opcode, val1, val2, output) = self.decode();
        let value = self.execute(opcode, val1, val2);
        bits_to_u8(&self.write_back(value, output))
    }

    pub fn fetch(&mut self) {
        dbg!(&self.program_counter);
        self.instruction_register = self.ram.fetch(self.program_counter, self.instruction_size);
        self.program_counter += 1;
    }

    pub fn decode(&self) -> (Bit, Vec<Bit>, Vec<Bit>, u8) {
        // This is specific to the ADD instruction
        let opcode = &self.instruction_register[0..4];
        let reg1 = bits_to_u8(&self.instruction_register[4..8]);
        let reg2 = bits_to_u8(&self.instruction_register[8..12]);
        let output = bits_to_u8(&self.instruction_register[12..16]);
        let (val1, val2) = self.register_file.read(reg1, reg2);

        // opcode is one Bit because ALU only has two instructions
        (opcode[3], val1, val2, output)
    }

    fn execute(&self, opcode: Bit, val1: Vec<Bit>, val2: Vec<Bit>) -> Vec<Bit> {
        alu(opcode, val1, val2)
    }

    fn write_back(&mut self, value: Vec<Bit>, register: u8) -> Vec<Bit> {
        self.register_file.set(register, value.clone());
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::circuits::*;

    #[test]
    fn bits_to_u8_works() {
        let bits = vec![true, false, true, false, true];
        let result = bits_to_u8(&bits);
        assert_eq!(result, 21)
    }

    fn setup_cpu() -> CPU {
        let ram = RAM::load(vec![false; 16]);
        CPU::new(ram, RegisterFile::new())
    }

    #[test]
    fn cpu_fetches_instruction() {
        let mut cpu = setup_cpu();
        cpu.fetch();
        assert_eq!(cpu.program_counter, 1);
        assert_eq!(cpu.instruction_register, cpu.ram.fetch(0, 16));
    }

    #[test]
    fn eight_bit_adder_works_with_carry_over() {
        //     1000 0000
        // +   1100 0000
        // = 1 0100 0000
        let b1 = vec![true, false, false, false, false, false, false, false];
        let b2 = vec![true, true, false, false, false, false, false, false];
        let b3 = vec![false, true, false, false, false, false, false, false];
        assert_eq!(m_bit_adder(8, b1, b2), (b3, true));
    }

    #[test]
    fn eight_bit_adder_works_scenario1() {
        //   0000 0111 ->  37
        // + 0000 0001 ->  91
        // = 0000 1000 -> 128
        let b1 = vec![false, false, false, false, false, true, true, true];
        let b2 = vec![false, false, false, false, false, false, false, true];
        let b3 = vec![false, false, false, false, true, false, false, false];
        assert_eq!(m_bit_adder(8, b1, b2), (b3, false));
    }

    #[test]
    fn eight_bit_adder_works_without_carry_over() {
        //   0010 0101 ->  37
        // + 0101 1011 ->  91
        // = 1000 0000 -> 128
        let b1 = vec![false, false, true, false, false, true, false, true];
        let b2 = vec![false, true, false, true, true, false, true, true];
        let b3 = vec![true, false, false, false, false, false, false, false];
        assert_eq!(m_bit_adder(8, b1, b2), (b3, false));
    }

    #[test]
    fn alu_equality() {
        let b1 = vec![false, false, true, true, false, false, false, false];
        let b2 = vec![false, true, false, true, false, false, false, false];
        assert_eq!(alu(false, b1, b2), vec![false; BITS]);

        let b1 = vec![false, true, true, true, false, false, false, false];
        let b2 = vec![false, true, true, true, false, false, false, false];
        assert_eq!(
            alu(false, b1, b2),
            vec![false, false, false, false, false, false, false, true]
        );
    }

    #[test]
    fn alu_addition() {
        let b1 = vec![false, false, true, true, false, false, false, false];
        let b2 = vec![false, true, false, true, false, false, false, false];
        assert_eq!(
            alu(true, b1, b2),
            vec![true, false, false, false, false, false, false, false]
        );
    }

    #[test]
    fn m_bit_equals_works() {
        let b1 = vec![true, false, true, true];
        let b2 = vec![true, true, false, true];
        assert!(!m_bit_equals(b1, b2));

        let b1 = vec![true, false, true, true];
        let b2 = vec![true, false, true, true];
        assert!(m_bit_equals(b1, b2));
    }

    #[test]
    fn m_bit_two_way_mux_works() {
        let b1 = vec![true, false, true, true];
        let b2 = vec![true, true, false, true];
        assert_eq!(
            m_bit_two_way_mux(b1, b2, true),
            vec![true, false, true, true]
        );
    }

    #[test]
    fn two_way_mux_truth_table() {
        assert!(!two_way_mux(false, false, false));
        assert!(two_way_mux(false, true, false));
        assert!(!two_way_mux(true, false, false));
        assert!(two_way_mux(true, true, false));
        assert!(!two_way_mux(false, false, true));
        assert!(!two_way_mux(false, true, true));
        assert!(two_way_mux(true, false, true));
        assert!(two_way_mux(true, true, true));
    }

    #[test]
    fn four_bit_adder_works_with_carry_over() {
        let b1 = vec![true, false, true, true];
        let b2 = vec![false, true, true, false];
        let b3 = vec![false, false, false, true];
        assert_eq!(m_bit_adder(4, b1, b2), (b3, true));
    }

    #[test]
    fn four_bit_adder_works_without_carry_over() {
        let b1 = vec![false, false, true, true];
        let b2 = vec![false, true, true, false];
        let b3 = vec![true, false, false, true];
        assert_eq!(m_bit_adder(4, b1, b2), (b3, false));
    }

    #[test]
    fn one_bit_adder_truth_table() {
        assert_eq!(one_bit_adder(false, false, false), (false, false));
        assert_eq!(one_bit_adder(false, false, true), (true, false));
        assert_eq!(one_bit_adder(true, false, false), (true, false));
        assert_eq!(one_bit_adder(true, false, true), (false, true));
        assert_eq!(one_bit_adder(false, true, false), (true, false));
        assert_eq!(one_bit_adder(false, true, true), (false, true));
        assert_eq!(one_bit_adder(true, true, false), (false, true));
        assert_eq!(one_bit_adder(true, true, true), (true, true));
    }

    #[test]
    fn equals_truth_table() {
        assert!(equals(false, false));
        assert!(!equals(true, false));
        assert!(!equals(false, true));
        assert!(equals(true, true));
    }
}
