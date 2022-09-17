use std::{
    convert::TryInto,
    fs::File,
    io::{Read, Write},
    time::{Duration, Instant},
};

const BUFFER_LEN: usize = 8192; //1048576;
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
) -> Option<(u128, usize)> {
    let init_index = (max_digits - n_digits) / 2;
    let mut result = None;

    for i in (0..=init_index).rev() {
        if palindrome[i] == palindrome[max_digits - i - 1] {
            n_digits = max_digits - 2 * i;
            let number = digits_to_number(&palindrome[i..], n_digits);
            if is_prime(number) {
                result = Some((number, n_digits));
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
        let result = find_prime_palindrome(&palindrome, MAX_DIGITS, 3);
        if let Some((number, size)) = result {
            assert_eq!(size, 3);
            assert_eq!(number, 383u128);
        } else {
            panic!("Prime palindrome not found.");
        }
    }

    #[test]
    fn test_find_prime_palindrome_extended() {
        const MAX_DIGITS: usize = 5;
        let palindrome: [u8; MAX_DIGITS] = [1, 3, 8, 3, 1];
        let result = find_prime_palindrome(&palindrome, MAX_DIGITS, 3);
        if let Some((number, size)) = result {
            assert_eq!(size, 5);
            assert_eq!(number, 13831u128);
        } else {
            panic!("Prime palindrome not found.");
        }
    }
}

fn register_palindrome(
    palindrome: u128,
    position: u64,
    elapsed_time: Duration,
    out_file: &mut File,
) -> std::io::Result<usize> {
    let out_msg = format!("after {elapsed_time:?} at {position}: {palindrome}\n");
    out_file.write(out_msg.as_bytes())
}

fn register_eof(
    file_index: usize,
    elapsed_time: Duration,
    out_file: &mut File,
) -> std::io::Result<usize> {
    let out_msg = format!("finished file {file_index} after {elapsed_time:?}\n");
    out_file.write(out_msg.as_bytes())
}

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let mut n_digits = INIT_N_DIGITS;

    let mut buffer = [0u8; BUFFER_LEN];
    let mut out_file = File::create("./res")?;

    for file_index in 0..=3 {
        let file_path = format!("/run/media/jaedson/048eda97-d4bd-403e-9540-ccdceaa630d9/Pi/Pi - Dec - Chudnovsky - {file_index}.ycd");
        let mut file = File::open(file_path)?;

        let mut position = file_index as u64 * 100_000_000_000 + 1; // position in 1-based
        let mut digits = [0u8; DIGITS_LEN];

        // Find file start
        {
            let mut temp = [0u8; 1];
            loop {
                file.read(&mut temp)?;
                if temp[0] == 0 {
                    break;
                }
            }
        }

        // Find all palindromes
        while file.read(&mut buffer)? == BUFFER_LEN {
            u64_to_digits(buffer, &mut digits);

            let mut i = DIGITS_SIZE;
            while i < DIGITS_LEN - n_digits {
                let k = i + n_digits - 1;
                let valid = (0..n_digits / 2).all(|j| digits[i + j] == digits[k - j]);
                if valid {
                    let min_index = i - (MAX_DIGITS - n_digits) / 2;
                    let max_index = min_index + MAX_DIGITS;
                    if let Some(result) =
                        find_prime_palindrome(&digits[min_index..max_index], MAX_DIGITS, n_digits)
                    {
                        let position = position + (i - DIGITS_SIZE) as u64;
                        register_palindrome(result.0, position, start.elapsed(), &mut out_file)?;
                        n_digits = result.1 + 2;
                    }
                }
                i += 1;
            }

            position += DIGITS_REAL_LEN;
        }
        register_eof(file_index, start.elapsed(), &mut out_file)?;
    }

    // E finalizamos com download via ureq
    // Para entao enviar para a VM do Google Cloud

    let duration = start.elapsed();

    println!("Time elapsed is {:?}", duration);
    Ok(())
}
