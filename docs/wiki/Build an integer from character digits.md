# Build an integer from character digits

Assume the following,

You are given multiple variables, `a`, `b`, ..., `z` which denote digits. The type of the variables is `char` and the goal is build the integer the integer `ab...x`

## Examples

two characters

```text
given,
	a = 3
	b = 5

answer,
	ab = 35
```

three characters

```text
given,
	a = 3
	b = 5
	c = 1

answer,
	abc = 351
```

etc.

## Solution

Assume a string of character digits,

```rust
fn build_integer(cs: &String) -> u32 {
	let mut n = 0;
	for c in cs.chars() {
		n = n * 10 + c.to_digit(10).unwrap();
	}
	n
}

// build_integer("1234")
// 1234
```

> [!note]
> The above is a contrived example to showcase the approach, normally you would simply call `c.parse::<u32>()`.

How does this work?

```text
input
	cs = "1234"

loop
	iteration 1 | n =   0 * 10 + 1 =    1
	iteration 2 | n =   1 * 10 + 2 =   12
	iteration 3 | n =  12 * 10 + 3 =  123
	iteration 4 | n = 123 * 10 + 4 = 1234

output
	1234
```
