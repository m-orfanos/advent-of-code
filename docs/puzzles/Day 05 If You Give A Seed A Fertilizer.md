# Day 05 If You Give A Seed A Fertilizer

Part 1

I attempted to create a map for each source->destination attribute and traverse through them as needed. Looking at the real input, this was a really bad approach as the ranges are pretty big.

I will have to think of another approach.

Instead of generating all the possible combinations, I worked with ranges/intervals instead. The checks simplified to validating the search query fell within the range `start < x < end`.

Part 2

Wow, this was really difficult. It took me about seven (?) hours to write something that worked with the example input (but immediately passed with the real input 💃).

I kept trying to split the interval and add missing parts on the fly but I hit a wall with that approach.

The two main breakthroughs were:

1) determine a fast method to calculate the intersection of two intervals
2) expand the mappings such that they cover a continuous range (e.g. no holes)

Both methods are briefly summarized in the following docs:

- [[Find the intersection of two intervals]]
- [[Fill in missing intervals to envelope the entire range]]
