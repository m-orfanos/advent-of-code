use std::io::{self, BufRead};

fn main() {
    struct WordDigit {
        word: &'static str,
        digit: u32,
    }

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

    #[derive(Debug)]
    struct Calibration {
        idx: usize,
        digit: u32,
    }

    let mut ans = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let haystack = line.unwrap();

        // parse the words
        let ws: Vec<Calibration> = word_digits
            .iter()
            .flat_map(|wd| {
                // return first and last, if applicable
                vec![
                    haystack.find(wd.word).map(|idx| Calibration {
                        idx,
                        digit: wd.digit,
                    }),
                    haystack.rfind(wd.word).map(|idx| Calibration {
                        idx,
                        digit: wd.digit,
                    }),
                ]
            })
            .filter(|w| w.is_some())
            .map(|w| w.unwrap())
            .collect();

        // parse the digits
        let ds: Vec<Calibration> = haystack
            .char_indices()
            .filter(|tup| tup.1.is_digit(10))
            .map(|tup| Calibration {
                idx: tup.0,
                digit: tup.1.to_digit(10).unwrap(),
            })
            .collect();

        // merge words and digits in a single vector
        let mut calibrations: Vec<&Calibration> = vec![&ws, &ds].into_iter().flatten().collect();
        calibrations.sort_by(|a, b| a.idx.cmp(&b.idx));

        // find first and last digits
        let n = calibrations.first().unwrap();
        let m = calibrations.last().unwrap();

        // build the number
        ans += n.digit * 10 + m.digit;
    }

    println!("{}", ans);
}
