# Day 01 Trebuchet

I got stuck on part 2 for a while. I made my life difficult for nothing. I started to roll my own version of the KMP search algorithm... Then I realized Rust must have something already in its standard library 🙃

https://en.wikipedia.org/wiki/Knuth%E2%80%93Morris%E2%80%93Pratt_algorithm

I ended up using the build-in String methods `find` and `rfind` instead of rolling my own string-searching algorithm.

- Optimization 1
	- I don't need to use KMP, instead I can take advantage of the digits words lacking proper prefixes/suffixes and skip ahead where there's a mismatch. This will result in zero backtracking.
- Optimization 2
	- Given a string input, I take one pattern (digit work, e.g. "one") and look for matches, both front and back. 
	- I need to keep track of the indices where matches were found. This way I can stop searching for matches in the next pattern when the tracked index is surpassed.
- Optimization 3
	- When doing above, always check if the current character being matched is a digit.
