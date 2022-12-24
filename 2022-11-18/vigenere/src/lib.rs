use std::vec;

/// Encrypt a ciphertext with a given key
pub fn vigenere_encrypt(plaintext: String, key: String) -> String {
    let mut ciphertext_nums: Vec<u32> = vec![];
    let key_nums = to_vec(key);
    for (i, plaintext_char) in to_vec(plaintext).into_iter().enumerate() {
        let offset = key_nums[i % key_nums.len()];
        ciphertext_nums.push((plaintext_char + offset) % 26)
    }
    to_string(ciphertext_nums)
}

/// Decrypt a ciphertext with a given key
pub fn vigenere_decrypt(ciphertext: String, key: String) -> String {
    let mut plaintext_nums: Vec<u32> = vec![];
    let key_nums = to_vec(key);
    for (i, ciphertext_char) in to_vec(ciphertext).into_iter().enumerate() {
        let offset = key_nums[i % key_nums.len()];
        if offset > ciphertext_char {
            plaintext_nums.push(ciphertext_char + 26 - offset)
        } else {
            plaintext_nums.push(ciphertext_char - offset)
        }
    }
    to_string(plaintext_nums)
}

fn alphabet_index(c: char) -> u32 {
    assert!(c as u32 >= 0x61 && c as u32 <= 0x7a);
    c as u32 - 0x61
}

fn to_char(i: u32) -> char {
    assert!(i <= 25);
    char::from_u32(i + 0x61).unwrap()
}

fn to_vec(s: String) -> Vec<u32> {
    let mut res: Vec<u32> = vec![];
    for c in s.chars() {
        res.push(alphabet_index(c));
    }
    res
}

fn to_string(v: Vec<u32>) -> String {
    let mut s: String = "".to_string();
    for num in v {
        s.push(to_char(num))
    }
    s
}

#[test]
fn test_alphabet_index() {
    assert_eq!(alphabet_index('a'), 0);
    assert_eq!(alphabet_index('b'), 1);
    assert_eq!(alphabet_index('y'), 24);
    assert_eq!(alphabet_index('z'), 25);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_alphabet_index_panics() {
    alphabet_index('-');
}

#[test]
fn test_to_char() {
    assert_eq!(to_char(0), 'a');
    assert_eq!(to_char(1), 'b');
    assert_eq!(to_char(24), 'y');
    assert_eq!(to_char(25), 'z');
}

#[test]
fn test_to_vec() {
    assert_eq!(to_vec("abcxyz".to_string()), [0, 1, 2, 23, 24, 25])
}

#[test]
fn test_to_string() {
    assert_eq!(to_string(vec![0, 1, 2, 23, 24, 25]), "abcxyz".to_string())
}

#[test]
fn test_encrypt() {
    assert_eq!(
        vigenere_encrypt("attackatdawn".to_string(), "lemon".to_string()),
        "lxfopvefrnhr".to_string()
    )
}

#[test]
fn test_decrypt() {
    assert_eq!(
        vigenere_decrypt("lxfopvefrnhr".to_string(), "lemon".to_string()),
        "attackatdawn".to_string()
    )
}
