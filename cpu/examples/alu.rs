use cpu::circuits::alu;

fn main() {
    let a = vec![true, false, true, false, false, false, true, true];
    let b = vec![true, false, true, false, false, false, true, true];
    let is_equal = alu(false, a, b);
    println!("10100011 == 10100011 -> {:?}", is_equal.last().unwrap());

    let a = vec![true, false, false, false, true, true, true, true];
    let b = vec![true, false, true, false, true, true, true, true];
    let is_equal = alu(false, a, b);
    println!("10001111 == 10101111 -> {:?}", is_equal.last().unwrap());

    let a = vec![false, false, false, false, false, false, false, true];
    let b = vec![false, false, false, false, false, true, true, true];
    let add = alu(true, a, b);
    println!("00000001 + 00000111 -> {:?}", cpu::binary_format(add));
}
