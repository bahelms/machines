use crate::gates::*;

// logic
fn equals(a: Bit, b: Bit) -> Bit {
    or_gate(and_gate(not_gate(a), not_gate(b)), and_gate(a, b))
}

// arithmetic
fn one_bit_adder(a: Bit, b: Bit, carry_in: Bit) -> (Bit, Bit) {
    let sum1 = xor_gate(a, b);
    let carry_over1 = and_gate(a, b);

    let sum2 = xor_gate(sum1, carry_in);
    let carry_over2 = and_gate(sum1, carry_in);

    let carry_out = or_gate(carry_over1, carry_over2);
    (sum2, carry_out)
}

fn four_bit_adder(a: [Bit; 4], b: [Bit; 4]) -> ([Bit; 4], Bit) {
    let mut sum = [false; 4];
    let mut carry_over = false;
    let matched_bits: Vec<(&Bit, &Bit)> = a.iter().zip(b.iter()).collect();

    for idx in (0..4).rev() {
        let (&a_bit, &b_bit) = matched_bits[idx];
        let result = one_bit_adder(a_bit, b_bit, carry_over);
        sum[idx] = result.0;
        carry_over = result.1;
    }

    (sum, carry_over)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_bit_adder_works_with_carry_over() {
        let b1 = [true, false, true, true];
        let b2 = [false, true, true, false];
        let b3 = [false, false, false, true];
        assert_eq!(four_bit_adder(b1, b2), (b3, true));
    }

    #[test]
    fn four_bit_adder_works_without_carry_over() {
        let b1 = [false, false, true, true];
        let b2 = [false, true, true, false];
        let b3 = [true, false, false, true];
        assert_eq!(four_bit_adder(b1, b2), (b3, false));
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
