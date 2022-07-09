pub fn num_to_char(num: usize) -> char {
    match num {
        0..=9 => ('0' as u8 + num as u8) as char,
        10..=35 => ('a' as u8 + num as u8 - 10) as char,
        36..=61 => ('A' as u8 + num as u8 - 36) as char,
        _ => char::REPLACEMENT_CHARACTER,
    }
}

pub fn char_to_num(c: char) -> Option<usize> {
    const ZERO: usize = '0' as usize;
    const NINE: usize = '9' as usize;
    const L_A: usize = 'a' as usize;
    const L_Z: usize = 'z' as usize;
    const U_A: usize = 'A' as usize;
    const U_Z: usize = 'Z' as usize;

    let c = c as usize;
    match c {
        ZERO..=NINE => Some(c - ZERO),
        L_A..=L_Z => Some(c - L_A + 10),
        U_A..=U_Z => Some(c - U_A + 36),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_number_test() {
        for x in 0..=61 {
            assert_eq!(Some(x), char_to_num(num_to_char(x)))
        }
        assert_eq!(char::REPLACEMENT_CHARACTER, num_to_char(1084));
        assert_eq!(None, char_to_num(char::REPLACEMENT_CHARACTER));
        assert_eq!(None, char_to_num('?'));
    }
}
