use std::io::{self, BufRead};

struct EnginePart {
    number: u32,
    row: i32,
    cols: Vec<usize>,
}

struct Symbol {
    // ch: char, // might be useful in part 2
    row: i32,
    col: usize,
}

fn main() {
    let mut parts: Vec<EnginePart> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    // keep track of the line number
    // visualize input as a matrix with <r> rows and <c> columns
    let mut row = -1;

    // parse all part numbers and symbols
    // while keeping track of their coordinates
    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();
        row += 1;

        let mut number = 0;
        let mut is_parsing_digits = false;
        let mut cols: Vec<usize> = Vec::new();

        for (col, ch) in line.char_indices() {
            if ch.is_digit(10) {
                // build number char by char
                is_parsing_digits = true;
                number = number * 10 + ch.to_digit(10).unwrap();
                cols.push(col);
            } else {
                // matched a symbol
                if is_parsing_digits {
                    // was building a number
                    parts.push(EnginePart {
                        number,
                        row,
                        cols: cols.clone(),
                    });
                    number = 0;
                    cols = Vec::new();
                }
                is_parsing_digits = false;
                match ch {
                    '.' => (),
                    _ => symbols.push(Symbol { row, col }),
                }
            }
        }

        // part number is at the end of the input
        if is_parsing_digits {
            parts.push(EnginePart {
                number,
                row,
                cols: cols.clone(),
            });
        }
    }

    // calculate sum of part numbers near symbols
    let mut ans = 0;

    for EnginePart {
        number: pnumber,
        row: prow,
        cols: pcols,
    } in parts
    {
        for Symbol {
            // ch: _,
            row: srow,
            col: scol,
        } in &symbols
        {
            if *srow < prow - 1 {
                // symbol is at least 2 rows above part
                continue;
            }
            if *srow > prow + 1 {
                // symbol is at least 2 rows below part
                // no remaining symbols near part
                break;
            }
            for pcol in &pcols {
                if scol - 1 <= *pcol && *pcol <= scol + 1 {
                    // symbol is adjacent to part
                    ans += pnumber;
                    break;
                }
            }
        }
    }

    println!("{}", ans);
}
