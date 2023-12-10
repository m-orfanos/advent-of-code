# Read a file line by line from stdin

Example usage

```shell
cargo run < <FILE-INPUT>
```

Solution 1

```rust
use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	for line_res in stdin.lock().lines() {
		let line = line_res.unwrap();
		// do something with line
	}
}
```

Solution 2

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
