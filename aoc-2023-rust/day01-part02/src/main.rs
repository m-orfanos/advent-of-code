use std::io::{self, BufRead};

struct WordDigit {
    word: &'static str,
    digit: i32,
}

fn main() {
    let word_digits = [
        WordDigit {
            word: "zero",
            digit: 0,
        },
        WordDigit {
            word: "one",
            digit: 1,
        },
        WordDigit {
            word: "two",
            digit: 2,
        },
        WordDigit {
            word: "three",
            digit: 3,
        },
        WordDigit {
            word: "four",
            digit: 4,
        },
        WordDigit {
            word: "five",
            digit: 5,
        },
        WordDigit {
            word: "six",
            digit: 6,
        },
        WordDigit {
            word: "seven",
            digit: 7,
        },
        WordDigit {
            word: "eight",
            digit: 8,
        },
        WordDigit {
            word: "nine",
            digit: 9,
        },
    ];

    let stdin = io::stdin();
    let mut n = 0;
    for line in stdin.lock().lines() {
        let haystack = line.as_ref().unwrap();
        let (_, digit_start) = find_forward(&word_digits, haystack);
        let (_, digit_end) = find_backward(&word_digits, haystack);
        n = n + digit_start * 10 + digit_end;
    }
    println!("{}", n);
}

fn find_forward(word_digits: &[WordDigit], haystack: &String) -> (usize, i32) {
    let mut idx = usize::MAX;
    let mut n = -1;

    // search for digits as words (zero, one, two, ..., nine)
    for word_digit in word_digits {
        let needle = word_digit.word;
        match haystack.find(needle) {
            Some(current_idx) => {
                if current_idx < idx {
                    // update only when a better match is found
                    idx = current_idx;
                    n = word_digit.digit;
                }
            }
            None => (),
        }
    }

    // iterate over haystack searching for digits (0, 1, 2, ..., 9)
    for (current_idx, current_ch) in haystack.char_indices() {
        if current_idx > idx {
            break;
        }
        if current_ch.is_digit(10) {
            // already found a better match
            idx = current_idx;
            n = current_ch.to_digit(10).unwrap() as i32;
        }
    }

    (idx, n)
}

fn find_backward(word_digits: &[WordDigit], haystack: &String) -> (usize, i32) {
    let mut idx = usize::MIN;
    let mut n = -1;

    // search for digits as words (zero, one, two, ..., nine) wrt the end
    for word_digit in word_digits {
        let needle = word_digit.word;
        match haystack.rfind(needle) {
            Some(match_idx) => {
                if match_idx > idx {
                    // update only when a better match is found
                    idx = match_idx;
                    n = word_digit.digit;
                }
            }
            None => (),
        }
    }

    // iterate over haystack in reverse searching for digits (0, 1, 2, ..., 9)
    for (current_idx, current_ch) in haystack.char_indices().rev() {
        if current_idx < idx {
            // already found a better match
            break;
        }
        if current_ch.is_digit(10) {
            idx = current_idx;
            n = current_ch.to_digit(10).unwrap() as i32;
        }
    }

    (idx, n)
}
