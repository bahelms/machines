pub type Bit = bool;

pub fn and_gate(a: Bit, b: Bit) -> Bit {
    a & b
}

pub fn or_gate(a: Bit, b: Bit) -> Bit {
    a | b
}

pub fn not_gate(a: Bit) -> Bit {
    !a
}

pub fn xor_gate(a: Bit, b: Bit) -> Bit {
    or_gate(and_gate(a, not_gate(b)), and_gate(not_gate(a), b))
}

fn nand_gate(a: Bit, b: Bit) -> Bit {
    not_gate(and_gate(a, b))
}

fn m_bit_gate(gate: impl Fn(Bit, Bit) -> Bit, a: Vec<Bit>, b: Vec<Bit>) -> Vec<Bit> {
    let matched_bits: Vec<(&Bit, &Bit)> = a.iter().zip(b.iter()).collect();
    let mut bytes = Vec::with_capacity(matched_bits.capacity());
    for (&a_bit, &b_bit) in matched_bits {
        bytes.push(gate(a_bit, b_bit));
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m_bit_and_gate_with_mismatched_bytes() {
        let b1 = vec![true, false, true, true, true];
        let b2 = vec![false, true, true, false];
        assert_eq!(
            m_bit_gate(and_gate, b1, b2),
            vec![false, false, true, false]
        );
    }

    #[test]
    fn m_bit_and_gate_works() {
        let b1 = vec![true, false, true, true];
        let b2 = vec![false, true, true, false];
        assert_eq!(
            m_bit_gate(and_gate, b1, b2),
            vec![false, false, true, false]
        );
    }

    #[test]
    fn and_gate_truth_table() {
        assert!(!and_gate(false, false));
        assert!(!and_gate(true, false));
        assert!(!and_gate(false, true));
        assert!(and_gate(true, true));
    }

    #[test]
    fn or_gate_truth_table() {
        assert!(!or_gate(false, false));
        assert!(or_gate(true, false));
        assert!(or_gate(false, true));
        assert!(or_gate(true, true));
    }

    #[test]
    fn not_gate_truth_table() {
        assert!(not_gate(false));
        assert!(!not_gate(true));
    }

    #[test]
    fn xor_gate_truth_table() {
        assert!(!xor_gate(false, false));
        assert!(xor_gate(true, false));
        assert!(xor_gate(false, true));
        assert!(!xor_gate(true, true));
    }

    #[test]
    fn nand_gate_truth_table() {
        assert!(nand_gate(false, false));
        assert!(nand_gate(false, true));
        assert!(nand_gate(true, false));
        assert!(!nand_gate(true, true));
    }
}
