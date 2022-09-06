pub mod circuits;
pub mod gates;

use gates::Bit;

pub fn binary_format(bits: &[Bit]) -> String {
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

fn bits_to_u8(bits: &[Bit]) -> u8 {
    let binary = binary_format(bits);
    u8::from_str_radix(&binary, 2).expect("Couldn't parse &[Bit] to usize")
}
