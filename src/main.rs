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

const BUFFER_LEN: usize = 65536; //1048576 8192 65536
const NUMBER_LEN: usize = BUFFER_LEN / 8;
const DIGITS_SIZE: usize = 19;
const DIGITS_REAL_LEN: u64 = (NUMBER_LEN * DIGITS_SIZE) as u64;
const DIGITS_LEN: usize = NUMBER_LEN * DIGITS_SIZE + MAX_DIGITS;
const MAX_DIGITS: usize = 37; // 128 bits
const N_DIGITS: usize = 27;

const MEIO: usize = N_DIGITS / 2;
const EXPOENTE: usize = 4 * MEIO;
const FULL: u64 = (1u64 << EXPOENTE) - 1;
const PADDING: usize = (MAX_DIGITS - N_DIGITS) / 2;
const DESL_ESQ: usize = 4 * (MEIO - 1);

fn u64_to_digits(raw: [u8; BUFFER_LEN], digits: &mut [u64; DIGITS_LEN]) {
    for i in 0..MAX_DIGITS {
        digits[i] = digits[DIGITS_LEN + i - MAX_DIGITS];
    }

    for i in 0..NUMBER_LEN {
        let bytes: [u8; 8] = raw[i * 8..i * 8 + 8].try_into().unwrap();
        let mut r0 = u64::from_le_bytes(bytes);
        let index = i * DIGITS_SIZE + MAX_DIGITS;

        for i in (0..19).rev() {
            digits[index + i] = r0 % 10;
            r0 /= 10;
        }
    }
}

fn prime_check(num: u128) -> bool {
    let number_string = num.to_string();
    let is_prime = is_prime::is_prime(&number_string);
    is_prime
}

fn digits_to_number(digits: &[u64], n_digits: usize) -> u128 {
    let mut result: u128 = 0;
    for i in 0..n_digits {
        result *= 10;
        result += digits[i] as u128;
    }
    return result;
}

fn find_prime_palindrome(
    palindrome: &[u64],
    position: u64,
) -> Option<Palindrome> {
    let mut result = None;

    for i in (0..=PADDING).rev() {
        if palindrome[i] == palindrome[MAX_DIGITS - i - 1] {
            let n_digits = MAX_DIGITS - 2 * i;
            let number = digits_to_number(&palindrome[i..], n_digits);
            if prime_check(number) {
                result = Some(Palindrome {
                    number,
                    n_digits,
                    position: position + i as u64,
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
    fn test_prime_check() {
        let digits = [7u64,3,3,1,5,3,0,5,5,8,3,2,1,2,3,8,5,5,0,3,5,1,3,3,7];
        let number = digits_to_number(&digits, digits.len());
        let is_prime = prime_check(number);
        assert_eq!(is_prime, true);
    }
}

fn register_palindrome(Palindrome { n_digits, number, position }: Palindrome) {
    println!("at {position}: {number} ({n_digits})");
}

fn register_eof(file_index: usize, elapsed_time: Duration) {
    println!("finished file {file_index} after {elapsed_time:?}");
}

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    let mut buffer = [0u8; BUFFER_LEN];

    for file_index in 368..=1000 {
        // let file_path =
        //     format!("/run/media/jaedson/048eda97-d4bd-403e-9540-ccdceaa630d9/Pi/Pi - Dec - Chudnovsky - {file_index}.ycd");
        // let mut reader = File::open(file_path).expect("Fail while opening file.");

        let file_path = format!("http://storage.googleapis.com/pi100t/Pi - Dec - Chudnovsky/Pi - Dec - Chudnovsky - {file_index}.ycd");
        let file_path_str = file_path.as_str();
        let resp = ureq::get(file_path_str).call().unwrap();
        let mut reader = resp.into_reader();

        let mut block_position = file_index as u64 * 100_000_000_000 + 1; // position in 1-based
        let mut digits = [0u64; DIGITS_LEN];

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

        // melhorado 62%, agora eh 249
        // aparentemente qualquer mudança piora o resultado e esse ja eh o limite

        let mut esquerda = 0u64;
        let mut direita = 0u64;

        // Find all palindromes
        while let Ok(()) = reader.read_exact(&mut buffer) {
            u64_to_digits(buffer, &mut digits);

            let a_slice = &digits[PADDING + MEIO - 1..];
            let b_slice = &digits[PADDING + N_DIGITS - 1..];

            for i in 0..DIGITS_LEN - MAX_DIGITS {    
                esquerda = (esquerda >> 4) | (a_slice[i] << DESL_ESQ);
                direita = ((direita << 4) & FULL) | b_slice[i];
                if esquerda == direita {
                    let palindrome = &digits[i..];
                    let position = block_position + i as u64 - MAX_DIGITS as u64;
                    let palindrome = find_prime_palindrome(palindrome, position);
                    if let Some(new_p) = palindrome {
                        register_palindrome(new_p);
                    }
                }
            }

            block_position += DIGITS_REAL_LEN;
        }

        register_eof(file_index, start.elapsed());
    }

    let duration = start.elapsed();

    println!("Time elapsed is {:?}", duration);
    Ok(())
}
