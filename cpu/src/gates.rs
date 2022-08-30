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

#[cfg(test)]
mod tests {
    use super::*;

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
}
