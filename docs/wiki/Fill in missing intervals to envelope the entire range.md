# Fill in missing intervals to envelope the entire range

```python
a = range(100)
xs = [
	 range(11,20),
	 range(41,50),
	 range(91,100)
]

ans = []
curr = a.start
for x in xs: # must be sorted
	start = x.start
	end = x.stop
	ans.append(range(curr, start-1))
	curr = end + 1

# still missing `range(90, 100)`
print(ans)
# [
#   range(1,10),
#   range(21,40),
#   range(51,90)
# ]
```

Another approach, (but I haven't tested it yet)

If you have values instead `(x1, y1), (x2, y2), ...`, where these denote the start/end of intervals and the intervals might be overlapping, you can

- add all the endpoints in a list and sort it
- every pair `(a[0], a[1]), (a[1], a[2]), ...` denotes a continuous range

you might need to

- extend on the LHS/RHS to include -inf/+inf
- remove "empty" ranges, using the example above `y1=x2` would produce `(x2,y1)` which is an empty inverval
