use cpu::circuits::{RegisterFile, CPU, RAM};

fn main() {
    // let instructions = vec!["ADD Reg1 Reg2 Reg1", "RET Reg1"];
    let instructions = vec![
        vec![
            vec![false; 4],
            vec![false, false, false, true],
            vec![false, false, true, false],
            vec![false, false, false, true],
        ],
        vec![
            vec![false, false, false, true],
            vec![false, false, false, true],
        ],
    ];
    let memory = RAM::load(instructions);
    let mut rf = RegisterFile::new();
    rf.insert(1, vec![false, false, true, true]);
    rf.insert(2, vec![false, true, false, false]);

    let mut cpu = CPU::new(memory, rf);
    let result = cpu.run();
    println!("3 + 4 = {:?}", result)
}
