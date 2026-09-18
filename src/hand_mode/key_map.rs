#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Left,
    Right,
}

pub fn hand_of_char(c: char) -> Option<Hand> {
    match c.to_ascii_lowercase() {
        'q' | 'w' | 'e' | 'r' | 't' | 'a' | 's' | 'd' | 'f' | 'g' | 'z' | 'x' | 'c' | 'v'
        | 'b' => Some(Hand::Left),
        'y' | 'u' | 'ı' | 'i' | 'o' | 'p' | 'ğ' | 'ü' | 'h' | 'j' | 'k' | 'l' | 'ş' | 'n'
        | 'm' | 'ö' | 'ç' => Some(Hand::Right),
        _ => None,
    }
}

pub fn word_is_single_hand(word: &str, hand: Hand) -> bool {
    word.chars().all(|c| match hand_of_char(c) {
        Some(h) => h == hand,
        None => true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sol_el_tuslari() {
        assert_eq!(hand_of_char('q'), Some(Hand::Left));
        assert_eq!(hand_of_char('w'), Some(Hand::Left));
        assert_eq!(hand_of_char('e'), Some(Hand::Left));
        assert_eq!(hand_of_char('r'), Some(Hand::Left));
        assert_eq!(hand_of_char('t'), Some(Hand::Left));
        assert_eq!(hand_of_char('a'), Some(Hand::Left));
        assert_eq!(hand_of_char('s'), Some(Hand::Left));
        assert_eq!(hand_of_char('d'), Some(Hand::Left));
        assert_eq!(hand_of_char('f'), Some(Hand::Left));
        assert_eq!(hand_of_char('g'), Some(Hand::Left));
        assert_eq!(hand_of_char('z'), Some(Hand::Left));
        assert_eq!(hand_of_char('x'), Some(Hand::Left));
        assert_eq!(hand_of_char('c'), Some(Hand::Left));
        assert_eq!(hand_of_char('v'), Some(Hand::Left));
        assert_eq!(hand_of_char('b'), Some(Hand::Left));
    }

    #[test]
    fn test_sag_el_tuslari() {
        assert_eq!(hand_of_char('y'), Some(Hand::Right));
        assert_eq!(hand_of_char('u'), Some(Hand::Right));
        assert_eq!(hand_of_char('ı'), Some(Hand::Right));
        assert_eq!(hand_of_char('i'), Some(Hand::Right));
        assert_eq!(hand_of_char('o'), Some(Hand::Right));
        assert_eq!(hand_of_char('p'), Some(Hand::Right));
        assert_eq!(hand_of_char('ğ'), Some(Hand::Right));
        assert_eq!(hand_of_char('ü'), Some(Hand::Right));
        assert_eq!(hand_of_char('h'), Some(Hand::Right));
        assert_eq!(hand_of_char('j'), Some(Hand::Right));
        assert_eq!(hand_of_char('k'), Some(Hand::Right));
        assert_eq!(hand_of_char('l'), Some(Hand::Right));
        assert_eq!(hand_of_char('ş'), Some(Hand::Right));
        assert_eq!(hand_of_char('n'), Some(Hand::Right));
        assert_eq!(hand_of_char('m'), Some(Hand::Right));
        assert_eq!(hand_of_char('ö'), Some(Hand::Right));
        assert_eq!(hand_of_char('ç'), Some(Hand::Right));
    }

    #[test]
    fn test_bosluk_ve_rakamlar() {
        assert_eq!(hand_of_char(' '), None);
        assert_eq!(hand_of_char('1'), None);
        assert_eq!(hand_of_char('.'), None);
    }

    #[test]
    fn test_kelime_sadece_sol_el() {
        assert!(word_is_single_hand("adas", Hand::Left));
        assert!(word_is_single_hand("sad", Hand::Left));
        assert!(word_is_single_hand("ew", Hand::Left));
    }

    #[test]
    fn test_kelime_sadece_sag_el() {
        assert!(word_is_single_hand("iş", Hand::Right));
        assert!(word_is_single_hand("şık", Hand::Right));
    }

    #[test]
    fn test_kelime_karisik() {
        assert!(!word_is_single_hand("elma", Hand::Left));
        assert!(!word_is_single_hand("elma", Hand::Right));
    }
}
