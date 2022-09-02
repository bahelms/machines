pub mod circuits;
pub mod gates;

use gates::Bit;

pub fn binary_format(bits: Vec<Bit>) -> String {
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
