#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BitOp {
    Set,
    Check,
    Remove,
    Toggle,
}

fn bit_mask<const N: usize>(a: [u8; N], b: [u8; N], operation: BitOp) -> [u8; N] {
    let mut flag = [0; 4];

    for i in (0..N).rev() {
        match operation {
            BitOp::Set => {flag[i] = set_flag(a[i], b[i])},
            BitOp::Check => {flag[i] = check_flag(a[i], b[i])},
            BitOp::Remove => {flag[i] = remove_flag(a[i], b[i])},
            BitOp::Toggle => {flag[i] = toggle_flag(a[i], b[i])},
        }
    }

    flag
}

fn set_flag(a: u8, b: u8) -> u8 {
    a | (b & 1)
}

fn check_flag(a: u8, b: u8) -> u8 {
    a & b
}

fn remove_flag(a: u8, b: u8) -> u8 {
    a & !b
}

fn toggle_flag(a: u8, b: u8) -> u8 {
    a ^ b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_flag_if_enum_bit_op_is_matches_set() {
        let a = [0, 1, 1, 0];
        let b = [1, 0, 0, 1];
        let bit_operation = BitOp::Set;
        
        let received = bit_mask(a, b, bit_operation);
        let expected = [1, 1, 1, 1];

        assert_eq!((received), (expected));
    }

    #[test]
    fn check_flag_if_enum_bit_op_matches_check() {
        let a = [1, 0, 1, 0];
        let b = [1, 1, 0, 0];
        let bit_operation = BitOp::Check;

        let received = bit_mask(a, b, bit_operation);
        let expected = [1, 0, 0, 0];

        assert_eq!((received), (expected));
    }

    #[test]
    fn remove_flag_if_enum_bit_op_matches_remove() {
        let a = [0, 1, 0, 1];
        let b = [1, 1, 1, 1];
        let bit_operation = BitOp::Remove;

        let received = bit_mask(a, b, bit_operation);
        let expected = [0, 0, 0, 0];

        assert_eq!((received), (expected));
    }

    #[test]
    fn toggle_flag_if_enum_bit_op_matches_toggle() {
        let a = [1, 0, 1, 1];
        let b = [1, 0, 0, 0];
        let bit_operation = BitOp::Toggle;

        let received = bit_mask(a, b, bit_operation);
        let expected = [0, 0, 1, 1];

        assert_eq!((received), (expected));
    }
}