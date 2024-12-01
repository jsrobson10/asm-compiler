
zero = 0xf0
one = 0xf1
counter = 0xf2
display_start = 0xf3
display_ptr = .
a = 0xf4
b = 0xf5
c = 0xf6

	copy_v 0, zero
	copy_v 1, one
	copy_v 0xc0, display_start
	copy_v 1, b
	copy_v 16, counter
scan:
	math.add a, b, c
	math.add b, zero, a
	math.add c, zero, b
	math.sub counter, one, counter
	math.add counter, display_start, display_ptr
	math_s.add a, zero, display_ptr
	jump_if.nequal counter, zero, scan


