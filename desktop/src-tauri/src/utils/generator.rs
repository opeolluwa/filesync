use random_str as random;

pub fn generate_random_digits() -> u16 {
    let min = 1000;
    let max = 9999;
    let random_digit = random::get_int(min, max);
    random_digit as u16
}

pub fn generate_passkey() -> String {
    let lowercase = true;
    let uppercase = true;
    let length = 8;
    let numbers = true;
    let symbols = true;

    random::get_string(length, lowercase, uppercase, numbers, symbols)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_random_digit_generator() {
        let digit = generate_random_digits();
        assert!(digit.to_string().len() >= 4);
    }

    #[test]
    fn test_random_letters_generator() {
        let passkey = generate_passkey();
        assert_eq!(passkey.len(), 8);
    }
}
