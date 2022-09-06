use cpu::circuits::{RegisterFile, CPU, RAM};

fn main() {
    /*
    Instructions

    "ADD Reg1 Reg2 Reg1" - 0000 0001 0010 0001
    false, false, false, true,
    false, false, false, true,
    false, false, true, false,
    false, false, false, true,

    "RET Reg1" - 0001 0001 0000 0000
    false, false, false, true,
    false, false, false, true,
    false, false, false, false,
    false, false, false, false,
    */
    let instructions = vec![
        false, false, false, true, false, false, false, true, false, false, true, false, false,
        false, false, true, false, false, false, true, false, false, false, true, false, false,
        false, false, false, false, false, false,
    ];
    let memory = RAM::load(instructions);

    // 8-bit registers
    let mut rf = RegisterFile::new();
    rf.set(
        1,
        vec![false, false, false, false, false, false, true, true],
    );
    rf.set(
        2,
        vec![false, false, false, false, false, true, false, false],
    );

    let mut cpu = CPU::new(memory, rf);
    let result = cpu.run();
    println!("3 + 4 = {:?}", result)
}
