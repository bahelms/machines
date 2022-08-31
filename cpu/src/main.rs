mod circuits;
mod gates;

fn binary_format(bits: Vec<gates::Bit>) -> String {
    bits.iter()
        .map(|&bit| {
            if bit {
                "1".to_string()
            } else {
                "0".to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn main() {
    let a = vec![true, false, true, false];
    let b = vec![true, false, true, false];
    let is_equal = circuits::four_bit_alu(false, a, b);
    println!("1010 == 1010 -> {:?}", is_equal.last().unwrap());
    let a = vec![true, false, false, false];
    let b = vec![true, false, true, false];
    let is_equal = circuits::four_bit_alu(false, a, b);
    println!("1000 == 1010 -> {:?}", is_equal.last().unwrap());

    let a = vec![false, false, false, true];
    let b = vec![false, true, true, true];
    let add = circuits::four_bit_alu(true, a, b);
    println!("0001 + 0111 -> {:?}", binary_format(add));
}
