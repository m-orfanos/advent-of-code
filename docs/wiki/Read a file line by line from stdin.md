# Read a file line by line from stdin

Example usage

```shell
cargo run < <FILE-INPUT>
```

Solution

```rust
use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	for line_res in stdin.lock().lines() {
		match line_res {
			Ok(line) => {
				// do something with line
			},
			// should never 
			Err(e) => panic!("{:?}", e),
		}
	}
}
```
