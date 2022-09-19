#![feature(array_windows)]

use std::{
    convert::TryInto,
    fs::File,
    io::Read,
    time::{Duration, Instant},
};

struct Palindrome {
    number: u128,
    position: u64,
    n_digits: usize,
}

const BUFFER_LEN: usize = 1048576; //8192;
const NUMBER_LEN: usize = BUFFER_LEN / 8;
const DIGITS_SIZE: usize = 19;
const DIGITS_REAL_LEN: u64 = (NUMBER_LEN * DIGITS_SIZE) as u64;
const DIGITS_LEN: usize = (NUMBER_LEN + 1) * DIGITS_SIZE;
const MAX_DIGITS: usize = 37; // 128 bits
const INIT_N_DIGITS: usize = 19;

fn u64_to_digits(raw: [u8; BUFFER_LEN], digits: &mut [u8; DIGITS_LEN]) {
    for i in 0..DIGITS_SIZE {
        digits[i] = digits[DIGITS_LEN + i - DIGITS_SIZE];
    }

    for i in 0..NUMBER_LEN {
        let bytes: [u8; 8] = raw[i * 8..i * 8 + 8].try_into().unwrap();
        let mut r0 = u64::from_le_bytes(bytes);
        let index = (i + 1) * DIGITS_SIZE;

        for i in (0..19).rev() {
            digits[index + i] = (r0 % 10) as u8;
            r0 /= 10;
        }
    }
}

fn is_prime(num: u128) -> bool {
    if num <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn digits_to_number(digits: &[u8], n_digits: usize) -> u128 {
    let mut result: u128 = 0;
    for i in 0..n_digits {
        result *= 10;
        result += digits[i] as u128;
    }
    return result;
}

fn find_prime_palindrome(
    palindrome: &[u8],
    max_digits: usize,
    mut n_digits: usize,
    block_position: u64,
    inner_position: usize,
) -> Option<Palindrome> {
    let init_index = (max_digits - n_digits) / 2;
    let mut result = None;

    for i in (0..=init_index).rev() {
        if palindrome[i] == palindrome[max_digits - i - 1] {
            n_digits = max_digits - 2 * i;
            let number = digits_to_number(&palindrome[i..], n_digits);
            if is_prime(number) {
                let position = block_position + (inner_position - DIGITS_SIZE) as u64;
                result = Some(Palindrome {
                    number,
                    n_digits,
                    position,
                });
            }
        } else {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits_to_number() {
        let digits = [1, 3, 5, 6, 2];
        let number = digits_to_number(&digits, 4);
        assert_eq!(number, 1356u128);
    }

    #[test]
    fn test_find_prime_palindrome() {
        const MAX_DIGITS: usize = 5;
        let palindrome: [u8; MAX_DIGITS] = [0, 3, 8, 3, 0];
        let result = find_prime_palindrome(&palindrome, MAX_DIGITS, 3, 0, 0);
        if let Some(palindrome) = result {
            assert_eq!(palindrome.n_digits, 3);
            assert_eq!(palindrome.number, 383u128);
        } else {
            panic!("Prime palindrome not found.");
        }
    }

    #[test]
    fn test_find_prime_palindrome_extended() {
        const MAX_DIGITS: usize = 5;
        let palindrome: [u8; MAX_DIGITS] = [1, 3, 8, 3, 1];
        let result = find_prime_palindrome(&palindrome, MAX_DIGITS, 3, 0, 0);
        if let Some(palindrome) = result {
            assert_eq!(palindrome.n_digits, 5);
            assert_eq!(palindrome.number, 13831u128);
        } else {
            panic!("Prime palindrome not found.");
        }
    }
}

fn register_palindrome(palindrome: u128, position: u64, elapsed_time: Duration) {
    println!("after {elapsed_time:?} at {position}: {palindrome}");
}

fn register_eof(file_index: usize, elapsed_time: Duration) {
    println!("finished file {file_index} after {elapsed_time:?}");
}

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let mut n_digits = INIT_N_DIGITS;

    let mut buffer = [0u8; BUFFER_LEN];

    for file_index in 0..=1000 {
        let file_path =
            format!("/home/jaedson/Documentos/Pi - Dec - Chudnovsky - {file_index}.ycd");
        let mut reader = File::open(file_path).expect("Fail while opening file.");

        // let file_path = format!("http://storage.googleapis.com/pi100t/Pi - Dec - Chudnovsky/Pi - Dec - Chudnovsky - {file_index}.ycd");
        // let file_path_str = file_path.as_str();
        // let resp = ureq::get(file_path_str).call().unwrap();
        // let mut reader = resp.into_reader();

        let mut block_position = file_index as u64 * 100_000_000_000 + 1; // position in 1-based
        let mut digits = [0u8; DIGITS_LEN];

        // Find file start
        {
            let mut temp = [0u8; 1];
            loop {
                reader
                    .read(&mut temp)
                    .expect("Fail while reading initial bytes.");
                if temp[0] == 0 {
                    break;
                }
            }
        }

        // melhorado 15%, agora eh 560s

        // Find all palindromes
        while let Ok(()) = reader.read_exact(&mut buffer) {
            u64_to_digits(buffer, &mut digits);
            let max_range = n_digits / 2;
            let padding = (MAX_DIGITS - n_digits) / 2;
            let new_n_digits = digits
                .array_windows::<MAX_DIGITS>()
                .enumerate()
                .filter(|(_, v)| {
                    (0..max_range).all(|i| v[padding + i] == v[padding + n_digits - 1 - i])
                })
                .map(|(inner_position, palindrome)| find_prime_palindrome(palindrome, MAX_DIGITS, n_digits, block_position, inner_position))
                .filter(|v| v.is_some())
                .map(|v| v.unwrap())
                .reduce(|accur, item| {
                    if item.n_digits > accur.n_digits {
                        item
                    } else {
                        accur
                    }
                });

            if let Some(palindrome) = new_n_digits {
                register_palindrome(
                    palindrome.number,
                    palindrome.position as u64,
                    start.elapsed(),
                );
                n_digits = palindrome.n_digits + 2;
            }

            block_position += DIGITS_REAL_LEN;
        }
        register_eof(file_index, start.elapsed());
    }

    let duration = start.elapsed();

    println!("Time elapsed is {:?}", duration);
    Ok(())
}
