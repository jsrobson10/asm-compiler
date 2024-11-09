
counter = .
display_ptr = .
a = .
b = .
c = .

	copy_v 1, b
	copy_v 16, counter
scan:
	math_pp.add a, b, c
	copy_p b, a
	copy_p c, b
	math_pv.sub counter, 1, counter
	math_pv.add counter, 0xf0, display_ptr
	deref_ps a, display_ptr
	jump_nz counter, scan


