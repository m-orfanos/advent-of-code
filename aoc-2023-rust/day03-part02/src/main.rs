use std::io::{self, BufRead};

fn main() {
    #[derive(Debug)]
    struct EnginePart {
        number: u32,
        row: i32,
        cols: Vec<usize>,
    }

    #[derive(Debug)]
    struct Symbol {
        // ch: char, // might be useful in part 2, it was not
        row: i32,
        col: usize,
    }

    let mut parts: Vec<EnginePart> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    // keep track of the line number
    // visualize input as a matrix with <r> rows and <c> columns
    let mut row = -1;

    // parse all part numbers and symbols
    // while keeping track of their coordinates
    for line_res in io::stdin().lock().lines() {
        row += 1;

        let mut number = 0;

        match line_res {
            Ok(line) => {
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
                            '*' => symbols.push(Symbol { row, col }),
                            _ => (),
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
            Err(e) => panic!("{:?}", e),
        }
    }

    // calculate sum of gear ratios
    let mut ans = 0;

    for Symbol {
        // ch: _,
        row: srow,
        col: scol,
    } in symbols
    {
        let mut gear_ratio = 1;
        let mut cnt = 0;

        for EnginePart {
            number: pnumber,
            row: prow,
            cols: pcols,
        } in &parts
        {
            if srow < *prow - 1 {
                // symbol is at least 2 rows above part
                // no remaining parts near symbol
                break;
            }
            if srow > *prow + 1 {
                // symbol is at least 2 rows below part
                continue;
            }
            for pcol in pcols {
                if scol - 1 <= *pcol && *pcol <= scol + 1 {
                    // part is adjacent to symbol
                    cnt += 1;
                    gear_ratio *= pnumber;
                    break;
                }
            }
        }

        if cnt > 1 {
            ans += gear_ratio;
        }
    }

    println!("{}", ans);
}
